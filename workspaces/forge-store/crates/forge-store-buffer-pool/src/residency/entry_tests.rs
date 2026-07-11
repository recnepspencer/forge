use crate::{
    BufferPoolBudget, BufferPoolEntryDenial, BufferPoolEntryDenialKind, DirtyPageBudget,
    PinnedPageBudget, ResidencyAuthorityTerm, ResidencyVocabulary, ResidentMemoryBudget,
    S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
    S2PhysicalSubstrateReadiness,
};

#[test]
fn buffer_pool_entry_consumes_s1_readiness_and_budget() {
    let s2_physical_substrate_readiness = s2_physical_substrate_readiness();
    let buffer_pool_budget = buffer_pool_budget();
    let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        s2_physical_substrate_readiness.physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(buffer_pool_budget)
    .admit()
    .unwrap();

    assert_eq!(admitted.admission().budget(), buffer_pool_budget);
    assert_eq!(admitted.admission().facts().physical_reference_count(), 2);
    assert_eq!(
        admitted
            .admission()
            .facts()
            .payload_admission_witness_count(),
        1
    );
    assert_eq!(
        admitted
            .admission()
            .facts()
            .manifest_layout_evidence_count(),
        3
    );
    assert_eq!(
        admitted
            .admission()
            .facts()
            .no_materialization_witness_count(),
        2
    );
    assert!(admitted.admission().facts().counter_evidence_count() > 0);
}

#[test]
fn independent_s1_handoffs_lower_to_same_s2_entry_facts_and_vocabulary() {
    let first_independent_entry = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        s2_physical_substrate_readiness().physical_substrate_snapshot(),
    )
    .unwrap();
    let second_independent_entry = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        s2_physical_substrate_readiness().physical_substrate_snapshot(),
    )
    .unwrap();

    assert_eq!(
        first_independent_entry.facts(),
        second_independent_entry.facts()
    );
    assert_eq!(
        ResidencyVocabulary::s2_phase_one(),
        ResidencyVocabulary::s2_phase_one()
    );
}

#[test]
fn residency_vocabulary_freezes_phase_one_authority_terms() {
    assert_eq!(
        ResidencyVocabulary::s2_phase_one(),
        &[
            ResidencyAuthorityTerm::ResidentMemory,
            ResidencyAuthorityTerm::PinnedPage,
            ResidencyAuthorityTerm::DirtyPage,
            ResidencyAuthorityTerm::CopiedBytes,
            ResidencyAuthorityTerm::MaterializedBytes,
            ResidencyAuthorityTerm::AllocationEnvelope,
            ResidencyAuthorityTerm::ReadinessHandoff,
            ResidencyAuthorityTerm::ResidentFrameTable,
            ResidencyAuthorityTerm::ResidentFrameGeneration,
            ResidencyAuthorityTerm::ResidentFrameHitMissCounters,
        ]
    );
}

#[test]
fn shortcut_denials_are_typed_not_string_profiles() {
    let denial = BufferPoolEntryDenial::forbidden_shortcut(
        BufferPoolEntryDenialKind::FoundationalEvidenceAsAuthorityRejected,
    );

    assert_eq!(
        denial.kind(),
        BufferPoolEntryDenialKind::FoundationalEvidenceAsAuthorityRejected
    );
}

#[test]
fn budget_declaration_keeps_resident_pinned_and_dirty_distinct() {
    let buffer_pool_budget = buffer_pool_budget();

    assert_eq!(buffer_pool_budget.resident_memory().as_bytes(), 64);
    assert_eq!(buffer_pool_budget.pinned_pages().as_pages(), 4);
    assert_eq!(buffer_pool_budget.dirty_pages().as_pages(), 2);
}

fn s2_physical_substrate_readiness() -> S2PhysicalSubstrateReadiness {
    let closeout = close_s1_physical_substrate_readiness(accepted_s1_readiness()).unwrap();
    prove_s2_physical_substrate_readiness(closeout).unwrap()
}

fn buffer_pool_budget() -> BufferPoolBudget {
    BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(64).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    )
}

fn accepted_s1_readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        ),
    )
    .unwrap()
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).unwrap()
}
