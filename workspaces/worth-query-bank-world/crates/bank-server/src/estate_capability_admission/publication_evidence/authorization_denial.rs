use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryEvidenceCloseoutDisposition,
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceLocality, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileAttachmentTargetKind,
};
use worth_query_host::facade::{
    primary_graph::{
        WorthQueryApplicationAuthorizationExplanationCause, WorthQueryOperationAuthorizationDenial,
    },
    publication::domain_computation::{
        publish_application_authorization_denial,
        WorthQueryApplicationAuthorizationPublicationProfile,
        WorthQueryPublishedApplicationAuthorizationDenial,
    },
};

use super::profile::publication_profile;

pub(in crate::estate_capability_admission) struct ExpectedAuthorizationDenialPublication {
    pub(in crate::estate_capability_admission) cause:
        WorthQueryApplicationAuthorizationExplanationCause,
    pub(in crate::estate_capability_admission) code: &'static str,
}

pub(in crate::estate_capability_admission) fn assert_authorization_denial_publication(
    denial: &WorthQueryOperationAuthorizationDenial,
    expected: ExpectedAuthorizationDenialPublication,
) {
    let profile = publication_profile();
    let published = publish_application_authorization_denial(
        denial,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();

    assert_retained_denial(&published, denial, expected.cause);
    assert_boundary_profile(&published, profile);
    assert_denial_diagnostic(&published, expected.code);
    assert_exact_provenance(&published, denial);
    assert_denied_and_publication_receipts(&published);
}

fn assert_retained_denial(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    denial: &WorthQueryOperationAuthorizationDenial,
    expected_cause: WorthQueryApplicationAuthorizationExplanationCause,
) {
    assert_eq!(published.artifact().denial(), denial);
    assert_eq!(published.artifact().cause(), expected_cause);
}

fn assert_boundary_profile(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    expected_profile: worth_foundational::facade::FoundationalProfileSet,
) {
    assert_eq!(
        published.boundary_category(),
        FoundationalBoundaryArtifactCategory::Artifact
    );
    assert_eq!(
        published.boundary().payload().target_kind(),
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    );
    let progression = published.boundary().payload().profile();
    assert_eq!(progression.requested(), &expected_profile);
    assert_eq!(progression.admitted(), &expected_profile);
    assert_eq!(progression.materialized(), &expected_profile);
}

fn assert_denial_diagnostic(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    expected_code: &str,
) {
    assert_eq!(
        published.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Denied
    );
    assert_eq!(
        published.explanation().rows()[0].code().as_str(),
        expected_code
    );
    let FoundationalDiagnosticRow::Decision(row) = &published.explanation().rows()[0] else {
        panic!("authorization denial must publish one decision row");
    };
    assert_eq!(
        row.denial_class(),
        Some(FoundationalDiagnosticDenialClass::PolicyDenied)
    );
    assert_eq!(
        row.locality_claim(),
        FoundationalDiagnosticLocalityClaim::ExactSubject
    );
    assert_eq!(
        row.widened_fallout_posture(),
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened
    );
}

fn assert_exact_provenance(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    denial: &WorthQueryOperationAuthorizationDenial,
) {
    assert_eq!(
        published.provenance().locality(),
        FoundationalBoundaryEvidenceLocality::Current
    );
    assert_eq!(
        published.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    assert_eq!(
        published
            .provenance()
            .source_basis()
            .boundary_artifact_locator()
            .unwrap()
            .artifact_id()
            .get(),
        denial.identity().unwrap().get()
    );
}

fn assert_denied_and_publication_receipts(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
) {
    let closeout = published.denied_closeout_receipt();
    assert_eq!(
        closeout.closeout_disposition(),
        Some(FoundationalBoundaryEvidenceCloseoutDisposition::Denied)
    );
    assert_eq!(
        closeout.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::NotExecuted
    );
    assert_eq!(
        closeout.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Closeout
    );
    assert!(!closeout.did_execute());

    let publication = published.publication_receipt();
    assert_eq!(
        publication.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    assert_eq!(
        publication.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
    assert!(publication.did_execute());
}
