use super::merge_certification_world::build_shared_state_conflict_runtime;
use crate::facade::{SignalGraph, SignalRuntime};
use crate::logic::transaction::{
    merge_plan_proof_report, merge_result_proof_report, runtime_proof_report,
    MERGE_PROOF_SCHEMA_VERSION,
};

#[test]
fn runtime_proof_report_is_versioned_and_bundle_stable() {
    let runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let left = runtime_proof_report(
        runtime.schema_registry().registry_digest(),
        runtime.merge_strategy_registry().registry_digest(),
        runtime.merge_base_strategy_registry().registry_digest(),
        runtime.aspect_merge_policy_registry().registry_digest(),
        runtime.conflict_isolation_registry().registry_digest(),
        runtime.conflict_policy_registry().registry_digest(),
        runtime.identity_matcher_registry().registry_digest(),
        runtime.source_only_policy_registry().registry_digest(),
        runtime.deletion_policy_registry().registry_digest(),
    );
    let right = runtime_proof_report(
        runtime.schema_registry().registry_digest(),
        runtime.merge_strategy_registry().registry_digest(),
        runtime.merge_base_strategy_registry().registry_digest(),
        runtime.aspect_merge_policy_registry().registry_digest(),
        runtime.conflict_isolation_registry().registry_digest(),
        runtime.conflict_policy_registry().registry_digest(),
        runtime.identity_matcher_registry().registry_digest(),
        runtime.source_only_policy_registry().registry_digest(),
        runtime.deletion_policy_registry().registry_digest(),
    );

    assert_eq!(left.proof_schema_version, MERGE_PROOF_SCHEMA_VERSION);
    assert_eq!(left.registry_bundle_digest, right.registry_bundle_digest);
    assert_eq!(
        left.conflict_isolation_registry_digest,
        runtime.conflict_isolation_registry().registry_digest()
    );
}

#[test]
fn merge_plan_proof_report_is_stable_and_matches_lowered_semantics_bundle() {
    let (mut runtime, feature, main) = build_shared_state_conflict_runtime();
    let planned = runtime
        .merge()
        .from(feature)
        .into(main)
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .plan()
        .unwrap();

    let left = merge_plan_proof_report(planned.plan(), planned.plan().registry_bundle_digest());
    let right = merge_plan_proof_report(planned.plan(), planned.plan().registry_bundle_digest());
    let semantics = planned.plan().selected_semantics();

    assert_eq!(left.proof_schema_version, MERGE_PROOF_SCHEMA_VERSION);
    assert_eq!(left.plan_digest, right.plan_digest);
    assert_eq!(left.semantics_digest, right.semantics_digest);
    assert_eq!(left.selected_strategy_digest, semantics.strategy_digest);
    assert_eq!(left.selected_merge_base_digest, semantics.merge_base_digest);
    assert_eq!(
        left.selected_conflict_policy_digest,
        semantics.conflict_policy_digest
    );
    assert_eq!(
        left.selected_conflict_isolation_digest,
        semantics.conflict_isolation_digest
    );
    assert_eq!(
        left.selected_identity_matcher_digest,
        semantics.identity_matcher_digest
    );
    assert_eq!(
        left.selected_source_only_policy_digest,
        semantics.source_only_policy_digest
    );
    assert_eq!(
        left.selected_deletion_policy_digest,
        semantics.deletion_policy_digest
    );
}

#[test]
fn merge_result_proof_report_is_stable_and_matches_lowered_semantics_bundle() {
    let (mut runtime, feature, main) = build_shared_state_conflict_runtime();
    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .run()
        .unwrap();

    let left = merge_result_proof_report(&result);
    let right = merge_result_proof_report(&result);
    let semantics = &result.selected_semantics;

    assert_eq!(left.proof_schema_version, MERGE_PROOF_SCHEMA_VERSION);
    assert_eq!(left.result_digest, right.result_digest);
    assert_eq!(left.semantics_digest, right.semantics_digest);
    assert_eq!(left.selected_strategy_digest, semantics.strategy_digest);
    assert_eq!(left.selected_merge_base_digest, semantics.merge_base_digest);
    assert_eq!(
        left.selected_conflict_policy_digest,
        semantics.conflict_policy_digest
    );
    assert_eq!(
        left.selected_conflict_isolation_digest,
        semantics.conflict_isolation_digest
    );
    assert_eq!(
        left.selected_identity_matcher_digest,
        semantics.identity_matcher_digest
    );
    assert_eq!(
        left.selected_source_only_policy_digest,
        semantics.source_only_policy_digest
    );
    assert_eq!(
        left.selected_deletion_policy_digest,
        semantics.deletion_policy_digest
    );
    assert_eq!(left.registry_bundle_digest, result.registry_bundle_digest,);
    assert_eq!(
        left.lowered_strategy_bundle_digest,
        result.lowered_strategy_bundle_digest,
    );
    assert_eq!(
        left.lineage_digest,
        crate::logic::transaction::merge_lineage_digest(&result)
    );
}

#[test]
fn merge_plan_proof_digest_changes_when_conflict_isolation_selection_changes() {
    let (mut runtime, feature, main) = build_shared_state_conflict_runtime();
    let per_node_proof = {
        let per_node = runtime
            .merge()
            .from(feature.clone())
            .into(main.clone())
            .conflict_isolation_policy_named("signal.conflict-isolation.per-node")
            .plan()
            .unwrap();
        merge_plan_proof_report(per_node.plan(), per_node.plan().registry_bundle_digest())
    };
    let per_aspect_proof = {
        let per_aspect = runtime
            .merge()
            .from(feature)
            .into(main)
            .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
            .plan()
            .unwrap();
        merge_plan_proof_report(
            per_aspect.plan(),
            per_aspect.plan().registry_bundle_digest(),
        )
    };

    assert_ne!(per_node_proof.plan_digest, per_aspect_proof.plan_digest);
    assert_ne!(
        per_node_proof.semantics_digest,
        per_aspect_proof.semantics_digest
    );
    assert_ne!(
        per_node_proof.selected_conflict_isolation_digest,
        per_aspect_proof.selected_conflict_isolation_digest
    );
}
