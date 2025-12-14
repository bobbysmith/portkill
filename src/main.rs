use std::env;
use std::process::{Command, exit};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("--version") | Some("-v") => {
            println!("portkill {VERSION}");
            return;
        }
        Some("--help") | Some("-h") => {
            eprintln!("usage: portkill <port>");
            return;
        }
        _ => {}
    }

    let port: u16 = match arg.and_then(|p| p.parse().ok()) {
        Some(p) => p,
        None => {
            eprintln!("usage: portkill <port>");
            exit(1);
        }
    };

    let output = match Command::new("lsof")
        .args(["-ti", &format!(":{port}")])
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            eprintln!("portkill: failed to run lsof");
            exit(1);
        }
    };

    let pids: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    if pids.is_empty() {
        println!("nothing running on port {port}");
        return;
    }

    for pid in &pids {
        let process_name = Command::new("lsof")
            .args(["-p", pid, "-F", "c"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| {
                s.lines()
                    .find(|line| line.starts_with('c'))
                    .map(|line| line.trim_start_matches('c').to_string())
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unknown".to_string());

        match Command::new("kill").arg(pid).status() {
            Ok(status) if status.success() => {
                println!("killed port {port} ({process_name} at pid {pid})");
            }
            Ok(_) => {
                println!("found {process_name} on port {port} (pid {pid}) but could not kill it");
            }
            Err(_) => {
                println!("error killing {process_name} on port {port} (pid {pid})");
            }
        }
    }
}
