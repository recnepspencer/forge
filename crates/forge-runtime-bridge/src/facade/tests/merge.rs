use super::*;
use crate::facade::{
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasis,
    BridgeMergeAuthorityBasisKind, BridgeMergeCausalFrontierDisposition,
    BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface, BridgeMergeParentOrderProof,
    BridgeMergePrecedenceStage, BridgeMergeRoutingOutcomeClass, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclaration,
    MergeHistoryDeclarationIdentity,
};

fn registered_merge(id: &str, class: BridgeMergeConsumptionClass) -> MergeHistoryDeclaration {
    MergeHistoryDeclaration::new(
        MergeHistoryDeclarationIdentity::new(id),
        class,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            format!("merge-artifact:{id}"),
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                TruthCommitIdentity::new("parent-a"),
                TruthCommitIdentity::new("parent-b"),
            ]),
        ),
    )
}

fn runtime_with_merge(declaration: MergeHistoryDeclaration) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_merge(declaration)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build with merge declaration")
}

#[test]
fn runtime_admits_registered_merge_declaration() {
    let declaration = registered_merge(
        "merge:analysis",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());

    let contract = runtime
        .admit_merge_history(declaration)
        .expect("registered merge declaration should be admitted");
    assert_eq!(
        contract
            .validated_declaration()
            .declaration()
            .bridge_class(),
        BridgeMergeConsumptionClass::AspectReconciliationMerge
    );
}

#[test]
fn runtime_lowers_and_reduces_merge_continuity_candidate() {
    let declaration = registered_merge(
        "merge:analysis",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("lowered merge packet should reduce");

    assert_eq!(
        reduced.outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
    assert_eq!(lowered.blocked_stage(), None);
    assert_eq!(lowered.counters().merge_packet_count(), 1);
}

#[test]
fn runtime_denies_deletion_merge_at_deletion_topology_stage() {
    let declaration =
        registered_merge("merge:deletion", BridgeMergeConsumptionClass::DeletionMerge);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("lowered merge packet should reduce");

    assert_eq!(
        reduced.outcome_class(),
        BridgeMergeRoutingOutcomeClass::Denied
    );
    assert_eq!(
        lowered.blocked_stage(),
        Some(BridgeMergePrecedenceStage::DeletionTopologyGate)
    );
}

#[test]
fn runtime_denies_causal_truncation_at_causal_stage() {
    let declaration = registered_merge(
        "merge:causal-truncated",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_authoritative_lineage(BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor)
    .with_causal_frontier(BridgeMergeCausalFrontierDisposition::Truncated)
    .with_schema_policy(BridgeMergeSchemaPolicyDisposition::Admitted);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");

    assert_eq!(
        lowered.blocked_stage(),
        Some(BridgeMergePrecedenceStage::CausalFrontierAdmissibility)
    );
    assert_eq!(lowered.counters().merge_causal_frontier_lookup_count(), 1);
}

#[test]
fn runtime_localizes_structural_contradiction_without_reopening_continuity() {
    let declaration = registered_merge(
        "merge:structural-contradiction",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryContradiction);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("lowered merge packet should reduce");

    assert_eq!(
        reduced.outcome_class(),
        BridgeMergeRoutingOutcomeClass::StructuralContradiction
    );
    assert_eq!(lowered.blocked_stage(), None);
    assert_eq!(lowered.counters().merge_continuity_count(), 0);
    assert_eq!(lowered.counters().merge_structural_contradiction_count(), 1);
}

#[test]
fn runtime_publishes_merge_continuity_and_explanation_bundle() {
    let declaration = registered_merge(
        "merge:publish",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");
    let continuity = runtime
        .publish_merge_continuity_artifact(&reduced)
        .expect("continuity publication should succeed");
    let explanation =
        runtime.publish_merge_explanation_artifact(&lowered, &reduced, Some(&continuity), None);

    assert!(continuity
        .canonical_basis()
        .contains("published-merge-continuity-artifact"));
    assert_eq!(explanation.continuity_digest(), Some(continuity.digest()));
    assert_eq!(
        explanation.outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
}

#[test]
fn runtime_publishes_merge_remap_only_when_structural_advisory_is_consistent() {
    let declaration = registered_merge(
        "merge:publish-remap",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");
    let remap = runtime
        .publish_merge_remap_artifact(&reduced)
        .expect("remap publication should succeed");

    assert!(remap
        .canonical_basis()
        .contains("published-merge-remap-artifact"));
}

#[test]
fn runtime_rejects_merge_publication_with_typed_denial_kind() {
    let declaration = registered_merge(
        "merge:typed-denial",
        BridgeMergeConsumptionClass::DeletionMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");

    let error = runtime
        .publish_merge_continuity_artifact(&reduced)
        .expect_err("deletion-gated merge should not publish continuity");

    assert_eq!(
        error.kind(),
        crate::error::BridgeMergeErrorKind::MergeDeletionDenied
    );
}

#[test]
fn runtime_replay_merge_history_certifies_full_bundle() {
    let declaration = registered_merge(
        "merge:replay-bundle",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge replay bundle should reconstruct");

    assert_eq!(bundle.contract().digest(), contract.digest());
    assert!(bundle.continuity_artifact().is_some());
    assert!(bundle.remap_artifact().is_some());
    assert_eq!(
        bundle.explanation_artifact().remap_digest(),
        bundle.remap_artifact().map(|artifact| artifact.digest())
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_remap_publication_count(),
        1
    );
    assert_eq!(
        bundle
            .explanation_artifact()
            .counters()
            .merge_explanation_request_count(),
        1
    );
}

#[test]
fn runtime_canonicalizes_and_explains_merge_record() {
    let declaration = registered_merge(
        "merge:diagnostics",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");
    let record = runtime.canonicalize_merge_record(&bundle);
    let explanation = runtime
        .diagnostics()
        .explain_last_merge_record()
        .expect("merge record should be retained");

    assert_eq!(record.bundle().digest(), bundle.digest());
    assert_eq!(
        runtime
            .diagnostics()
            .last_merge_record()
            .expect("merge record should be present")
            .record_identity(),
        record.record_identity()
    );
    assert_eq!(
        explanation.outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
    assert!(explanation.continuity_digest().is_some());
    assert!(explanation.remap_digest().is_some());
}

#[test]
fn runtime_replays_canonical_merge_record() {
    let declaration = registered_merge(
        "merge:canonical-replay",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");
    let record = runtime.canonicalize_merge_record(&bundle);

    let replayed = runtime
        .replay_canonical_merge_record(&record)
        .expect("canonical merge record should replay");

    assert_eq!(replayed.digest(), bundle.digest());
    assert_eq!(
        replayed
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        1
    );
    assert_eq!(
        replayed
            .explanation_artifact()
            .counters()
            .merge_explanation_request_count(),
        1
    );
}

#[test]
fn runtime_replay_rejects_incompatible_merge_record_version() {
    let declaration = registered_merge(
        "merge:canonical-replay-version",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");
    let record = runtime
        .canonicalize_merge_record(&bundle)
        .with_schema_version_for_test("forge-runtime-bridge.merge-record.v999");

    let error = runtime
        .replay_canonical_merge_record(&record)
        .expect_err("incompatible merge record version should be rejected");

    assert_eq!(
        error.kind(),
        crate::error::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
}
