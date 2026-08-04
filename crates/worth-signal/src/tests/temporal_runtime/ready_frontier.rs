use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime, TemporalCondition,
    TemporalWakeRetirementReason,
};

#[test]
fn due_temporal_wake_batch_promotion_is_canonical_by_due_tick_then_ordinal() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let later = runtime
        .schedule_temporal_wake(TemporalCondition::after(9).unwrap(), ClockTick::new(9))
        .unwrap();
    let first_due = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    let second_due_same_tick = runtime
        .schedule_temporal_wake(TemporalCondition::throttle(3).unwrap(), ClockTick::new(3))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(9),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    let promoted_ids = promoted.iter().map(|wake| wake.id()).collect::<Vec<_>>();
    assert_eq!(
        promoted_ids,
        vec![first_due.id(), second_due_same_tick.id(), later.id()]
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_eligibility_lowering_count,
        3
    );
    assert_eq!(
        runtime
            .temporal_frontier_snapshot()
            .scheduled_frontier_width(),
        0
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(first_due.id())
    );
}

#[test]
fn due_temporal_wake_batch_promotion_leaves_future_frontier_entries_scheduled() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let ready_now = runtime
        .schedule_temporal_wake(TemporalCondition::after(2).unwrap(), ClockTick::new(2))
        .unwrap();
    let future = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), ready_now.id());
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(future.id())
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );
}

#[test]
fn retiring_ready_wake_updates_frontier_indexes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let first = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    let second = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted[0].id(), first.id());
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(first.id())
    );

    runtime
        .retire_temporal_wake(first.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();

    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(second.id())
    );
}

#[test]
fn rescheduling_scheduled_wake_supersedes_old_wake_and_updates_frontier() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    let reschedule = runtime
        .reschedule_temporal_wake(wake.id(), ClockTick::new(9))
        .unwrap();

    assert_eq!(
        reschedule.retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert_eq!(reschedule.retired().id(), wake.id());
    assert_eq!(reschedule.scheduled().due_tick(), ClockTick::new(9));
    assert_ne!(reschedule.scheduled().id(), wake.id());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(9))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 1);
}
