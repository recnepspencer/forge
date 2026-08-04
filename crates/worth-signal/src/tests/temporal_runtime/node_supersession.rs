use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, EvaluationCondition, SignalGraph, SignalRuntime,
    TemporalCondition, TemporalWakeOwner, TemporalWakeRetirementReason,
};

#[test]
fn replacing_node_checkpoint_image_supersedes_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let other_owner = runtime.graph_mut().node().build();

    let scheduled = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let ready = runtime
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

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), ready.id());

    let image = runtime.graph().node_checkpoint_image(owner).unwrap();
    let retired = runtime
        .replace_node_from_checkpoint_image(owner, image)
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 2);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(surviving_other.id())
    );

    let scheduled_err = runtime
        .retire_temporal_wake(scheduled.id(), TemporalWakeRetirementReason::Cancelled)
        .unwrap_err();
    assert!(format!("{scheduled_err}").contains("unknown temporal wake"));
}

#[test]
fn rewriting_node_evaluation_config_supersedes_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let manual = runtime
        .schedule_temporal_wake(TemporalCondition::after(9).unwrap(), ClockTick::new(9))
        .unwrap();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(5).unwrap(),
            ClockTick::new(5),
        )
        .unwrap();

    let mut eval_config = runtime.graph().node_eval_config(owner).unwrap().clone();
    eval_config.condition = EvaluationCondition::Temporal(TemporalCondition::throttle(7).unwrap());
    let retired = runtime
        .replace_node_evaluation_config(owner, eval_config.clone())
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(
        runtime.graph().node_eval_config(owner).unwrap(),
        &eval_config
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(manual.id())
    );
}

#[test]
fn graph_mut_replace_node_checkpoint_image_uses_runtime_temporal_supersession_protocol() {
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

    let image = runtime.graph().node_checkpoint_image(owner).unwrap();
    let retired = runtime
        .graph_mut()
        .replace_node_from_checkpoint_image(owner, image)
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn graph_mut_replace_node_evaluation_config_uses_runtime_temporal_supersession_protocol() {
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

    let mut eval_config = runtime.graph().node_eval_config(owner).unwrap().clone();
    eval_config.condition =
        EvaluationCondition::Temporal(TemporalCondition::stale_after(6).unwrap());
    let retired = runtime
        .graph_mut()
        .replace_node_evaluation_config(owner, eval_config.clone())
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(
        runtime.graph().node_eval_config(owner).unwrap(),
        &eval_config
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}
