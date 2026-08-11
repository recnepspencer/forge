use super::*;

#[test]
fn runtime_lowers_and_reduces_merge_continuity_candidate() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:analysis"),
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
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:deletion"),
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
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:causal-truncated"),
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
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:structural-contradiction"),
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
