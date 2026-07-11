use crate::retention_reclaim::test_support::{
    hold_counter_for_kind, live_read_hold_admission, mismatched_abandoned_resume_barrier_admission,
    mismatched_scope_admission, plan, reclaim_fixture, retention_hold_admission,
};
use crate::{
    BlobReclaimResidueKind, BlobRetentionHold, BlobRetentionHoldKind, BlobRetentionReclaimDenial,
    BlobRetentionReclaimRequest,
};

#[test]
fn repeated_reclaim_planning_converges_to_same_permit_residue_and_counters() {
    let case = "phase15-equivalence";
    let admission = reclaim_fixture(case, 1);
    let request = BlobRetentionReclaimRequest::for_admission(admission);

    let first = plan(request.clone());
    let second = plan(request);

    assert_eq!(first, second);
    let permit = first.into_permit();
    assert_eq!(permit.counters().orphan_candidates(), 1);
    assert_eq!(permit.counters().reclaim_permits(), 1);
    assert_eq!(permit.counters().reclaimed_chunks(), 1);
    assert_eq!(permit.counters().residue_localizations(), 1);
    assert_eq!(
        permit.residue_report().kind(),
        BlobReclaimResidueKind::FailedReclaimBytes
    );
    assert!(!permit.residue_report().can_satisfy_blob_content());
    assert!(!permit.residue_report().can_satisfy_reachability());
}

#[test]
fn missing_s6_reclaim_posture_denies_before_permit() {
    let BlobRetentionReclaimDenial::MissingS6ReclaimPosture { counters } =
        crate::BlobRetentionReclaimAdmissionAuthority::store_owned()
            .deny_missing_s6_reclaim_posture()
    else {
        panic!("missing S.6 posture must deny");
    };

    assert_eq!(counters.reclaim_policy_evidence_denials(), 1);
    assert_eq!(counters.reclaim_permits(), 0);
}

#[test]
fn mismatched_s6_reclaim_scope_denies_before_permit() {
    let Err(BlobRetentionReclaimDenial::S6ReclaimPostureScopeMismatch { counters }) =
        mismatched_scope_admission("phase15-wrong-s6", 3)
    else {
        panic!("wrong S.6 posture scope must deny");
    };

    assert_eq!(counters.reclaim_policy_evidence_denials(), 1);
    assert_eq!(counters.reclaim_permits(), 0);
}

#[test]
fn mismatched_abandoned_resume_barrier_denies_before_permit() {
    let Err(BlobRetentionReclaimDenial::ReclaimCandidateIdentityMismatch { counters }) =
        mismatched_abandoned_resume_barrier_admission("phase15-wrong-region", 6, 77)
    else {
        panic!("wrong abandoned resume barrier identity must deny");
    };

    assert_eq!(counters.identity_mismatch_denials(), 1);
    assert_eq!(counters.reclaim_permits(), 0);
}

#[test]
fn residue_from_abandoned_resume_is_localized_not_content() {
    let case = "phase15-residue";
    let admission = reclaim_fixture(case, 4);
    let request =
        BlobRetentionReclaimRequest::for_admission(admission).with_abandoned_resume_residue();

    let permit = plan(request).into_permit();

    assert_eq!(
        permit.residue_report().kind(),
        BlobReclaimResidueKind::AbandonedResumeSessionBytes
    );
    assert_eq!(
        permit.residue_report().counters().residue_localizations(),
        1
    );
}

#[test]
fn live_read_plan_hold_blocks_admission_before_planning() {
    let Err(BlobRetentionReclaimDenial::ReclaimBlockedByRetentionHold { kind, counters }) =
        live_read_hold_admission("phase15-live-read-hold", 5)
    else {
        panic!("live read-plan hold must deny before a reclaim request can be planned");
    };

    assert_eq!(kind, BlobRetentionHoldKind::ReadPlan);
    assert_eq!(counters.read_plan_hold_denials(), 1);
    assert_eq!(counters.reclaim_permits(), 0);
}

#[test]
fn every_retention_hold_kind_independently_blocks_reclaim() {
    let cases = [
        (
            BlobRetentionHold::generation("generation"),
            BlobRetentionHoldKind::Generation,
        ),
        (
            BlobRetentionHold::time_window("time"),
            BlobRetentionHoldKind::TimeWindow,
        ),
        (
            BlobRetentionHold::export("export"),
            BlobRetentionHoldKind::Export,
        ),
        (
            BlobRetentionHold::capsule("capsule"),
            BlobRetentionHoldKind::Capsule,
        ),
        (
            BlobRetentionHold::quarantine("quarantine"),
            BlobRetentionHoldKind::Quarantine,
        ),
        (
            BlobRetentionHold::read_plan("read"),
            BlobRetentionHoldKind::ReadPlan,
        ),
        (
            BlobRetentionHold::checkpoint("checkpoint"),
            BlobRetentionHoldKind::Checkpoint,
        ),
        (
            BlobRetentionHold::tenant_custody("tenant"),
            BlobRetentionHoldKind::TenantCustody,
        ),
        (
            BlobRetentionHold::resume_session("resume"),
            BlobRetentionHoldKind::ResumeSession,
        ),
        (
            BlobRetentionHold::placement_move("placement"),
            BlobRetentionHoldKind::PlacementMove,
        ),
        (
            BlobRetentionHold::backup("backup"),
            BlobRetentionHoldKind::Backup,
        ),
    ];

    for (index, (hold, expected)) in cases.into_iter().enumerate() {
        let denial = retention_hold_admission("phase15-retention-hold", index as u16 + 10, hold)
            .expect_err("retention hold must deny ordinary reclaim admission");

        let BlobRetentionReclaimDenial::ReclaimBlockedByRetentionHold { kind, counters } = denial
        else {
            panic!("retention hold {expected:?} must deny reclaim");
        };

        assert_eq!(kind, expected);
        assert_eq!(hold_counter_for_kind(counters, expected), 1);
        assert_eq!(counters.reclaim_permits(), 0);
    }
}

#[test]
fn weak_representations_return_denials_not_authority() {
    let residue = crate::reject_backend_residue_as_retention_reclaim_authority();
    let copied_receipt = crate::reject_copied_receipt_as_retention_reclaim_authority();
    let copied_counter = crate::reject_copied_counter_as_retention_reclaim_authority();
    let projection = crate::reject_terminal_projection_as_retention_reclaim_authority();
    let policy_evidence_only = crate::reject_reclaim_policy_evidence_as_retention_reclaim_authority();

    assert!(matches!(
        residue,
        BlobRetentionReclaimDenial::BackendResidueRejected { .. }
    ));
    assert!(matches!(
        copied_receipt,
        BlobRetentionReclaimDenial::CopiedReceiptRejected { .. }
    ));
    assert!(matches!(
        copied_counter,
        BlobRetentionReclaimDenial::CopiedCounterRejected { .. }
    ));
    assert!(matches!(
        projection,
        BlobRetentionReclaimDenial::TerminalProjectionRejected { .. }
    ));
    assert!(matches!(
        policy_evidence_only,
        BlobRetentionReclaimDenial::S6HandoffAloneRejected { .. }
    ));
}
