use crate::facade::{
    Aspect, AspectVersion, ClockAdvanceRequest, ClockDomain, ClockTick, EvaluationRequestMode,
    NodeEvaluationResult, SignalError, SignalGraph, SignalRuntime,
};
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn throttle_burst_reuses_original_window_without_reschedule() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();
    let calls = AtomicU32::new(0);

    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
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
                    AspectVersion::from_updates([(Aspect::new(0), 2)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let admitted = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(admitted.tasks_executed, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn throttle_admission_summary_records_reuse_without_window_drift() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(first.scheduled_count(), 1);
    assert_eq!(first.scheduled()[0].due_tick(), ClockTick::new(5));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let second = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(second.scheduled_count(), 0);
    assert_eq!(second.rescheduled_count(), 0);
    assert_eq!(second.reused_count(), 1);
    assert_eq!(second.reused()[0].original_due_tick(), ClockTick::new(5));
    assert_eq!(second.reused()[0].attempted_due_tick(), ClockTick::new(7));
    assert_eq!(second.reused()[0].decision_tick(), ClockTick::new(2));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let third = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(third.scheduled_count(), 0);
    assert_eq!(third.rescheduled_count(), 0);
    assert_eq!(third.reused_count(), 1);
    assert_eq!(third.reused()[0].original_due_tick(), ClockTick::new(5));
    assert_eq!(third.reused()[0].attempted_due_tick(), ClockTick::new(9));
    assert_eq!(third.reused()[0].decision_tick(), ClockTick::new(4));

    let frontier = runtime.temporal_frontier_snapshot();
    assert_eq!(frontier.next_due_tick(), Some(ClockTick::new(5)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(runtime.telemetry().temporal.wake_reuse_count, 2);
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 0);
}
