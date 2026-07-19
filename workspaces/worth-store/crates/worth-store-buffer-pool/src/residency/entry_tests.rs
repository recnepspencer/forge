use crate::{
    BufferPoolBudget, BufferPoolEntryDenial, BufferPoolEntryDenialKind, DirtyPageBudget,
    PinnedPageBudget, ResidencyAuthorityTerm, ResidencyVocabulary, ResidentMemoryBudget,
    S2PhysicalResidencyEntry,
};
use worth_store_contracts::PhysicalSubstrateReadinessSnapshot;

#[test]
fn buffer_pool_entry_applies_snapshot_admission_and_budget_laws() {
    let buffer_pool_budget = buffer_pool_budget();
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
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
fn independent_model_snapshots_lower_to_same_entry_facts_and_vocabulary() {
    let first_independent_entry =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap();
    let second_independent_entry =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap();

    assert_eq!(
        first_independent_entry.facts(),
        second_independent_entry.facts()
    );
    assert_eq!(
        ResidencyVocabulary::physical_substrate_vocabulary(),
        ResidencyVocabulary::physical_substrate_vocabulary()
    );
}

#[test]
fn residency_vocabulary_freezes_authority_terms() {
    assert_eq!(
        ResidencyVocabulary::physical_substrate_vocabulary(),
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

fn algorithm_model_snapshot() -> PhysicalSubstrateReadinessSnapshot {
    PhysicalSubstrateReadinessSnapshot::from_exact_counts(true, 2, 1, 1, 3, 2, 5)
}

fn buffer_pool_budget() -> BufferPoolBudget {
    BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(64).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    )
}
