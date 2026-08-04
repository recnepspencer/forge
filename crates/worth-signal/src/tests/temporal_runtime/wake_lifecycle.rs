use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime, TemporalCondition,
    TemporalWakeId, TemporalWakeRetirementReason, WakeOrdinal,
};

#[test]
fn scheduling_temporal_wake_assigns_monotonic_identity_and_updates_summary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let first = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();
    let second = runtime
        .schedule_temporal_wake(
            TemporalCondition::stale_after(9).unwrap(),
            ClockTick::new(9),
        )
        .unwrap();

    assert_eq!(first.id(), TemporalWakeId::new(0));
    assert_eq!(second.id(), TemporalWakeId::new(1));
    assert_eq!(first.ordinal(), WakeOrdinal::new(1));
    assert_eq!(second.ordinal(), WakeOrdinal::new(2));

    let summary = runtime.temporal_wake_summary();
    assert_eq!(summary.scheduled_count(), 2);
    assert_eq!(summary.ready_count(), 0);
    assert_eq!(summary.retired_count(), 0);
    assert_eq!(summary.next_wake_id(), TemporalWakeId::new(2));
    assert_eq!(summary.next_wake_ordinal(), WakeOrdinal::new(2));
    assert_eq!(runtime.telemetry().temporal.temporal_wake_count, 2);
    assert_eq!(runtime.telemetry().temporal.scheduled_frontier_width, 2);
    assert_eq!(runtime.telemetry().temporal.wake_allocation_count, 2);
}

#[test]
fn promoting_temporal_wake_to_ready_requires_due_tick() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::throttle(4).unwrap(), ClockTick::new(4))
        .unwrap();

    let err = runtime.promote_temporal_wake_ready(wake.id()).unwrap_err();
    assert!(
        format!("{err}").contains("before due tick"),
        "promotion should deny readiness before the scheduled due tick arrives"
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();

    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    assert_eq!(ready.id(), wake.id());
    assert_eq!(ready.scheduled_ordinal(), wake.ordinal());
    assert_eq!(ready.ready_ordinal(), WakeOrdinal::new(2));
    assert_eq!(ready.ready_tick(), ClockTick::new(4));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 1);
    assert_eq!(
        runtime
            .temporal_frontier_snapshot()
            .scheduled_frontier_width(),
        0
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().ready_frontier_width(),
        1
    );
    assert_eq!(runtime.telemetry().temporal.ready_queue_width, 1);
}

#[test]
fn retiring_ready_temporal_wake_records_reason_and_updates_summary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();

    let retired = runtime
        .retire_temporal_wake(ready.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();

    assert_eq!(retired.id(), ready.id());
    assert_eq!(retired.active_ordinal(), ready.ready_ordinal());
    assert_eq!(retired.retired_ordinal(), WakeOrdinal::new(3));
    assert_eq!(retired.retired_tick(), ClockTick::new(3));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Consumed);

    let summary = runtime.temporal_wake_summary();
    assert_eq!(summary.scheduled_count(), 0);
    assert_eq!(summary.ready_count(), 0);
    assert_eq!(summary.retired_count(), 1);
    assert_eq!(runtime.telemetry().temporal.retired_wake_count, 1);
}

#[test]
fn retiring_unknown_temporal_wake_is_rejected() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let err = runtime
        .retire_temporal_wake(
            TemporalWakeId::new(99),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap_err();

    assert!(
        format!("{err}").contains("unknown temporal wake 99"),
        "runtime should reject retirement for wake ids that were never admitted"
    );
}
