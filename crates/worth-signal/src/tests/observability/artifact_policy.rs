use crate::facade::{
    ArtifactRetentionPolicy, ChangedRegion, DiagnosticsAvailability, EvaluationRequestMode,
    NodeEvaluationResult, NodeId, PartitionSubscription, SignalGraph, SignalRuntimePolicy,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn explicit_omit_policy_surfaces_unavailable_artifacts() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    let (explanation, explanation_mode) =
        graph.materialize_explanation_artifact(dependent).unwrap();
    let (provenance, provenance_mode) = graph.materialize_provenance_artifact(dependent).unwrap();

    assert!(explanation.is_none());
    assert!(provenance.is_none());
    assert_eq!(explanation_mode, DiagnosticsAvailability::OmittedByTier);
    assert_eq!(provenance_mode, DiagnosticsAvailability::OmittedByTier);
}

#[test]
fn explicit_retained_and_reconstructed_artifact_apis_match_policy() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan_with_precompute(&bootstrap, &|node, view| {
            let result = if node == source {
                view.finish(version_ab(1, 0))
            } else {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .unwrap();
    assert!(graph
        .observe()
        .materialize()
        .retained_explanation_artifact(dependent)
        .is_some());
    assert!(graph
        .observe()
        .materialize()
        .retained_provenance_artifact(dependent)
        .is_some());
    assert_eq!(
        graph
            .observe()
            .materialize()
            .retained_explanation_artifact(dependent)
            .unwrap()
            .materialization_mode,
        DiagnosticsAvailability::RetainedAvailable
    );
    assert_eq!(
        graph
            .observe()
            .materialize()
            .retained_provenance_artifact(dependent)
            .unwrap()
            .materialization_mode,
        DiagnosticsAvailability::RetainedAvailable
    );

    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    assert!(graph
        .observe()
        .materialize()
        .retained_explanation_artifact(dependent)
        .is_none());
    assert!(graph
        .observe()
        .materialize()
        .retained_provenance_artifact(dependent)
        .is_none());
    let reconstructed_explanation = graph
        .observe()
        .materialize()
        .reconstruct_explanation_artifact(dependent)
        .unwrap();
    let reconstructed_provenance = graph
        .observe()
        .materialize()
        .reconstruct_provenance_artifact(dependent)
        .unwrap();
    assert_eq!(
        reconstructed_explanation.materialization_mode,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(
        reconstructed_provenance.materialization_mode,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert!(!reconstructed_explanation.upstream.is_empty());
    assert!(reconstructed_provenance
        .vertices
        .iter()
        .any(|vertex| vertex.node == dependent));
    assert_eq!(
        reconstructed_provenance.causal_links,
        reconstructed_explanation.causal_links
    );
    assert!(
        graph
            .observe()
            .metrics()
            .storage
            .hot_path_artifact_reconstruction_count
            >= 2
    );
}

#[test]
fn retained_and_reconstructed_artifacts_preserve_semantic_parity() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph.set_runtime_policy(SignalRuntimePolicy::development());

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan_with_precompute(&bootstrap, &|node, view| {
            let result = if node == source {
                view.finish(
                    NodeEvaluationResult::from_version(version_ab(5, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
                        .with_label("source-region"),
                )
            } else {
                let version = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-12"),
                )?;
                view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("dependent-rib")
                        .with_label("dependent-label"),
                )
            };
            Ok(result)
        })
        .unwrap();

    let retained_explanation = graph
        .observe()
        .materialize()
        .retained_explanation_artifact(dependent)
        .expect("development mode should retain explanation artifacts");
    let reconstructed_explanation = graph
        .reconstruct_explanation_artifact_without_retained_fast_path(dependent)
        .unwrap();
    assert_eq!(
        retained_explanation.upstream,
        reconstructed_explanation.upstream
    );
    assert_eq!(
        retained_explanation.historical_artifact_record,
        reconstructed_explanation.historical_artifact_record
    );
    assert_eq!(
        retained_explanation.causal_links,
        reconstructed_explanation.causal_links
    );
    assert_eq!(
        retained_explanation.changed_regions,
        reconstructed_explanation.changed_regions
    );
    assert_eq!(
        retained_explanation.reuse_certification,
        reconstructed_explanation.reuse_certification
    );
    let retained_historical = graph
        .observe()
        .materialize()
        .materialize_historical_artifact_record(dependent)
        .unwrap()
        .unwrap();
    let retained_trace = graph
        .observe()
        .materialize()
        .materialize_trace_summary(dependent)
        .unwrap()
        .unwrap();
    assert_eq!(
        crate::data::trace::SemanticArtifactParity::from_historical_artifact_record(
            &retained_historical
        ),
        crate::data::trace::SemanticArtifactParity::from_trace_summary(&retained_trace)
    );

    let retained_provenance = graph
        .observe()
        .materialize()
        .retained_provenance_artifact(dependent)
        .expect("development mode should retain provenance artifacts");
    let reconstructed_provenance = graph
        .reconstruct_provenance_artifact_without_retained_fast_path(dependent)
        .unwrap();
    assert_eq!(
        retained_provenance.vertices,
        reconstructed_provenance.vertices
    );
    assert_eq!(retained_provenance.edges, reconstructed_provenance.edges);
    assert_eq!(
        retained_provenance.causal_links,
        reconstructed_provenance.causal_links
    );
    assert_eq!(
        retained_provenance.rewiring,
        reconstructed_provenance.rewiring
    );
    let reconstructed_historical = reconstructed_explanation
        .historical_artifact_record
        .clone()
        .expect("reconstructed explanation should carry historical artifact record");
    assert_eq!(
        crate::data::trace::SemanticArtifactParity::from_historical_artifact_record(
            &retained_historical
        ),
        crate::data::trace::SemanticArtifactParity::from_historical_artifact_record(
            &reconstructed_historical
        )
    );
}
