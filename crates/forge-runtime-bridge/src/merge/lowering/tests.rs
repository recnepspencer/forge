use crate::input::envelope::TruthCommitIdentity;
use crate::merge::{
    AdmittedMergeHistoryContract, AdmittedMergeRegistry, BridgeMergeAuthorityBasis,
    BridgeMergeAuthorityBasisKind, BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface,
    BridgeMergeParentOrderProof, BridgeMergeStructuralAdvisoryDisposition,
    LoweredMergeHistoryPacketSet, MergeHistoryDeclaration, MergeHistoryDeclarationIdentity,
};

fn contract(structural: BridgeMergeStructuralAdvisoryDisposition) -> AdmittedMergeHistoryContract {
    let declaration = MergeHistoryDeclaration::new(
        MergeHistoryDeclarationIdentity::new("merge:test"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            "merge-artifact:test",
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                TruthCommitIdentity::new("parent-a"),
                TruthCommitIdentity::new("parent-b"),
            ]),
        ),
    )
    .with_structural_advisory(structural);
    AdmittedMergeRegistry::freeze(vec![declaration])
        .expect("merge registry should freeze")
        .contracts()[0]
        .clone()
}

#[test]
fn lowered_merge_packet_set_is_canonical_for_same_contract() {
    let contract = contract(BridgeMergeStructuralAdvisoryDisposition::NotConsulted);
    let left = LoweredMergeHistoryPacketSet::from_contract(&contract);
    let right = LoweredMergeHistoryPacketSet::from_contract(&contract);

    assert_eq!(left, right);
    assert_eq!(left.counters().merge_packet_count(), 1);
}

#[test]
fn lowered_merge_packet_set_marks_structural_contradictions() {
    let contract = contract(BridgeMergeStructuralAdvisoryDisposition::AdvisoryContradiction);
    let lowered = LoweredMergeHistoryPacketSet::from_contract(&contract);

    assert!(lowered.structural_contradiction());
    assert_eq!(lowered.counters().merge_structural_contradiction_count(), 1);
}
