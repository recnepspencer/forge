use std::sync::Arc;

use crate::data::telemetry::InvalidationPerformedCounter;
use crate::facade::{
    DependencyEdge, SignalGraph, SignalObservationCompletion, SignalObservationRequest,
    SignalObservationSession, SignalRuntime, SignalRuntimePolicy,
};
use crate::tests::support::ASPECT_A;

#[test]
fn direct_graph_reconstitution_rebinds_the_installed_observation_plan() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let bytes = serde_json::to_vec(&graph).unwrap();
    let mut restored: SignalGraph = serde_json::from_slice(&bytes).unwrap();
    let source = restored.node().build();
    let consumer = restored.node().build();
    restored
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    let bootstrap = restored
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    restored
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    crate::facade::mark_dirty(&mut restored, consumer, ASPECT_A).unwrap();
    let session = restored
        .begin_observation_session(SignalObservationRequest::counters().with_performed_work())
        .unwrap();
    let plan = restored
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    restored
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();
    assert_eq!(
        restored
            .invalidation_performed_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated),
        1,
        "an explicitly selected counter surface must remain connected after serde reconstitution"
    );
    assert_eq!(restored.invalidation_performed_work().len(), 1);
    let receipt = restored.finish_observation_session(&session).unwrap();
    assert!(receipt.retains_executed_target(restored.runtime_instance_id(), consumer));
}

#[test]
fn continuous_serde_reconstitution_keeps_default_capture_active_without_a_session() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    graph.set_runtime_policy(
        SignalRuntimePolicy::operational().with_observation_activation(
            worth_foundational::ObservationActivationProfile::Continuous,
        ),
    );

    let bytes = serde_json::to_vec(&graph).unwrap();
    let mut restored: SignalGraph = serde_json::from_slice(&bytes).unwrap();
    let plan = restored
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    restored
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();

    assert_eq!(
        restored
            .invalidation_performed_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated),
        1,
        "continuous defaults must remain active after serde reconstitution"
    );
    assert_eq!(restored.invalidation_performed_work().len(), 1);
}

#[test]
fn operational_serde_reconstitution_keeps_on_demand_capture_idle_without_a_session() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    crate::facade::mark_dirty(&mut graph, consumer, ASPECT_A).unwrap();

    let bytes = serde_json::to_vec(&graph).unwrap();
    let mut restored: SignalGraph = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        restored.runtime_policy().observation_activation(),
        worth_foundational::ObservationActivationProfile::OnDemand
    );
    let plan = restored
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    restored
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();

    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| restored.invalidation_performed_counters().value(counter) == 0));
    assert!(restored.invalidation_performed_work().is_empty());
    assert!(restored.observe().lineage_records().is_empty());
    assert!(restored.observe().explanation_fact(consumer).is_none());
    assert!(restored.observe().provenance_fact(consumer).is_none());
    assert!(restored
        .observe()
        .latest_frontier_execution_summary()
        .is_none());
    assert!(restored.observe().replay_events().is_empty());
    assert!(restored.observe().latest_flow_diagnostics().is_none());
}

#[test]
fn failed_public_observation_cancels_capture_before_returning_error() {
    let mut runtime = crate::facade::SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let result = runtime.observe_execution(SignalObservationRequest::counters(), |_| {
        Err::<(), _>(crate::data::error::SignalError::invalid_input(
            "execution failed",
        ))
    });
    assert!(result.is_err());
    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| runtime
            .graph()
            .invalidation_performed_counters()
            .value(counter)
            == 0));
}

#[test]
fn stale_and_duplicate_session_tokens_cannot_transition_lifecycle() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let session = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    let stale = SignalObservationSession {
        graph_instance: session.graph_instance,
        generation: session.generation.saturating_add(1),
        request: session.request,
        liveness: Arc::clone(&session.liveness),
        drop_cleanup: Arc::clone(&session.drop_cleanup),
    };
    let duplicate = SignalObservationSession {
        graph_instance: session.graph_instance,
        generation: session.generation,
        request: session.request,
        liveness: Arc::clone(&session.liveness),
        drop_cleanup: Arc::clone(&session.drop_cleanup),
    };

    assert!(graph.cancel_observation_session(&stale).is_err());
    assert_eq!(
        graph.cancel_observation_session(&session).unwrap(),
        SignalObservationCompletion::Cancelled
    );
    assert!(graph.cancel_observation_session(&duplicate).is_err());
    assert_eq!(
        graph.last_observation_completion(),
        Some(SignalObservationCompletion::Cancelled)
    );
}

#[test]
fn completed_session_rejects_duplicate_stale_and_foreign_finish_without_state_change() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = runtime.graph_mut().node().build();
    let target = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .set_dependencies(target, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    let bootstrap = runtime
        .graph_mut()
        .build_evaluation_plan(
            &[target],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    runtime
        .graph_mut()
        .execute_prepared_plan(&bootstrap, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    crate::facade::mark_dirty(runtime.graph_mut(), target, ASPECT_A).unwrap();
    let session = runtime
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    let duplicate = SignalObservationSession {
        graph_instance: session.graph_instance,
        generation: session.generation,
        request: session.request,
        liveness: Arc::clone(&session.liveness),
        drop_cleanup: Arc::clone(&session.drop_cleanup),
    };
    let stale = SignalObservationSession {
        graph_instance: session.graph_instance,
        generation: session.generation.saturating_add(1),
        request: session.request,
        liveness: Arc::clone(&session.liveness),
        drop_cleanup: Arc::clone(&session.drop_cleanup),
    };
    let foreign = SignalObservationSession {
        graph_instance: session.graph_instance,
        generation: session.generation,
        request: session.request,
        liveness: Arc::clone(&session.liveness),
        drop_cleanup: Arc::clone(&session.drop_cleanup),
    };

    let plan = runtime
        .graph_mut()
        .build_evaluation_plan(
            &[target],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    runtime
        .graph_mut()
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(2, 0)))
        })
        .unwrap();
    let receipt = runtime.finish_observation_session(&session).unwrap();
    assert_eq!(receipt.completion(), SignalObservationCompletion::Completed);
    assert!(
        receipt
            .realized_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated)
            > 0
    );
    assert_eq!(
        runtime.graph().last_observation_completion(),
        Some(SignalObservationCompletion::Completed)
    );

    assert!(runtime.finish_observation_session(&duplicate).is_err());
    assert!(runtime.finish_observation_session(&stale).is_err());
    let mut other_graph = SignalGraph::new();
    other_graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let other_session = other_graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    assert!(other_graph.finish_observation_session(&foreign).is_err());
    assert!(other_graph
        .cancel_observation_session(&other_session)
        .is_ok());
    assert_eq!(
        runtime.graph().last_observation_completion(),
        Some(SignalObservationCompletion::Completed),
        "denied finish attempts must not mutate the completed session"
    );
}
