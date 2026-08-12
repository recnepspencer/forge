use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryEvidenceExecutionPosture,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
    FoundationalBoundaryEvidenceReceiptKind, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileAttachmentTargetKind,
};
use worth_query_host::facade::{
    primary_graph::WorthQueryMandatoryReview,
    publication::domain_computation::{
        publish_mandatory_review, WorthQueryApplicationAuthorizationPublicationProfile,
        WorthQueryPublishedApplicationAuthorization,
        WorthQueryPublishedApplicationAuthorizationKind,
    },
};

use super::profile::publication_profile;

pub(in crate::estate_capability_admission) fn assert_review_required_publication_lineage(
    requested: &WorthQueryPublishedApplicationAuthorization,
    approved: &WorthQueryPublishedApplicationAuthorization,
    mandatory: &WorthQueryMandatoryReview,
) {
    let profile = publication_profile();
    let review = publish_mandatory_review(
        mandatory,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();
    let repeated = publish_mandatory_review(
        mandatory,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();

    assert_review_boundary(&review, profile);
    assert_review_diagnostic(&review);
    assert_review_provenance_and_receipt(&review);
    assert_distinct_transition_locators(requested, approved, &review);
    assert_eq!(boundary_locator(&review), boundary_locator(&repeated));
    assert_ne!(boundary_locator(&review).artifact_id().get(), 0);
}

fn assert_review_boundary(
    review: &WorthQueryPublishedApplicationAuthorization,
    expected_profile: worth_foundational::facade::FoundationalProfileSet,
) {
    assert_eq!(
        review.kind(),
        WorthQueryPublishedApplicationAuthorizationKind::RevokedReviewRequired
    );
    assert_eq!(
        review.boundary_category(),
        FoundationalBoundaryArtifactCategory::Receipt
    );
    assert_eq!(
        review.boundary().payload().target_kind(),
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    );
    let progression = review.boundary().payload().profile();
    assert_eq!(progression.requested(), &expected_profile);
    assert_eq!(progression.admitted(), &expected_profile);
    assert_eq!(progression.materialized(), &expected_profile);
}

fn assert_review_diagnostic(review: &WorthQueryPublishedApplicationAuthorization) {
    assert_eq!(
        review.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Accepted
    );
    assert_eq!(
        review.explanation().rows()[0].code().as_str(),
        "worth.query.elevation.revoked.review-required"
    );
    let FoundationalDiagnosticRow::Decision(row) = &review.explanation().rows()[0] else {
        panic!("review-required publication must contain one decision row");
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

fn assert_review_provenance_and_receipt(review: &WorthQueryPublishedApplicationAuthorization) {
    assert_eq!(
        review.provenance().locality(),
        FoundationalBoundaryEvidenceLocality::Current
    );
    assert_eq!(
        review.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    let receipt = review.publication_receipt();
    assert_eq!(
        receipt.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    assert_eq!(
        receipt.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
    assert!(receipt.did_execute());
}

fn assert_distinct_transition_locators(
    requested: &WorthQueryPublishedApplicationAuthorization,
    approved: &WorthQueryPublishedApplicationAuthorization,
    review: &WorthQueryPublishedApplicationAuthorization,
) {
    let requested = boundary_locator(requested);
    let approved = boundary_locator(approved);
    let review = boundary_locator(review);
    assert_ne!(requested, approved);
    assert_ne!(requested, review);
    assert_ne!(approved, review);
}

fn boundary_locator(
    publication: &WorthQueryPublishedApplicationAuthorization,
) -> worth_foundational::facade::BoundaryArtifactLocator {
    publication
        .provenance()
        .source_basis()
        .boundary_artifact_locator()
        .unwrap()
}
