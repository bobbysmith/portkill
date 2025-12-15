#[derive(Debug, PartialEq)]
pub enum Action {
    Kill { pid: String, name: String },
    WouldKill { pid: String, name: String },
}

pub fn find_pids<F>(port: u16, run: F) -> Vec<String>
where
    F: Fn(&[&str]) -> Option<String>,
{
    run(&["-ti", &format!(":{port}")])
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn find_process_name<F>(pid: &str, run: F) -> Option<String>
where
    F: Fn(&[&str]) -> Option<String>,
{
    run(&["-p", pid, "-F", "c"])
        .and_then(|out| {
            out.lines()
                .find(|l| l.starts_with('c'))
                .map(|l| l.trim_start_matches('c').to_string())
        })
        .filter(|s| !s.is_empty())
}

pub fn plan_actions<F>(
    pids: Vec<String>,
    dry_run: bool,
    lookup: F,
) -> Vec<Action>
where
    F: Fn(&str) -> Option<String>,
{
    pids
        .into_iter()
        .map(|pid| {
            let name = lookup(&pid).unwrap_or_else(|| "unknown".to_string());
            if dry_run {
                Action::WouldKill { pid, name }
            } else {
                Action::Kill { pid, name }
            }
        })
        .collect()
}
