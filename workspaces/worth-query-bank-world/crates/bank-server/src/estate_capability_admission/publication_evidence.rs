use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryArtifactCategory,
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalDiagnosticOutcomeKind,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use worth_query_host::facade::{
    primary_graph::WorthQueryMandatoryReview,
    publication::domain_computation::{
        publish_mandatory_review, WorthQueryApplicationAuthorizationPublicationProfile,
        WorthQueryPublishedApplicationAuthorization,
        WorthQueryPublishedApplicationAuthorizationKind,
    },
};

pub(super) fn publication_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .unwrap()
}

pub(super) fn assert_review_required_publication_lineage(
    requested: &WorthQueryPublishedApplicationAuthorization,
    approved: &WorthQueryPublishedApplicationAuthorization,
    mandatory: &WorthQueryMandatoryReview,
) {
    let review_required = publish_mandatory_review(
        mandatory,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(publication_profile()),
    )
    .unwrap();
    assert_eq!(
        review_required.kind(),
        WorthQueryPublishedApplicationAuthorizationKind::RevokedReviewRequired
    );
    assert_eq!(
        review_required.boundary_category(),
        FoundationalBoundaryArtifactCategory::Receipt
    );
    assert_eq!(
        review_required.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Accepted
    );
    assert_eq!(
        review_required.explanation().rows()[0].code().as_str(),
        "worth.query.elevation.revoked.review-required"
    );
    assert_eq!(
        review_required.publication_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    assert_distinct_transition_locators(requested, approved, &review_required);
    let review_locator = boundary_locator(&review_required);
    assert_eq!(
        review_locator.artifact_id().get(),
        mandatory
            .close_commit_receipt()
            .outcome_identity()
            .unwrap()
            .get()
    );
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
