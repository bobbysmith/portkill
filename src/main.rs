use std::env;
use std::process::Command;

fn main() {
    let port = match env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("usage: portkill <port>");
            return;
        }
    };

    let output = match Command::new("lsof")
        .args(["-ti", &format!(":{port}")])
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            println!("portkill: failed to run lsof");
            return;
        }
    };

    let pids: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    if pids.is_empty() {
        println!("nothing running on port {}", port);
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
                println!(
                    "killed port {} ({} at pid {})",
                    port, process_name, pid
                );
            }
            Ok(_) => {
                println!(
                    "found {} on port {} (pid {}) but could not kill it",
                    process_name, port, pid
                );
            }
            Err(_) => {
                println!(
                    "error killing {} on port {} (pid {})",
                    process_name, port, pid
                );
            }
        }
    }
}
