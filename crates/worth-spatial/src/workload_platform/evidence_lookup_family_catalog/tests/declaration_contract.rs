use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::super::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};

#[test]
fn overlap_family_declares_once_for_multiple_matching_stages() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let family = closeout
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("overlap family exists");

    assert!(family
        .stage_applicability()
        .declares_multiple_matching_stages());
    assert_eq!(
        family.stage_applicability().stage_receipt_family_identity(),
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane()
    );
}

#[test]
fn stage_selection_routes_matching_receipt_family_without_stage_edits() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let shared_plane = closeout.families_for_stage(
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
    );
    let local_frame = closeout.families_for_stage(
        WorkloadEvidenceStage::BooleanLocalFrameSelection,
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
    );
    let wrong_receipt_family = closeout.families_for_stage(
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
        &EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
    );

    assert_eq!(shared_plane.family_count(), 1);
    assert_eq!(
        shared_plane.family_identities(),
        &["spatial-touch.boolean.overlap-evidence.v1".to_string()]
    );
    assert_eq!(
        local_frame.family_identities(),
        shared_plane.family_identities()
    );
    assert_eq!(wrong_receipt_family.family_count(), 0);
    assert_eq!(shared_plane.counters().candidate_family_count(), 3);
    assert_eq!(shared_plane.counters().receipt_family_match_count(), 1);
    assert_eq!(shared_plane.counters().stage_match_count(), 1);
}
