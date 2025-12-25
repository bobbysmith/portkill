use clap::Parser;
use std::io::{self, Write};
use std::process::Command;
use portkill::logic::*;
use portkill::platform::real_lsof;

/// CLI utility to terminate processes listening on a TCP port
#[derive(Parser, Debug)]
#[clap(
    name = "portkill",
    version = env!("CARGO_PKG_VERSION"),
    author = "Bobby Smith",
    about = "CLI utility to terminate processes listening on a TCP port",
)]
struct Cli {
    port: u16,

    #[clap(short, long, help = "Show what would be killed without killing it")]
    dry_run: bool,

    #[clap(short, long, help = "Ask for confirmation before killing a process")]
    interactive: bool,
}

fn main() {
    let args = Cli::parse();

    let pids = find_pids(args.port, real_lsof);

    fn format_description(process_name: &str, pid: &str, port: &str) -> String {
        format!("{process_name} (pid {pid}) on port {port}")
    }

    if pids.is_empty() {
        if args.dry_run {
            println!("[dry-run] no processes would be killed on port {}", args.port);
        } else {
            println!("no processes found on port {}", args.port);
        }
        return;
    }

    for pid in pids {
        let process_name = find_process_name(&pid, real_lsof)
            .unwrap_or_else(|| "unknown".to_string());

        let description = format_description(&process_name, &pid, &args.port.to_string());

        if args.dry_run {
            println!("[dry-run] would kill {description}");
            continue;
        }

        if args.interactive {
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
