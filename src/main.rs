use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};
use portkill::logic::*;
use portkill::platform::real_lsof;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut dry_run = false;
    let mut interactive = false;
    let mut port: Option<u16> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--dry-run" | "-d" => dry_run = true,
            "--interactive" | "-i" => interactive = true,
            "--version" | "-v" => {
                println!("portkill {VERSION}");
                return;
            }
            "--help" | "-h" => {
println!(
r#"
portkill v{VERSION}

A CLI utility to terminate processes listening on a TCP port.

USAGE:
    portkill <port> [options]

OPTIONS:
    -h, --help          Show this help message
    -v, --version       Show version information
    -d, --dry-run       Show what would be killed without killing it
    -i, --interactive   Prompt before killing a process

EXAMPLES:
    portkill 3000
    portkill --dry-run 3000
    portkill -i 3000
"#
);
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

    fn format_description(process_name: &str, pid: &str, port: &str) -> String {
        format!("{process_name} (pid {pid}) on port {port}")
    }

    if pids.is_empty() {
        if dry_run {
            println!("[dry-run] no processes would be killed on port {port}");
        } else {
            println!("no processes found on port {port}");
        }
        return;
    }

    for pid in pids {
        let process_name = find_process_name(&pid, real_lsof)
            .unwrap_or_else(|| "unknown".to_string());

        let description = format_description(&process_name, &pid, &port.to_string());

        if dry_run {
            println!("[dry-run] would kill {description}");
            continue;
        }

        if interactive {
            print!("kill {description}? [y/N] ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("[error] input not read; skipping {description}");
                continue;
            }

            if input.trim().to_lowercase() != "y" {
                println!("[skip] {description}");
                continue;
            }
        }

        match Command::new("kill").arg(&pid).status() {
            Ok(status) if status.success() => {
                println!("[killed] {description}");
            }
            Ok(_) => {
                eprintln!("[error] failed to kill {description}");
            }
            Err(_) => {
                eprintln!("[error] could not execute kill for {description}");
            }
        }
    }
}
