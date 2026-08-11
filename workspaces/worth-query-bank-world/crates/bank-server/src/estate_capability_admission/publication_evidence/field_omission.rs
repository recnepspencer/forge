use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryEvidenceExecutionPosture,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
    FoundationalBoundaryEvidenceReceiptKind, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileAttachmentTargetKind,
    FoundationalProfileSet,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_field_omission, WorthQueryApplicationAuthorizationPublicationProfile,
    WorthQueryPublishedApplicationDisclosure, WorthQueryPublishedApplicationFieldOmission,
};

use super::profile::publication_profile;

pub(in crate::estate_capability_admission) fn assert_omission_noninterference(
    first: &WorthQueryPublishedApplicationFieldOmission,
    second: &WorthQueryPublishedApplicationFieldOmission,
    expected_profile: FoundationalProfileSet,
) {
    assert_eq!(first.boundary_category(), second.boundary_category());
    assert_boundary_profile(first, expected_profile);
    assert_boundary_profile(second, expected_profile);
    assert_eq!(
        first.explanation().outcome_kind(),
        second.explanation().outcome_kind()
    );
    assert_eq!(
        first.explanation().rows()[0].code(),
        second.explanation().rows()[0].code()
    );
    assert_provenance_and_receipt(first);
    assert_provenance_and_receipt(second);
    assert_eq!(first.artifact(), second.artifact());
    assert_eq!(first.boundary(), second.boundary());
    assert_eq!(first.explanation(), second.explanation());
    assert_eq!(first.provenance(), second.provenance());
    assert_eq!(first.publication_receipt(), second.publication_receipt());
    assert_eq!(source_locator(first), source_locator(second));
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
}

pub(in crate::estate_capability_admission) fn assert_field_omission_publication(
    disclosure: &WorthQueryPublishedApplicationDisclosure,
) {
    let profile = publication_profile();
    let omission = publish_application_field_omission(
        disclosure,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();

    assert_eq!(omission.artifact().disclosure(), disclosure);
    assert_boundary_profile(&omission, profile);
    assert_omission_diagnostic(&omission);
    assert_provenance_and_receipt(&omission);
}

fn assert_boundary_profile(
    omission: &WorthQueryPublishedApplicationFieldOmission,
    expected_profile: FoundationalProfileSet,
) {
    assert_eq!(
        omission.boundary_category(),
        FoundationalBoundaryArtifactCategory::Artifact
    );
    assert_eq!(
        omission.boundary().payload().target_kind(),
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    );
    let progression = omission.boundary().payload().profile();
    assert_eq!(progression.requested(), &expected_profile);
    assert_eq!(progression.admitted(), &expected_profile);
    assert_eq!(progression.materialized(), &expected_profile);
}

fn assert_omission_diagnostic(omission: &WorthQueryPublishedApplicationFieldOmission) {
    assert_eq!(
        omission.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Partial
    );
    let FoundationalDiagnosticRow::Decision(row) = &omission.explanation().rows()[0] else {
        panic!("field omission must publish one decision row");
    };
    assert_eq!(row.denial_class(), None);
    assert_eq!(
        row.locality_claim(),
        FoundationalDiagnosticLocalityClaim::ExactSubject
    );
    assert_eq!(
        row.widened_fallout_posture(),
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened
    );
}

fn assert_provenance_and_receipt(omission: &WorthQueryPublishedApplicationFieldOmission) {
    assert_eq!(
        omission.provenance().locality(),
        FoundationalBoundaryEvidenceLocality::Current
    );
    assert_eq!(
        omission.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    let receipt = omission.publication_receipt();
    assert_eq!(
        receipt.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
    assert_eq!(
        receipt.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    assert!(receipt.did_execute());
}

fn source_locator(
    omission: &WorthQueryPublishedApplicationFieldOmission,
) -> worth_foundational::facade::BoundaryArtifactLocator {
    omission
        .provenance()
        .source_basis()
        .boundary_artifact_locator()
        .unwrap()
}
