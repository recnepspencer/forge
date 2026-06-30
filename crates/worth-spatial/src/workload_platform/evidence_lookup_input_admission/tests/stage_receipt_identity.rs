use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity;

use super::super::{
    admit_evidence_lookup_input, EvidenceLookupInputAdmissionErrorKind,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupStageReceiptAdmission,
};
use super::fixtures::AdmissionSubject;

#[test]
fn lookup_input_requires_explicit_stage_receipt_identity() {
    let subject = AdmissionSubject::event_ledger();
    let request =
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(subject.authority());

    let denial = admit_evidence_lookup_input(subject.catalog(), request)
        .expect_err("admission cannot infer stage receipt identity from defaults");

    assert_eq!(
        denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::MissingStageReceiptIdentity
    );
    assert_eq!(denial.counters().raw_row_scan_count(), 0);
}

#[test]
fn wrong_stage_receipt_identity_denies_before_selection() {
    let subject = AdmissionSubject::projection_consumption();

    let wrong_stage_authority = AdmissionSubject::event_ledger();
    let stage_denial = admit_evidence_lookup_input(
        subject.catalog(),
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(subject.authority())
            .with_stage_receipt_identity(
                EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                    wrong_stage_authority.authority(),
                    EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
                ),
            ),
    )
    .expect_err("wrong stage denies before selection");

    assert_eq!(
        stage_denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::SpatialTouchStageMismatch
    );
    assert_eq!(stage_denial.counters().catalog_candidate_family_count(), 0);

    let other_projection_authority =
        AdmissionSubject::projection_consumption_with_identity("other-projection-receipt");
    let authority_mismatch_denial = admit_evidence_lookup_input(
        subject.catalog(),
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(subject.authority())
            .with_stage_receipt_identity(
            EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                other_projection_authority.authority(),
                EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
            ),
        ),
    )
    .expect_err("same-stage receipt from a different authority must deny");

    assert_eq!(
        authority_mismatch_denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::StageReceiptAuthorityMismatch
    );
    assert_eq!(
        authority_mismatch_denial
            .counters()
            .catalog_candidate_family_count(),
        0
    );

    let receipt_denial = admit_evidence_lookup_input(
        subject.catalog(),
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(subject.authority())
            .with_stage_receipt_identity(
                EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                    subject.authority(),
                    EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
                ),
            ),
    )
    .expect_err("wrong receipt family denies before lookup product construction");

    assert_eq!(
        receipt_denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::NoFamilyForStageReceiptIdentity
    );
    assert_eq!(
        receipt_denial.counters().catalog_candidate_family_count(),
        3
    );
}
