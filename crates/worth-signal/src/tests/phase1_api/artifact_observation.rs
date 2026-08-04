use crate::facade::*;
use crate::tests::support::*;

#[test]
fn observer_exposes_runtime_and_retained_artifacts_separately() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let node = graph.node().output_identity().build();
    let runtime_only = graph.node().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(7, 0))
            .with_output_identity("wing-surface")
            .with_continuity_token("wing-lineage")
            .with_label("forensic"))
    };
    evaluate(&mut graph, node, &mut compute).unwrap();
    let mut runtime_only_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(8, 0));
    evaluate(&mut graph, runtime_only, &mut runtime_only_compute).unwrap();
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_causality(Some(CausalityMetadata {
            kind: "host_patch".to_string(),
            fields: std::collections::BTreeMap::from([(
                "patch_id".to_string(),
                "wing-42".to_string(),
            )]),
        }));

    let observer = graph.observe();
    let runtime = observer.runtime_artifact_state(node).unwrap().unwrap();
    let retained = observer
        .retained_diagnostic_artifact(node)
        .unwrap()
        .unwrap();
    let materializer = observer.materialize();
    let historical = materializer
        .materialize_historical_artifact_record(node)
        .unwrap()
        .unwrap();
    let trace = materializer
        .materialize_trace_summary(node)
        .unwrap()
        .unwrap();

    assert_eq!(
        runtime.output_identity().map(|id| id.as_str()),
        Some("wing-surface")
    );
    assert_eq!(
        runtime.continuity_token().map(|token| token.as_str()),
        Some("wing-lineage")
    );
    assert_eq!(
        runtime.memoized_origin(),
        MemoizedResultOrigin::DirectCompute
    );
    assert_eq!(
        runtime.reuse_basis().clone_inner(),
        ReuseBasis::fresh_compute()
    );
    assert_eq!(retained.labels, vec!["forensic".to_owned()]);
    assert_eq!(historical.node, node);
    assert_eq!(
        historical.runtime.output_identity().cloned(),
        runtime.output_identity().cloned()
    );
    assert_eq!(
        historical.runtime.reuse_basis().clone_inner(),
        runtime.reuse_basis().clone_inner()
    );
    assert_eq!(
        historical.retained.as_ref().unwrap().labels,
        retained.labels
    );
    assert_eq!(trace.reuse_basis, runtime.reuse_basis().clone_inner());
    assert_eq!(
        historical
            .causality
            .as_ref()
            .and_then(|causality| causality.fields.get("patch_id"))
            .map(|value| value.as_str()),
        Some("wing-42")
    );
    assert_eq!(trace.labels, vec!["forensic".to_owned()]);
    assert_eq!(
        trace.output_identity.as_ref().map(|id| id.as_str()),
        Some("wing-surface")
    );

    let runtime_only_state = observer
        .runtime_artifact_state(runtime_only)
        .unwrap()
        .unwrap();
    assert!(
        observer
            .retained_diagnostic_artifact(runtime_only)
            .unwrap()
            .is_none(),
        "runtime-only artifacts must not require retained richness"
    );
    let runtime_only_historical = materializer
        .materialize_historical_artifact_record(runtime_only)
        .unwrap()
        .unwrap();
    assert!(
        runtime_only_historical.retained.is_none(),
        "cold historical assembly should remain available without retained payload"
    );
    let runtime_only_trace = materializer
        .materialize_trace_summary(runtime_only)
        .unwrap()
        .unwrap();
    assert_eq!(
        runtime_only_trace.output_hash,
        runtime_only_state.output_hash(),
        "cold trace assembly should derive from runtime truth even when retained richness is absent"
    );
}
