use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, NodeId, SignalGraph, SignalRuntime,
    TemporalCondition, TemporalWakeOwner, TemporalWakeRetirementReason,
};

#[test]
fn owned_temporal_wake_lifecycle_preserves_owner_across_schedule_ready_retire_and_reschedule() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(5).unwrap(),
            ClockTick::new(5),
        )
        .unwrap();
    assert_eq!(wake.owner(), TemporalWakeOwner::Node(node));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    assert_eq!(ready.owner(), TemporalWakeOwner::Node(node));

    let reschedule = runtime
        .reschedule_temporal_wake(ready.id(), ClockTick::new(8))
        .unwrap();
    assert_eq!(reschedule.retired().owner(), TemporalWakeOwner::Node(node));
    assert_eq!(
        reschedule.scheduled().owner(),
        TemporalWakeOwner::Node(node)
    );
}

#[test]
fn retiring_temporal_wakes_for_owner_is_selective_and_updates_frontiers() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner_a = runtime.graph_mut().node().build();
    let owner_b = runtime.graph_mut().node().build();

    let wake_a_due_2 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_a),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();
    let _wake_a_due_4 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_a),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let wake_b_due_3 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_b),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), wake_a_due_2.id());

    let retired = runtime
        .retire_temporal_wakes_for_owner(
            TemporalWakeOwner::Node(owner_a),
            TemporalWakeRetirementReason::Disposed,
        )
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner_a));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 2);
    assert!(retired
        .retired()
        .iter()
        .all(|wake| wake.owner() == TemporalWakeOwner::Node(owner_a)));
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(wake_b_due_3.id())
    );
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
}

#[test]
fn default_manual_temporal_wake_owner_is_explicit() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();

    assert_eq!(wake.owner(), TemporalWakeOwner::Manual);
}

#[test]
fn scheduling_owned_temporal_wake_rejects_stale_or_forged_node_owner() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();
    runtime.graph_mut().unregister_node(node).unwrap();

    let stale_err = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap_err();
    assert!(
        format!("{stale_err}").contains("non-live node owner"),
        "stale node handles must not be allowed to mint node-owned temporal wakes"
    );

    let forged_err = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(NodeId::new(999_999, 0)),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap_err();
    assert!(
        format!("{forged_err}").contains("non-live node owner"),
        "forged node handles must not be allowed to mint node-owned temporal wakes"
    );
}

#[test]
fn due_promotion_retires_disposed_node_owned_wakes_instead_of_promoting_them() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();
    runtime.graph_mut().unregister_node(owner).unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();

    assert!(promoted.is_empty());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    let retired = runtime
        .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Disposed)
        .unwrap_err();
    assert!(
        format!("{retired}").contains("unknown temporal wake"),
        "disposed due wakes should already be retired by the batch promotion path"
    );
}

#[test]
fn ready_wake_cannot_grant_previous_value_access_after_owner_unregistration() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(1).unwrap(),
            ClockTick::new(1),
        )
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    runtime.graph_mut().unregister_node(owner).unwrap();

    let err = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap_err();

    assert!(
        format!("{err}").contains("non-ready temporal wake"),
        "runtime-owned node disposal should retire the ready wake before previous-value access can be re-granted"
    );
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}
