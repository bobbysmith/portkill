use std::process::Command;

pub fn real_lsof(args: &[&str]) -> Option<String> {
    let output = Command::new("lsof").args(args).output().ok()?;
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}
