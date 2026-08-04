use crate::facade::{
    ClockAdvanceOrdinal, ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime,
    TemporalCondition,
};

#[test]
fn clock_advance_rejects_metadata_only_domains() {
    let runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let err = runtime
        .validate_clock_advance(ClockAdvanceRequest::new(
            ClockDomain::WallClock,
            ClockTick::new(10),
        ))
        .unwrap_err();

    assert!(
        format!("{err}").contains("metadata-only"),
        "wall-clock advances must be rejected as non-authoritative"
    );
}

#[test]
fn clock_advance_rejects_monotonic_regression() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(8),
        ))
        .unwrap();

    let err = runtime
        .validate_clock_advance(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap_err();

    assert!(
        format!("{err}").contains("clock regression"),
        "authoritative monotonic time must never move backward"
    );
}

#[test]
fn clock_advance_updates_basis_and_ordinal() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let validated = runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(12),
        ))
        .unwrap();

    assert_eq!(validated.previous_tick(), ClockTick::ZERO);
    assert_eq!(validated.next_tick(), ClockTick::new(12));
    assert_eq!(validated.ordinal(), ClockAdvanceOrdinal::new(1));

    let basis = runtime.clock_basis();
    assert_eq!(basis.domain(), ClockDomain::MonotonicExecution);
    assert_eq!(basis.current_tick(), ClockTick::new(12));
    assert_eq!(basis.last_advance_ordinal(), ClockAdvanceOrdinal::new(1));
}

#[test]
fn clock_advance_summary_is_cost_honest_and_does_not_promote_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    let summary = runtime
        .advance_clock_with_summary(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();

    assert_eq!(summary.validated_advance().previous_tick(), ClockTick::ZERO);
    assert_eq!(summary.validated_advance().next_tick(), ClockTick::new(5));
    assert_eq!(summary.promoted_wake_count(), 0);
    assert!(
        summary.ready_selection_deferred(),
        "clock advance must not hide ready promotion behind the clock input surface"
    );
    assert_eq!(
        summary.frontier_before().next_due_wake_id(),
        Some(wake.id())
    );
    assert_eq!(summary.frontier_after().next_due_wake_id(), Some(wake.id()));
    assert_eq!(summary.frontier_after().scheduled_frontier_width(), 1);
    assert_eq!(summary.frontier_after().ready_frontier_width(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_broad_scan_denial_count,
        0,
        "clock advance should not claim or perform ready-frontier selection work"
    );
}
