use crate::facade::{
    Aspect, AspectVersion, ClockAdvanceRequest, ClockDomain, ClockTick, EvaluationRequestMode,
    LoweredTemporalEligibility, NodeEvaluationResult, NodeState, SignalError, SignalGraph,
    SignalRuntime, TemporalEligibilityAuthority,
};
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn runtime_execute_prepared_plan_uses_clock_basis_for_at_or_after_without_temporal_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().at_or_after(5).build();
    let plan = runtime
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();
    let calls = AtomicU32::new(0);
    let aspect = Aspect::new(0);

    let before = runtime
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(aspect, 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_executed, 0);
    assert_eq!(
        runtime.graph().get_state(node).unwrap(),
        NodeState::MaybeStale
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();

    let after = runtime
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(aspect, 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    assert_eq!(runtime.graph().get_state(node).unwrap(), NodeState::Clean);
    assert_eq!(
        runtime
            .graph()
            .node_aspect_version(node)
            .unwrap()
            .get(aspect),
        1
    );
}

#[test]
fn runtime_target_execution_uses_clock_basis_for_at_or_after_without_temporal_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().at_or_after(7).build();
    let calls = AtomicU32::new(0);
    let aspect = Aspect::new(1);

    let before = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_executed, 0);
    assert_eq!(
        runtime.graph().get_state(node).unwrap(),
        NodeState::MaybeStale
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap();

    let after = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    assert_eq!(runtime.graph().get_state(node).unwrap(), NodeState::Clean);
    assert_eq!(
        runtime
            .graph()
            .node_aspect_version(node)
            .unwrap()
            .get(aspect),
        3
    );
}

#[test]
fn node_owned_after_declaration_schedules_defers_admits_and_consumes_one_wake() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(5).unwrap().build();
    let aspect = Aspect::new(2);
    let calls = AtomicU32::new(0);

    let before = runtime
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

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_deferred_by_condition, 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    let deferred = before.stages[0].task_records[0]
        .temporal_eligibility
        .as_ref()
        .expect("deferred node-owned temporal condition should carry proof");
    assert_eq!(
        deferred.authority(),
        TemporalEligibilityAuthority::RuntimeScheduledWake
    );
    assert_eq!(before.temporal_summary.deferred_count(), 1);
    assert_eq!(before.temporal_summary.runtime_scheduled_wake_count(), 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let after = runtime
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

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    let ready = after.stages[0].task_records[0]
        .temporal_eligibility
        .as_ref()
        .expect("ready node-owned temporal condition should carry proof");
    assert_eq!(
        ready.authority(),
        TemporalEligibilityAuthority::RuntimeScheduledWake
    );
    assert!(ready.ready_by_time());
    assert!(matches!(
        ready,
        LoweredTemporalEligibility::Ready(ready) if ready.wake_id().is_some()
    ));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn sealed_temporal_policy_family_uses_node_owned_runtime_wakes_without_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let debounce = runtime.graph_mut().node().debounce(3).unwrap().build();
    let throttle = runtime.graph_mut().node().throttle(3).unwrap().build();
    let stale_after = runtime.graph_mut().node().stale_after(3).unwrap().build();
    let interval = runtime.graph_mut().node().interval(3).unwrap().build();
    let nodes = [debounce, throttle, stale_after, interval];
    let calls = AtomicU32::new(0);

    let before = runtime
        .targets(nodes)
        .run(&(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(3), 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_deferred_by_condition, 4);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 4);
    assert_eq!(before.temporal_summary.deferred_count(), 4);
    assert_eq!(before.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(before.temporal_summary.runtime_scheduled_wake_count(), 4);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .unwrap();
    let after = runtime
        .targets(nodes)
        .run(&(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(3), 2)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 4);
    assert_eq!(after.tasks_executed, 4);
    assert_eq!(after.temporal_summary.ready_count(), 4);
    assert_eq!(after.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(after.temporal_summary.runtime_scheduled_wake_count(), 4);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 4);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(6))
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .interval_wake_regeneration_count,
        1
    );
}
