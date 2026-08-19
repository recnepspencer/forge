use crate::data::telemetry::InvalidationPerformedCounter;
use crate::facade::{
    DependencyEdge, SignalGraph, SignalObservationCompletion, SignalObservationRequest,
    SignalRuntimePolicy,
};
use crate::tests::support::ASPECT_A;

#[test]
fn cancellation_is_typed_and_allows_a_fresh_session() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    let session = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();
    assert!(
        graph
            .invalidation_performed_counters()
            .value(InvalidationPerformedCounter::TopologyRevisionRevalidations)
            > 0
    );
    let plan = graph
        .build_evaluation_plan(
            &[consumer],
            crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&plan, &(), &|context| {
            Ok(context.finish(crate::tests::support::version_ab(1, 0)))
        })
        .unwrap();
    assert!(
        graph
            .invalidation_performed_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated)
            > 0
    );
    assert_eq!(
        graph.cancel_observation_session(&session).unwrap(),
        SignalObservationCompletion::Cancelled
    );
    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| graph.invalidation_performed_counters().value(counter) == 0));
    let replacement = graph
        .begin_observation_session(SignalObservationRequest::work())
        .unwrap();
    assert!(graph.finish_observation_session(&replacement).is_err());
}

#[test]
fn dropped_session_clears_selected_capture_without_minting_receipt() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().build();
    let consumer = graph.node().build();
    {
        let _session = graph
            .begin_observation_session(SignalObservationRequest::counters())
            .unwrap();
        graph
            .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
            .unwrap();
        assert!(
            graph
                .invalidation_performed_counters()
                .value(InvalidationPerformedCounter::TopologyRevisionRevalidations)
                > 0
        );
        let plan = graph
            .build_evaluation_plan(
                &[consumer],
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
            )
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &|context| {
                Ok(context.finish(crate::tests::support::version_ab(1, 0)))
            })
            .unwrap();
        assert!(
            graph
                .invalidation_performed_counters()
                .value(InvalidationPerformedCounter::NodesEvaluated)
                > 0
        );
    }
    assert!(InvalidationPerformedCounter::ALL
        .into_iter()
        .all(|counter| graph.invalidation_performed_counters().value(counter) == 0));
    assert_eq!(
        graph.last_observation_completion(),
        Some(SignalObservationCompletion::Abandoned)
    );
    let fresh = graph
        .begin_observation_session(SignalObservationRequest::counters())
        .unwrap();
    assert!(graph.finish_observation_session(&fresh).is_err());
}
