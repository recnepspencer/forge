use crate::facade::{
    BridgeMergeConsumptionClass, BridgeMergeDenialClass, BridgeMergePrecedenceStage,
    BridgeMergeRoutingOutcomeClass, BridgeMergeStructuralAdvisoryDisposition,
    MergeHistoryDeclarationIdentity, TruthCommitIdentity,
};

use super::super::support::merge_declaration;
use super::support::{many_to_one_mapping_declaration, runtime_with_merge};

#[test]
fn ordered_parent_history_remains_deterministic_under_adapter_variation() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::new("merge:ordered-parent-determinism"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        [
            TruthCommitIdentity::new("parent-a"),
            TruthCommitIdentity::new("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let left_runtime = runtime_with_merge(declaration.clone());
    let right_runtime = runtime_with_merge(declaration.clone());

    let left_contract = left_runtime
        .admit_merge_history(declaration.clone())
        .expect("left contract should admit");
    let right_contract = right_runtime
        .admit_merge_history(declaration)
        .expect("right contract should admit");
    let left_bundle = left_runtime
        .replay_merge_history(&left_contract)
        .expect("left bundle should replay");
    let right_bundle = right_runtime
        .replay_merge_history(&right_contract)
        .expect("right bundle should replay");

    assert_eq!(
        left_bundle
            .lowered_packet_set()
            .parent_order_digest_basis()
            .digest(),
        right_bundle
            .lowered_packet_set()
            .parent_order_digest_basis()
            .digest()
    );
    assert_eq!(left_bundle.digest(), right_bundle.digest());
    assert_eq!(
        left_bundle
            .contract()
            .validated_declaration()
            .declaration()
            .ontology_mapping()
            .digest(),
        right_bundle
            .contract()
            .validated_declaration()
            .declaration()
            .ontology_mapping()
            .digest()
    );
}

#[test]
fn merge_ontology_lowering_remains_lossless_under_many_to_one_bridge_class_mapping() {
    let declaration = many_to_one_mapping_declaration(MergeHistoryDeclarationIdentity::new(
        "merge:many-to-one-ontology",
    ));
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("many-to-one ontology mapping should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("many-to-one ontology mapping should replay");
    let mapping = contract
        .validated_declaration()
        .declaration()
        .ontology_mapping();

    assert_eq!(mapping.entries().len(), 2);
    assert_eq!(
        contract
            .validated_declaration()
            .declaration()
            .bridge_class(),
        BridgeMergeConsumptionClass::AspectReconciliationMerge
    );
    assert!(!bundle.digest().is_empty());
}

#[test]
fn unsupported_merge_classes_fail_without_branch_reconciliation_authority_escape() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::new("merge:topology-denial"),
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        [
            TruthCommitIdentity::new("parent-a"),
            TruthCommitIdentity::new("parent-b"),
        ],
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");

    assert_eq!(
        bundle.reduced_routing_artifact().outcome_class(),
        BridgeMergeRoutingOutcomeClass::Denied
    );
    assert_eq!(
        bundle.lowered_packet_set().blocked_stage(),
        Some(BridgeMergePrecedenceStage::DeletionTopologyGate)
    );
    assert_eq!(
        bundle.lowered_packet_set().denial_class(),
        Some(BridgeMergeDenialClass::TopologyRewireGate)
    );
    assert!(bundle.continuity_artifact().is_none());
    assert!(bundle.remap_artifact().is_none());
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_continuity_count(),
        0
    );
}

#[test]
fn topology_rewire_denial_is_typed_and_keeps_counter_scope_local() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::new("merge:topology-rewire-denial"),
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        [
            TruthCommitIdentity::new("parent-a"),
            TruthCommitIdentity::new("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("topology rewire declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("topology rewire bundle should reconstruct");

    assert_eq!(
        bundle.lowered_packet_set().denial_class(),
        Some(BridgeMergeDenialClass::TopologyRewireGate)
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_topology_rewire_class_count(),
        1
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_history_segment_scan_count(),
        1
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_widened_scan_count(),
        0
    );
}

#[test]
fn merge_replay_preserves_routing_and_explanation_parity() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::new("merge:replay-parity"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        [
            TruthCommitIdentity::new("parent-a"),
            TruthCommitIdentity::new("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should admit");
    let original_bundle = runtime
        .replay_merge_history(&contract)
        .expect("original bundle should reconstruct");
    let canonical_record = runtime.canonicalize_merge_record(&original_bundle);
    let replayed_bundle = runtime
        .replay_canonical_merge_record(&canonical_record)
        .expect("canonical replay should reconstruct");

    assert_eq!(original_bundle.digest(), replayed_bundle.digest());
    assert_eq!(
        original_bundle.explanation_artifact().digest(),
        replayed_bundle.explanation_artifact().digest()
    );
    assert_eq!(
        replayed_bundle
            .reduced_routing_artifact()
            .counters()
            .merge_history_segment_scan_count(),
        1
    );
    assert_eq!(
        replayed_bundle
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        1
    );
}
