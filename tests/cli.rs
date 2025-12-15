use assert_cmd::Command;
use predicates::str::contains;

fn portkill_cmd() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("portkill")
}

#[test]
fn no_args_shows_usage() {
    let mut cmd = portkill_cmd();
    cmd.assert()
        .failure()
        .stderr(contains("usage: portkill <port>"));
}

#[test]
fn invalid_port_fails() {
    let mut cmd = portkill_cmd();
    cmd.arg("banana");
    cmd.assert().failure();
}

#[test]
fn unused_port_prints_nothing_running() {
    let mut cmd = portkill_cmd();
    cmd.arg("54321");
    cmd.assert()
        .success()
        .stdout(contains("nothing running on port"));
}

#[test]
fn version_flag_prints_version() {
    let mut cmd = portkill_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("portkill "));
}

#[test]
fn short_version_flag_prints_version() {
    let mut cmd = portkill_cmd();
    cmd.arg("-v");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("portkill "));
}

#[test]
fn help_flag_prints_usage() {
    let mut cmd = portkill_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("portkill <port> [options]"))
        .stdout(predicates::str::contains("OPTIONS:"))
        .stdout(predicates::str::contains("--dry-run"));
}

#[test]
fn short_help_flag_prints_usage() {
    let mut cmd = portkill_cmd();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("portkill <port> [options]"))
        .stdout(predicates::str::contains("OPTIONS:"))
        .stdout(predicates::str::contains("--dry-run"));
}

