use super::admission_test_support::{complete_ledger_from_rows, split_request_subject};
use super::{
    deny_query_descriptor_digest_as_spatial_evidence_lookup_authority,
    SpatialEvidenceLookupDenialKind, SpatialEvidenceLookupExpectation,
    SpatialGeometryEvidenceTouchRequest,
};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn lookup_boundary_denies_wrong_boolean_stage_identity_index_support_and_query_digest() {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("split receipt should admit");

    let wrong_stage = authority
        .spatial_evidence_lookup_matching(
            &subject.complete,
            SpatialEvidenceLookupExpectation::from_authority(&authority)
                .with_boolean_stage(BooleanEvidenceStageKind::EventLedger),
        )
        .expect_err("wrong expected boolean stage must deny before lookup construction");
    assert_eq!(
        wrong_stage.kind(),
        SpatialEvidenceLookupDenialKind::WrongBooleanStage
    );

    let wrong_identity = authority
        .spatial_evidence_lookup_matching(
            &subject.complete,
            SpatialEvidenceLookupExpectation::from_authority(&authority)
                .with_evidence_identity("foreign evidence identity"),
        )
        .expect_err("wrong expected evidence identity must deny before lookup construction");
    assert_eq!(
        wrong_identity.kind(),
        SpatialEvidenceLookupDenialKind::WrongEvidenceIdentity
    );

    let wrong_index = authority
        .spatial_evidence_lookup_matching(
            &subject.complete,
            SpatialEvidenceLookupExpectation::from_authority(&authority)
                .with_stage_index_identity("foreign stage index"),
        )
        .expect_err("wrong expected stage-index identity must deny before lookup construction");
    assert_eq!(
        wrong_index.kind(),
        SpatialEvidenceLookupDenialKind::WrongStageIndexIdentity
    );

    let unsupported_authority = super::authority::admit_spatial_geometry_evidence_touch_authority(
        authority.boolean_stage(),
        authority.evidence_stage(),
        authority.evidence_identity().to_string(),
        WorkloadEvidenceSupport::Unsupported,
        authority.evidence_counters(),
        authority.lookup_counters(),
        authority.stage_index_identity().to_string(),
        authority.stage_link_set_identity().to_string(),
    );
    let unsupported = unsupported_authority
        .spatial_evidence_lookup(&subject.complete)
        .expect_err("unsupported support posture must not construct lookup product");
    assert_eq!(
        unsupported.kind(),
        SpatialEvidenceLookupDenialKind::UnsupportedSupportPosture
    );

    let query_substitution = deny_query_descriptor_digest_as_spatial_evidence_lookup_authority(
        "query-descriptor-digest",
    );
    assert_eq!(
        query_substitution.kind(),
        SpatialEvidenceLookupDenialKind::QueryDescriptorDigestSubstitution
    );
}

#[test]
fn lookup_boundary_denies_ledger_identity_substitution() {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("split receipt should admit");
    let mut rows = subject.complete.rows().to_vec();
    rows.push(WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::BooleanClassify,
        "unrelated boolean classify evidence",
        WorkloadEvidenceStageCounters::boolean_classify(),
    ));
    let substituted_ledger = complete_ledger_from_rows(rows);

    let denial = authority
        .spatial_evidence_lookup(&substituted_ledger)
        .expect_err("foreign stage-index identity must deny");

    assert_eq!(
        denial.kind(),
        SpatialEvidenceLookupDenialKind::WrongStageIndexIdentity
    );
}
