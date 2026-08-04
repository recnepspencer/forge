use crate::facade::{
    Aspect, AspectVersion, ClockAdvanceRequest, ClockDomain, ClockTick, EvaluationRequestMode,
    NodeEvaluationResult, SignalError, SignalGraph, SignalRuntime, TemporalWakeRetirementReason,
};
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn debounce_burst_supersedes_owned_wake_and_waits_for_new_quiet_period() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();
    let aspect = Aspect::new(7);
    let calls = AtomicU32::new(0);

    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 2)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(7))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    assert!(runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .is_empty());
    assert_eq!(calls.load(Ordering::Relaxed), 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap();
    let admitted = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 4)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(admitted.tasks_executed, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn debounce_admission_summary_records_each_burst_supersession_without_extra_live_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(first.scheduled_count(), 1);
    assert_eq!(first.rescheduled_count(), 0);
    assert_eq!(first.reused_count(), 0);
    assert_eq!(first.scheduled()[0].due_tick(), ClockTick::new(5));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let second = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(second.scheduled_count(), 0);
    assert_eq!(second.rescheduled_count(), 1);
    assert_eq!(second.reused_count(), 0);
    assert_eq!(
        second.rescheduled()[0].retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert_eq!(
        second.rescheduled()[0].scheduled().due_tick(),
        ClockTick::new(7)
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let third = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(third.scheduled_count(), 0);
    assert_eq!(third.rescheduled_count(), 1);
    assert_eq!(third.reused_count(), 0);
    assert_eq!(
        third.rescheduled()[0].scheduled().due_tick(),
        ClockTick::new(9)
    );
    assert_eq!(third.total_decision_count(), 1);

    let wake_summary = runtime.temporal_wake_summary();
    assert_eq!(
        wake_summary.scheduled_count(),
        1,
        "debounce burst coalescing should keep one live scheduled wake"
    );
    assert_eq!(wake_summary.retired_count(), 2);
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 2);
    assert_eq!(
        runtime.telemetry().temporal.scheduled_frontier_width,
        1,
        "rescheduling one owner must not widen the active frontier"
    );
}

#[test]
fn legacy_temporal_wake_admission_return_does_not_treat_debounce_reschedule_as_new_wake() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake(node).unwrap();
    assert!(first.is_some());

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let rescheduled = runtime.admit_node_temporal_wake(node).unwrap();

    assert!(
        rescheduled.is_none(),
        "single-wake admission convenience reports fresh schedules; summaries carry reschedule evidence"
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(7))
    );
}
