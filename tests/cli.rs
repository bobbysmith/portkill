use assert_cmd::cargo;
use assert_cmd::Command;
use predicates::prelude::*;
use std::net::TcpListener;
use std::process::{Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn portkill_cmd() -> Command {
    cargo::cargo_bin_cmd!("portkill")
}

#[test]
fn no_args_shows_usage() {
    portkill_cmd()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage: portkill <PORT>"));
}

#[test]
fn invalid_port_fails() {
    portkill_cmd().arg("banana").assert().failure();
}

#[test]
fn unused_port_prints_nothing_running() {
    portkill_cmd()
        .arg("54321")
        .assert()
        .success()
        .stdout(predicate::str::contains("no processes found on port"));
}

fn find_available_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn spawn_dummy_process_on_port(port: u16) -> Child {
    let child = std::process::Command::new("nc")
        .args(["-l", &port.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("nc is required for interactive tests");

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(2) {
        if TcpListener::bind(("127.0.0.1", port)).is_err() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

    child
}

#[test]
fn interactive_mode_confirms_yes_kills_process() {
    let port = find_available_port();
    let mut child = spawn_dummy_process_on_port(port);
    let pid = child.id();

    portkill_cmd()
        .arg("-i")
        .arg(port.to_string())
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("(pid {pid}) on port {port}")))
        .stdout(predicate::str::contains("[killed]"));

    thread::sleep(Duration::from_millis(50));
    assert!(child.try_wait().unwrap().is_some());
}

#[test]
fn interactive_mode_confirms_no_skips_process() {
    let port = find_available_port();
    let mut child = spawn_dummy_process_on_port(port);

    portkill_cmd()
        .arg("-i")
        .arg(port.to_string())
        .write_stdin("n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("[skip]"));

    assert!(child.try_wait().unwrap().is_none());
    let _ = child.kill();
}

#[test]
fn interactive_mode_default_enter_skips_process() {
    let port = find_available_port();
    let mut child = spawn_dummy_process_on_port(port);

    portkill_cmd()
        .arg("-i")
        .arg(port.to_string())
        .write_stdin("\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("[skip]"));

    assert!(child.try_wait().unwrap().is_none());
    let _ = child.kill();
}
