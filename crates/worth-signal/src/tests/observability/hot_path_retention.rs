use crate::facade::{
    ArtifactRetentionPolicy, EvaluationRequestMode, NodeEvaluationResult, SignalGraph,
    SignalRuntimePolicy,
};
use crate::tests::support::{version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn hot_effect_path_only_retains_cold_artifact_records_when_policy_requires_it() {
    let mut development = SignalGraph::new();
    let dev_source = development.node().build();
    let dev_dependent = development.node().build();
    development
        .append_dependency(dev_dependent, dev_source, ASPECT_A)
        .unwrap();
    development.set_runtime_policy(SignalRuntimePolicy::development());

    let dev_bootstrap = development
        .build_evaluation_plan(
            &[dev_source, dev_dependent],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    development
        .execute_prepared_plan_with_precompute(&dev_bootstrap, &|node, view| {
            let result = if node == dev_source {
                view.finish(version_ab(7, 0))
            } else {
                let version = view.read_aspect_version(dev_source, ASPECT_A)?;
                view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("retained-development")
                        .with_label("development-retained"),
                )
            };
            Ok(result)
        })
        .unwrap();

    assert!(
        development
            .get_entry(dev_dependent)
            .unwrap()
            .retained_diagnostic_artifact()
            .is_some(),
        "development retention should keep cold artifact richness on-node"
    );
    assert!(
        development
            .observe()
            .metrics()
            .storage
            .hot_path_artifact_retention_count
            > 0
    );
    assert!(
        development
            .observe()
            .metrics()
            .storage
            .hot_write_runtime_artifact_count
            > 0
    );
    assert!(
        development
            .observe()
            .metrics()
            .storage
            .hot_write_cold_record_materialization_count
            > 0
    );
    assert_eq!(
        development
            .observe()
            .metrics()
            .storage
            .hot_write_cold_bypass_count,
        0
    );
    assert!(
        development
            .observe()
            .metrics()
            .storage
            .eager_cold_artifact_materialization_count
            > 0
    );
    assert_eq!(
        development
            .observe()
            .metrics()
            .storage
            .deferred_cold_artifact_bypass_count,
        0
    );

    let mut omitted = SignalGraph::new();
    let op_source = omitted.node().build();
    let op_dependent = omitted.node().build();
    omitted
        .append_dependency(op_dependent, op_source, ASPECT_A)
        .unwrap();
    omitted.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );

    let op_bootstrap = omitted
        .build_evaluation_plan(
            &[op_source, op_dependent],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    omitted
        .execute_prepared_plan_with_precompute(&op_bootstrap, &|node, view| {
            let result = if node == op_source {
                view.finish(version_ab(7, 0))
            } else {
                let version = view.read_aspect_version(op_source, ASPECT_A)?;
                view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("retained-operational")
                        .with_label("operational-cold-seed"),
                )
            };
            Ok(result)
        })
        .unwrap();

    assert!(
        omitted
            .get_entry(op_dependent)
            .unwrap()
            .retained_diagnostic_artifact()
            .is_none(),
        "omit retention should keep cold artifact richness off the hot node lane"
    );
    assert_eq!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_path_artifact_retention_count,
        0,
        "omit mode should not report eager cold artifact retention"
    );
    assert_eq!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_write_cold_record_materialization_count,
        0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_write_runtime_artifact_count
            > 0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_write_cold_bypass_count
            > 0
    );
    assert_eq!(
        omitted
            .observe()
            .metrics()
            .storage
            .eager_cold_artifact_materialization_count,
        0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .deferred_cold_artifact_bypass_count
            > 0,
        "omit mode should prove that cold artifact assembly was bypassed by policy"
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_node_inline_size_bytes
            > 0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .warm_node_inline_size_bytes
            > 0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .hot_runtime_artifact_inline_size_bytes
            > 0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .warm_runtime_artifact_inline_size_bytes
            > 0
    );
    assert!(
        omitted
            .observe()
            .metrics()
            .storage
            .cold_artifact_record_inline_size_bytes
            > 0
    );
}
