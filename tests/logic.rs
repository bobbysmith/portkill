use portkill::logic::*;

#[test]
fn find_pids_parses_output() {
    let pids = find_pids(3000, |_| Some("123\n456\n".into()));
    assert_eq!(pids, vec!["123", "456"]);
}

#[test]
fn find_process_name_extracts_name() {
    let name = find_process_name("123", |_| Some("cnode\n".into()));
    assert_eq!(name, Some("node".into()));
}

#[test]
fn dry_run_plans_would_kill() {
    let actions = plan_actions(
        vec!["123".into()],
        true,
        |_| Some("node".into()),
    );

    assert_eq!(
        actions,
        vec![Action::WouldKill {
            pid: "123".into(),
            name: "node".into()
        }]
    );
}

#[test]
fn normal_run_plans_kill() {
    let actions = plan_actions(
        vec!["456".into()],
        false,
        |_| Some("python".into()),
    );

    assert_eq!(
        actions,
        vec![Action::Kill {
            pid: "456".into(),
            name: "python".into()
        }]
    );
}
