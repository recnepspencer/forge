use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime, TemporalCondition,
    TemporalWakeOwner, TemporalWakeRetirementReason,
};

#[test]
fn runtime_unregister_node_structurally_retires_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let other_owner = runtime.graph_mut().node().build();

    let owned_scheduled = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let owned_ready = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(1).unwrap(),
            ClockTick::new(1),
        )
        .unwrap();
    let surviving_other = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(other_owner),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap();
    let surviving_manual = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), owned_ready.id());

    let retired = runtime.unregister_node(owner).unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 2);
    assert!(retired
        .retired()
        .iter()
        .all(|wake| wake.owner() == TemporalWakeOwner::Node(owner)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(surviving_other.id())
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(3))
    );
    let manual_due = runtime
        .retire_temporal_wake(
            surviving_manual.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap();
    assert_eq!(manual_due.owner(), TemporalWakeOwner::Manual);
    let other_due = runtime
        .retire_temporal_wake(
            surviving_other.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap();
    assert_eq!(other_due.owner(), TemporalWakeOwner::Node(other_owner));
    let owned_err = runtime
        .retire_temporal_wake(
            owned_scheduled.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap_err();
    assert!(format!("{owned_err}").contains("unknown temporal wake"));
}

#[test]
fn graph_mut_unregister_node_uses_runtime_temporal_disposal_protocol() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();

    let retired = runtime.graph_mut().unregister_node(owner).unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}
