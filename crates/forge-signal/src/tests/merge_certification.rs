use crate::facade::*;
use crate::logic::transaction::{
    branch_state_proof_report, canonical_digest, merge_plan_proof_report,
    merge_result_proof_report, runtime_proof_report, BranchStateDenseGridProofBasis,
    BranchStateProofBasis, ReplayArtifactProofInput, ReplayMismatchClass,
    BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
};
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};
use crate::tests::support::*;
use std::collections::BTreeMap;

fn certification_schema_registry() -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_isolation(
            SignalSchemaId(91),
            SignalSchemaName::new("signal.demo.merge-certification-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            Some(ConflictPolicyName::new(
                "signal.conflict.resolve-source-when-structure-matches",
            )),
            None,
            None,
            None,
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn certification_aspect_schema_registry() -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_aspects(
            SignalSchemaId(92),
            SignalSchemaName::new("signal.demo.merge-certification-aspect-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            None,
            None,
            vec![AspectMergePolicyBinding::new(
                ASPECT_A,
                AspectMergePolicyName::new("signal.aspect.prefer-source"),
            )],
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn build_shared_state_conflict_runtime() -> (
    SignalRuntime<(), (), (), (), ()>,
    SignalBranchHandle,
    SignalBranchHandle,
) {
    let graph = SignalGraph::new().with_schema_registry(certification_schema_registry());
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-certification-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A, ASPECT_B])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(501, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-merge-certification")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(502, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(503, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    (runtime, feature, main)
}

fn build_aspect_policy_runtime() -> (
    SignalRuntime<(), (), (), (), ()>,
    SignalBranchHandle,
    SignalBranchHandle,
) {
    let graph = SignalGraph::new().with_schema_registry(certification_aspect_schema_registry());
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(511, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-merge-certification-aspect")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-certification-aspect-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    (runtime, feature, main)
}

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

#[test]
fn merge_execution_counters_obey_bounded_shared_conflict_contract() {
    let (mut runtime, feature, main) = build_shared_state_conflict_runtime();
    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .run()
        .unwrap();

    assert!(
        result.counters.source_slice_breadth >= result.counters.final_candidate_breadth,
        "source slice breadth must dominate the final candidate set"
    );
    assert_eq!(
        result.counters.final_candidate_breadth,
        result.planned_candidates.nodes.len() as u64,
        "final candidate breadth should match the lowered candidate node set"
    );
    assert!(
        result.counters.reconciliation_breadth <= result.counters.final_candidate_breadth,
        "reconciliation breadth must stay within the admitted candidate set"
    );
    assert_eq!(
        result.counters.conflict_isolation_record_count,
        result.conflict_isolation_plan.records.len() as u64,
        "conflict isolation record count should mirror the lowered isolation plan"
    );
    assert_eq!(
        result.counters.conflict_isolation_expansion_breadth,
        result.conflict_isolation_plan.expansion_breadth,
        "conflict isolation breadth counters must mirror the lowered isolation plan"
    );
    assert_eq!(
        result.counters.conflict_isolation_expansion_breadth, 0,
        "current conflict isolation lowering must not widen candidate admission"
    );
    assert_eq!(
        result.counters.identity_ambiguous_match_count, 0,
        "shared-state conflict certification case should not introduce identity ambiguity"
    );
    assert_eq!(
        result.counters.identity_rejected_admissibility_count, 0,
        "shared-state conflict certification case should not rely on rejected identity admissibility"
    );
}

#[test]
fn aspect_policy_and_decision_lowering_remain_consistent() {
    let (mut runtime, feature, main) = build_aspect_policy_runtime();
    let planned = runtime.merge().from(feature).into(main).plan().unwrap();

    let aspect_policy_plan = planned.plan().aspect_policy_plan();
    let aspect_decision_plan = planned.plan().aspect_decision_plan();

    assert_eq!(aspect_policy_plan.records.len(), 1);
    assert_eq!(aspect_decision_plan.records.len(), 1);
    assert_eq!(aspect_policy_plan.records[0].aspect, ASPECT_A);
    assert_eq!(aspect_decision_plan.records[0].aspect, ASPECT_A);
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_decision_plan.records[0]
            .selected_policy_name
            .as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_digest,
        aspect_decision_plan.records[0].selected_policy_digest
    );
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_basis,
        aspect_decision_plan.records[0].selected_policy_basis
    );
    assert_eq!(
        aspect_decision_plan.records[0].outcome,
        AspectMergeDecisionOutcome::SourceIntroducedIntoTarget
    );
}

#[test]
fn merge_base_selection_remains_consistent_from_plan_to_result_proof() {
    let (mut runtime, feature, main) = build_aspect_policy_runtime();
    let (
        plan_selected_merge_base_name,
        plan_selected_merge_base_digest,
        lowered_selected_merge_base_digest,
        plan_proof_selected_merge_base_digest,
    ) = {
        let planned = runtime
            .merge()
            .from(feature.clone())
            .into(main.clone())
            .merge_base_named("signal.merge-base.fork-point")
            .plan()
            .unwrap();
        let plan_proof =
            merge_plan_proof_report(planned.plan(), planned.plan().registry_bundle_digest());
        let lowered_merge_base = planned
            .plan()
            .lowered_merge_base()
            .expect("lowered merge-base plan");
        (
            planned.plan().selected_semantics().merge_base_name.clone(),
            planned
                .plan()
                .selected_semantics()
                .merge_base_digest
                .clone(),
            lowered_merge_base.selected_merge_base_digest.clone(),
            plan_proof.selected_merge_base_digest,
        )
    };

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .merge_base_named("signal.merge-base.fork-point")
        .run()
        .unwrap();
    let result_proof = merge_result_proof_report(&result);

    assert_eq!(
        plan_selected_merge_base_name,
        result.selected_semantics.merge_base_name
    );
    assert_eq!(
        plan_selected_merge_base_digest,
        result.selected_semantics.merge_base_digest
    );
    assert_eq!(
        lowered_selected_merge_base_digest,
        plan_proof_selected_merge_base_digest
    );
    assert_eq!(
        result.selected_merge_base_digest,
        result_proof.selected_merge_base_digest
    );
    assert_eq!(
        plan_proof_selected_merge_base_digest,
        result_proof.selected_merge_base_digest
    );
}

#[test]
fn branch_state_proof_basis_is_stable_at_core_boundary() {
    let basis = BranchStateProofBasis {
        proof_schema_version: BRANCH_STATE_PROOF_BASIS_VERSION.to_owned(),
        catalog_ids: vec!["gearTeeth".to_owned(), "hudModel".to_owned()],
        dense_grids: vec![BranchStateDenseGridProofBasis {
            family_id: "gearToothModel".to_owned(),
            width: 8,
            height: 1,
            key_count: 8,
            ids: vec!["tooth-0".to_owned(), "tooth-1".to_owned()],
        }],
        store: BTreeMap::from([
            ("gearTeeth".to_owned(), 22_u64),
            ("lightIntensity".to_owned(), 178_u64),
        ]),
    };

    let left = branch_state_proof_report(
        7,
        "main",
        Some(42),
        BRANCH_STATE_PROOF_BASIS_VERSION,
        &basis,
    );
    let right = branch_state_proof_report(
        7,
        "main",
        Some(42),
        BRANCH_STATE_PROOF_BASIS_VERSION,
        &basis,
    );

    assert_eq!(
        left.proof_schema_version,
        format!(
            "{}:{}",
            MERGE_PROOF_SCHEMA_VERSION, BRANCH_STATE_PROOF_BASIS_VERSION
        )
    );
    assert_eq!(left.state_digest, right.state_digest);
    assert_eq!(left.state_digest, canonical_digest(&basis));
}

#[test]
fn replay_artifact_proof_report_surfaces_typed_mismatch_classes() {
    let expected = ReplayArtifactProofInput {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: Some("registry-a".to_owned()),
        lowered_strategy_bundle_digest: Some("bundle-a".to_owned()),
        merge_plan_digest: Some("plan-a".to_owned()),
        merge_result_digest: Some("result-a".to_owned()),
        lineage_digest: Some("lineage-a".to_owned()),
        branch_state_digest: "state-a".to_owned(),
    };
    let replayed = ReplayArtifactProofInput {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: Some("registry-b".to_owned()),
        lowered_strategy_bundle_digest: Some("bundle-b".to_owned()),
        merge_plan_digest: Some("plan-b".to_owned()),
        merge_result_digest: Some("result-b".to_owned()),
        lineage_digest: Some("lineage-b".to_owned()),
        branch_state_digest: "state-b".to_owned(),
    };

    let report =
        crate::logic::transaction::replay_artifact_proof_report(expected.clone(), replayed.clone());

    assert!(!report.parity);
    assert_eq!(
        report.expected.registry_bundle_digest,
        expected.registry_bundle_digest
    );
    assert_eq!(
        report.replayed.branch_state_digest,
        replayed.branch_state_digest
    );
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::RegistryBundleDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::LoweredStrategyBundleDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::MergePlanDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::MergeResultDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::LineageDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::BranchStateDigestMismatch));
}
