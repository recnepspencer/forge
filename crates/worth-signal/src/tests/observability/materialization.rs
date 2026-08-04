use crate::facade::{
    ArtifactRetentionPolicy, DiagnosticsAvailability, EvaluationRequestMode, NodeEvaluationResult,
    NodeId, SignalGraph, SignalRuntimePolicy,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn artifact_materialization_availability_states_are_explicit_and_non_ambiguous() {
    let mut retained_graph = SignalGraph::new();
    let retained_source = retained_graph.node().build();
    let retained_dependent = retained_graph.node().build();
    retained_graph
        .append_dependency(retained_dependent, retained_source, ASPECT_A)
        .unwrap();
    retained_graph.set_runtime_policy(SignalRuntimePolicy::development());
    let retained_bootstrap = retained_graph
        .build_evaluation_plan(
            &[retained_source, retained_dependent],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    retained_graph
        .execute_prepared_plan_with_precompute(&retained_bootstrap, &|node, view| {
            let result = if node == retained_source {
                view.finish(version_ab(1, 0))
            } else {
                let version = view.read_aspect_version(retained_source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .unwrap();
    let (retained_explanation, retained_mode) = retained_graph
        .materialize_explanation_artifact(retained_dependent)
        .unwrap();

    let mut reconstructed_graph = SignalGraph::new();
    let reconstructed_source = reconstructed_graph.node().build();
    let reconstructed_dependent = reconstructed_graph.node().build();
    reconstructed_graph
        .append_dependency(reconstructed_dependent, reconstructed_source, ASPECT_A)
        .unwrap();
    reconstructed_graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let mut reconstructed_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(
        &mut reconstructed_graph,
        reconstructed_source,
        &mut reconstructed_compute,
    )
    .unwrap();
    evaluate(
        &mut reconstructed_graph,
        reconstructed_dependent,
        &mut reconstructed_compute,
    )
    .unwrap();
    let (reconstructed_explanation, reconstructed_mode) = reconstructed_graph
        .materialize_explanation_artifact(reconstructed_dependent)
        .unwrap();

    let mut omitted_graph = SignalGraph::new();
    let omitted_source = omitted_graph.node().build();
    let omitted_dependent = omitted_graph.node().build();
    omitted_graph
        .append_dependency(omitted_dependent, omitted_source, ASPECT_A)
        .unwrap();
    omitted_graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit),
    );
    let mut omitted_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut omitted_graph, omitted_source, &mut omitted_compute).unwrap();
    evaluate(&mut omitted_graph, omitted_dependent, &mut omitted_compute).unwrap();
    let (omitted_explanation, omitted_mode) = omitted_graph
        .materialize_explanation_artifact(omitted_dependent)
        .unwrap();

    let mut denied_graph = SignalGraph::new();
    let denied_source = denied_graph.node().build();
    let denied_dependent = denied_graph.node().build();
    denied_graph
        .append_dependency(denied_dependent, denied_source, ASPECT_A)
        .unwrap();
    let mut denied_policy = SignalRuntimePolicy::operational();
    denied_policy
        .reconstruction_budget
        .allow_explanation_reconstruction = false;
    denied_graph.set_runtime_policy(denied_policy);
    let mut denied_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut denied_graph, denied_source, &mut denied_compute).unwrap();
    evaluate(&mut denied_graph, denied_dependent, &mut denied_compute).unwrap();
    let (denied_explanation, denied_mode) = denied_graph
        .materialize_explanation_artifact(denied_dependent)
        .unwrap();

    assert!(retained_explanation.is_some());
    assert!(reconstructed_explanation.is_some());
    assert!(omitted_explanation.is_none());
    assert!(denied_explanation.is_none());

    assert_eq!(retained_mode, DiagnosticsAvailability::RetainedAvailable);
    assert_eq!(
        reconstructed_mode,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(omitted_mode, DiagnosticsAvailability::OmittedByTier);
    assert_eq!(denied_mode, DiagnosticsAvailability::DeniedByBudget);

    assert!(retained_mode.is_available());
    assert!(reconstructed_mode.is_available());
    assert!(!omitted_mode.is_available());
    assert!(!denied_mode.is_available());
    assert!(!retained_mode.is_reconstructed());
    assert!(reconstructed_mode.is_reconstructed());
    assert!(!omitted_mode.is_reconstructed());
    assert!(!denied_mode.is_reconstructed());

    assert_ne!(retained_mode.message(), reconstructed_mode.message());
    assert_ne!(retained_mode.message(), omitted_mode.message());
    assert_ne!(retained_mode.message(), denied_mode.message());
    assert_ne!(reconstructed_mode.message(), omitted_mode.message());
    assert_ne!(reconstructed_mode.message(), denied_mode.message());
    assert_ne!(omitted_mode.message(), denied_mode.message());
}
