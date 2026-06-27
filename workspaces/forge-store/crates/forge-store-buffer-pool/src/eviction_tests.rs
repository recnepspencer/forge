use crate::dirty_state_test_support::{admit_payload_frame, load_request, resident_frame_table};
use crate::{EvictionPressure, EvictionProtectionReason, ResidentFrameDenialKind};

#[test]
fn eviction_plan_scans_only_resident_frame_table_candidates() {
    let mut table = resident_frame_table(8192, 4, 1);
    let first = admit_payload_frame(&mut table, 7, 2, b"first");
    let second = admit_payload_frame(&mut table, 8, 3, b"second");

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let candidates = plan.candidate_set();

    assert_eq!(table.resident_frame_count(), 2);
    assert_eq!(candidates.resident_frames_scanned(), 2);
    assert_eq!(candidates.candidate_count(), 2);
    assert_eq!(candidates.selected_identity(), first.identity());
    assert!(!candidates.includes_protected_frames());
    assert_eq!(table.eviction_counters().resident_frame_scan_count(), 2);
    assert_eq!(table.eviction_counters().policy_rank_count(), 1);
    assert_ne!(first.identity(), second.identity());
}

#[test]
fn identical_resident_tables_produce_same_eviction_plan_identity() {
    let mut first_table = resident_frame_table(8192, 8, 1);
    let mut second_table = resident_frame_table(8192, 8, 1);
    let first_dirty = admit_payload_frame(&mut first_table, 7, 2, b"dirty");
    let second_dirty = admit_payload_frame(&mut second_table, 7, 2, b"dirty");
    let first_clean = admit_payload_frame(&mut first_table, 8, 3, b"clean");
    let second_clean = admit_payload_frame(&mut second_table, 8, 3, b"clean");
    first_table
        .mark_dirty(first_dirty.resident_frame_token())
        .unwrap();
    second_table
        .mark_dirty(second_dirty.resident_frame_token())
        .unwrap();

    let first_plan = first_table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let second_plan = second_table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();

    assert_eq!(first_clean.identity(), second_clean.identity());
    assert_eq!(
        first_plan.selected_identity(),
        second_plan.selected_identity()
    );
    assert_eq!(
        first_plan.candidate_set().resident_frames_scanned(),
        second_plan.candidate_set().resident_frames_scanned()
    );
    assert_eq!(
        first_plan.candidate_set().candidate_count(),
        second_plan.candidate_set().candidate_count()
    );
}

#[test]
fn eviction_scan_work_stays_bound_to_sparse_resident_set() {
    let mut table = resident_frame_table(8192, 64, 1);
    let first = admit_payload_frame(&mut table, 7, 2, b"first");
    let second = admit_payload_frame(&mut table, 8, 3, b"second");

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    assert_eq!(table.resident_frame_count(), 2);
    assert_eq!(plan.candidate_set().resident_frames_scanned(), 2);
    assert_eq!(plan.candidate_set().candidate_count(), 2);
    assert_eq!(table.eviction_counters().resident_frame_scan_count(), 2);
    assert_eq!(plan.candidate_set().selected_identity(), first.identity());

    table.record_eviction(plan).unwrap();
    let third = admit_payload_frame(&mut table, 9, 4, b"third");
    let next_plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();

    assert_eq!(table.resident_frame_count(), 2);
    assert_eq!(next_plan.candidate_set().resident_frames_scanned(), 2);
    assert_eq!(next_plan.candidate_set().candidate_count(), 2);
    assert_eq!(
        next_plan.candidate_set().selected_identity(),
        third.identity()
    );
    assert_ne!(second.identity(), third.identity());
}

#[test]
fn eviction_excludes_protected_frames_before_policy_ranking() {
    let mut table = resident_frame_table(8192, 6, 2);
    let pinned = admit_payload_frame(&mut table, 7, 2, b"pinned");
    let dirty = admit_payload_frame(&mut table, 8, 3, b"dirty");
    let verifier = admit_payload_frame(&mut table, 9, 4, b"verifier");
    let recovery = admit_payload_frame(&mut table, 10, 5, b"recovery");
    let streaming = admit_payload_frame(&mut table, 11, 6, b"streaming");
    let clean = admit_payload_frame(&mut table, 12, 7, b"clean");
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

    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let candidates = plan.candidate_set();
    let protected = candidates.protected_exclusions();

    assert_eq!(candidates.selected_identity(), clean.identity());
    assert_eq!(candidates.resident_frames_scanned(), 6);
    assert_eq!(candidates.candidate_count(), 1);
    assert_eq!(candidates.policy_rank_count(), 1);
    assert_eq!(protected.pinned_count(), 1);
    assert_eq!(protected.dirty_unpublished_count(), 1);
    assert_eq!(protected.verifier_protected_count(), 1);
    assert_eq!(protected.recovery_protected_count(), 1);
    assert_eq!(protected.streaming_protected_count(), 1);
    assert_eq!(table.eviction_counters().protected_exclusion_count(), 5);
}

#[test]
fn all_protected_resident_set_denies_with_precise_reasons() {
    let mut table = resident_frame_table(8192, 5, 1);
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

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::AllEvictionCandidatesProtected
    );
    assert!(protected.contains(EvictionProtectionReason::Pinned));
    assert!(protected.contains(EvictionProtectionReason::DirtyUnpublished));
    assert!(protected.contains(EvictionProtectionReason::VerifierProtected));
    assert!(protected.contains(EvictionProtectionReason::RecoveryProtected));
    assert!(protected.contains(EvictionProtectionReason::StreamingProtected));
    assert_eq!(table.eviction_counters().all_protected_denial_count(), 1);
    assert_eq!(table.eviction_counters().policy_rank_count(), 0);
    assert_eq!(table.eviction_counters().resident_frame_scan_count(), 5);
}

#[test]
fn eviction_receipt_releases_resident_bytes_and_stales_reused_slot_token() {
    let mut table = resident_frame_table(8192, 1, 1);
    let first = admit_payload_frame(&mut table, 7, 2, b"first");
    let first_token = first.resident_frame_token();
    let first_size = first.request().frame_size().as_bytes();
    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();
    let receipt = table.record_eviction(plan).unwrap();

    assert_eq!(receipt.identity(), first.identity());
    assert_eq!(receipt.evicted_frame_count(), 1);
    assert_eq!(receipt.released_resident_bytes().as_bytes(), first_size);
    assert_eq!(receipt.counters().receipt_count(), 1);
    assert_eq!(table.counters().resident_bytes().as_bytes(), 0);
    assert!(!receipt.proves_durability());

    let second = table.admit_frame(load_request(8, 3, b"second")).unwrap();
    let denial = table.resident_frame(first_token).unwrap_err();
    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::StaleResidentGeneration
    );
    assert_ne!(first.identity(), second.identity());
}

#[test]
fn eviction_execution_revalidates_candidate_protection_after_planning() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"stale-plan");
    let plan = table
        .plan_eviction(EvictionPressure::for_resident_frames(1).unwrap())
        .unwrap();

    table
        .protect_frame_for_verifier(admission.resident_frame_token())
        .unwrap();
    let denial = table.record_eviction(plan).unwrap_err();
    let protected = denial.protected_frame_denial().unwrap().reasons();

    assert_eq!(denial.kind(), ResidentFrameDenialKind::EvictionPlanStale);
    assert!(protected.contains(EvictionProtectionReason::VerifierProtected));
    assert_eq!(table.eviction_counters().stale_plan_denial_count(), 1);
    assert_eq!(table.eviction_counters().receipt_count(), 0);
    assert_eq!(
        table.counters().resident_bytes().as_bytes(),
        admission.request().frame_size().as_bytes()
    );
}
