use std::env;
use std::process::{Command, exit};
use portkill::logic::*;
use portkill::platform::real_lsof;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut dry_run = false;
    let mut port: Option<u16> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--dry-run" | "-d" => dry_run = true,
            "--version" | "-v" => {
                println!("portkill {VERSION}");
                return;
            }
            "--help" | "-h" => {
                eprintln!("usage: portkill <port>");
                return;
            }
            _ => port = arg.parse().ok(),
        }
    }

    let port = match port {
        Some(p) => p,
        None => {
            eprintln!("usage: portkill <port>");
            exit(1);
        }
    };

    let pids = find_pids(port, real_lsof);

    if pids.is_empty() {
        if dry_run {
            println!("[dry-run] nothing would be killed on port {port}");
        } else {
            println!("nothing running on port {port}");
        }
        return;
    }

    for pid in pids {
        let process_name = find_process_name(&pid, real_lsof)
            .unwrap_or_else(|| "unknown".to_string());

        if dry_run {
            println!("[dry-run] would kill port {port} ({process_name} at pid {pid})");
            continue;
        }

        match Command::new("kill").arg(&pid).status() {
            Ok(status) if status.success() => {
                println!("killed port {port} ({process_name} at pid {pid})");
            }
            Ok(_) => {
                println!("found {process_name} on port {port} (pid {pid}) but could not kill it");
            }
            Err(_) => {
                eprintln!("[error] failed to kill {process_name} on port {port} (pid {pid})");
            }
        }
    }
}