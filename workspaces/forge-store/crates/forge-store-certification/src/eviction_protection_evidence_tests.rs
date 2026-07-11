use crate::{
    courtroom::harness::test_support::dirty_publication_evidence_test_support::{
        admit_payload_frame, resident_frame_table,
    },
    EvictionProtectionEvidenceReport, EvictionProtectionEvidenceRow,
};
use forge_store_buffer_pool::{EvictionPressure, EvictionProtectionReason};

#[test]
fn eviction_candidate_evidence_consumes_resident_frame_table_plan() {
    let mut table = resident_frame_table(3, 1);
    admit_payload_frame(&mut table, 7, 2, b"first");
    admit_payload_frame(&mut table, 8, 3, b"second");

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let candidate_evidence = EvictionProtectionEvidenceReport::from_plan(
        EvictionProtectionEvidenceRow::CandidateSetFromResidentFrameTable,
        &plan,
    )
    .unwrap();
    let scan_evidence = EvictionProtectionEvidenceReport::from_scan_bound(&table, &plan).unwrap();

    assert_eq!(
        candidate_evidence.row(),
        EvictionProtectionEvidenceRow::CandidateSetFromResidentFrameTable
    );
    assert_eq!(
        scan_evidence.row(),
        EvictionProtectionEvidenceRow::CandidateScanBoundedByResidentFrameCount
    );
    assert_eq!(candidate_evidence.counters().resident_frame_scan_count(), 2);
    assert_eq!(candidate_evidence.counters().candidate_count(), 2);
}

#[test]
fn scan_bound_evidence_accepts_sparse_resident_table_plan() {
    let mut table = resident_frame_table(64, 1);
    admit_payload_frame(&mut table, 7, 2, b"first");
    admit_payload_frame(&mut table, 8, 3, b"second");

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let evidence = EvictionProtectionEvidenceReport::from_scan_bound(&table, &plan).unwrap();

    assert_eq!(
        evidence.row(),
        EvictionProtectionEvidenceRow::CandidateScanBoundedByResidentFrameCount
    );
    assert_eq!(table.resident_frame_count(), 2);
    assert_eq!(evidence.counters().resident_frame_scan_count(), 2);
    assert_eq!(evidence.counters().candidate_count(), 2);
}

#[test]
fn protected_frame_exclusion_evidence_requires_exclusion_before_ranking() {
    let mut table = resident_frame_table(3, 1);
    let pinned = admit_payload_frame(&mut table, 7, 2, b"pinned");
    let verifier = admit_payload_frame(&mut table, 8, 3, b"verifier");
    admit_payload_frame(&mut table, 9, 4, b"clean");
    let pinned_lease = table
        .lease_page(pinned.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned_lease);
    table
        .protect_frame_for_verifier(verifier.resident_frame_token())
        .unwrap();

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let evidence = EvictionProtectionEvidenceReport::from_plan(
        EvictionProtectionEvidenceRow::ProtectedFramesExcludedBeforePolicyRanking,
        &plan,
    )
    .unwrap();
    let protected = plan.candidate_set().protected_exclusions();

    assert_eq!(
        evidence.row(),
        EvictionProtectionEvidenceRow::ProtectedFramesExcludedBeforePolicyRanking
    );
    assert!(protected.contains(EvictionProtectionReason::Pinned));
    assert!(protected.contains(EvictionProtectionReason::VerifierProtected));
    assert_eq!(evidence.counters().policy_rank_count(), 1);
    assert_eq!(evidence.counters().protected_exclusion_count(), 2);
}

#[test]
fn all_protected_eviction_denial_evidence_requires_precise_reasons() {
    let mut table = resident_frame_table(5, 1);
    let pinned = admit_payload_frame(&mut table, 6, 1, b"pinned");
    let dirty = admit_payload_frame(&mut table, 7, 2, b"dirty");
    let verifier = admit_payload_frame(&mut table, 8, 3, b"verifier");
    let recovery = admit_payload_frame(&mut table, 9, 4, b"recovery");
    let streaming = admit_payload_frame(&mut table, 10, 5, b"streaming");
    let pinned_lease = table
        .lease_page(pinned.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned_lease);
    table.mark_dirty(dirty.resident_frame_token()).unwrap();
    table
        .protect_frame_for_verifier(verifier.resident_frame_token())
        .unwrap();
    table
        .protect_frame_for_recovery(recovery.resident_frame_token())
        .unwrap();
    table
        .protect_frame_for_streaming(streaming.resident_frame_token())
        .unwrap();

    let denial = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap_err();
    let protected = denial.protected_frame_denial().unwrap().reasons();
    let evidence = EvictionProtectionEvidenceReport::from_denial(
        EvictionProtectionEvidenceRow::AllProtectedResidentSetDeniedWithReasons,
        denial,
    )
    .unwrap();

    assert_eq!(
        evidence.row(),
        EvictionProtectionEvidenceRow::AllProtectedResidentSetDeniedWithReasons
    );
    assert!(protected.contains(EvictionProtectionReason::Pinned));
    assert!(protected.contains(EvictionProtectionReason::DirtyUnpublished));
    assert!(protected.contains(EvictionProtectionReason::VerifierProtected));
    assert!(protected.contains(EvictionProtectionReason::RecoveryProtected));
    assert!(protected.contains(EvictionProtectionReason::StreamingProtected));
    assert_eq!(evidence.counters().all_protected_denial_count(), 1);
    assert_eq!(evidence.counters().policy_rank_count(), 0);
    assert_eq!(evidence.counters().candidate_count(), 0);
}

#[test]
fn eviction_receipt_evidence_stays_memory_residency_only() {
    let mut table = resident_frame_table(1, 1);
    admit_payload_frame(&mut table, 7, 2, b"evict");
    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let receipt = table.record_eviction(plan).unwrap();
    let evidence = EvictionProtectionEvidenceReport::from_receipt(receipt).unwrap();

    assert_eq!(
        evidence.row(),
        EvictionProtectionEvidenceRow::EvictionReceiptObserved
    );
    assert_eq!(evidence.counters().receipt_count(), 1);
    assert_eq!(table.counters().resident_bytes().as_bytes(), 0);
    assert!(!receipt.proves_durability());
}
