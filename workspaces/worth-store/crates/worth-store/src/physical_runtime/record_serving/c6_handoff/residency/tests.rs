#[test]
fn c6_handoff_carries_no_local_scheduler_or_pending_registry() {
    let handoff = include_str!("mod.rs");
    let local_scheduler = ["C6", "LocalScheduler"].concat();
    let pending_registry = ["Pending", "WorkRegistry"].concat();
    assert!(
        !handoff.contains(&local_scheduler) && !handoff.contains(&pending_registry),
        "C5_PREDICATE:c6-local-scheduler C.6 acquired a local scheduler or pending registry"
    );
}
