use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileAttachmentTargetKind,
    FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord,
    FoundationalProfileProgressionDenial, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use super::profile::admit_profile;
use super::*;
use crate::application_authorization::denial_explanation::materialize_denial_explanation;
use crate::application_authorization::field_omission_explanation::materialize_field_omission_explanation;
use worth_query_execution::facade::primary_graph::WorthQueryApplicationAuthorizationExplanationCause;

#[test]
fn exact_query_transition_lowers_into_foundational_publication_material() {
    let profile = profile(DiagnosticRichnessProfile::Standard);
    let identity = WorthQueryApplicationAuthorizationBoundaryIdentity {
        locator: BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(17),
            BoundaryArtifactField::Payload,
        ),
    };

    let lowered = lower_boundary_material(
        WorthQueryPublishedApplicationAuthorizationKind::ExpiredReviewRequired,
        identity,
        3,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();

    assert_eq!(
        boundary_receipt_category_of(lowered.boundary.payload().payload()),
        FoundationalBoundaryArtifactCategory::Receipt
    );
    assert_eq!(
        lowered.boundary.payload().payload().attested_effect_count(),
        3
    );
    assert_eq!(
        lowered.boundary.payload().target_kind(),
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    );
    assert_eq!(
        lowered.explanation.rows()[0].code().as_str(),
        "worth.query.elevation.expired.review-required"
    );
    assert_eq!(
        lowered.provenance.freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    assert_eq!(
        lowered.publication_receipt.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
}

#[test]
fn closed_publication_taxonomy_preserves_every_exact_denial_family() {
    use FoundationalDiagnosticOutcomeKind::{Denied, Mismatch, Violation};
    use WorthQueryApplicationAuthorizationExplanationCause as Cause;

    let identity = publication_identity(21);
    for (cause, code, outcome) in [
        (
            Cause::MissingCapability,
            "worth.query.authorization.missing-capability",
            Denied,
        ),
        (
            Cause::ExplicitPolicyDenial,
            "worth.query.authorization.explicit-policy-denial",
            Denied,
        ),
        (
            Cause::ScopeMismatch,
            "worth.query.authorization.scope-mismatch",
            Mismatch,
        ),
        (
            Cause::PurposeMismatch,
            "worth.query.authorization.purpose-mismatch",
            Mismatch,
        ),
        (
            Cause::Conflict,
            "worth.query.authorization.conflict",
            Violation,
        ),
        (
            Cause::SeparationOfDuty,
            "worth.query.authorization.separation-of-duty",
            Violation,
        ),
        (
            Cause::ElevationRequired,
            "worth.query.authorization.elevation-required",
            Denied,
        ),
        (
            Cause::ElevationDenied,
            "worth.query.authorization.elevation-denied",
            Denied,
        ),
        (
            Cause::ElevationExpired,
            "worth.query.authorization.elevation-expired",
            Denied,
        ),
    ] {
        let explanation = materialize_denial_explanation(cause, identity, exact_profile()).unwrap();

        assert_eq!(explanation.outcome_kind(), outcome, "{cause:?}");
        assert_eq!(explanation.rows().len(), 1, "{cause:?}");
        assert_eq!(explanation.rows()[0].code().as_str(), code, "{cause:?}");
        assert_decision_posture(
            &explanation.rows()[0],
            Some(FoundationalDiagnosticDenialClass::PolicyDenied),
        );
    }
}

#[test]
fn successful_governed_outcomes_remain_distinct_from_denials() {
    let identity = publication_identity(22);
    let omission = materialize_field_omission_explanation(identity, exact_profile()).unwrap();
    let review_required = materialize_explanation(
        WorthQueryPublishedApplicationAuthorizationKind::RevokedReviewRequired,
        identity,
        exact_profile(),
    )
    .unwrap();

    assert_eq!(
        omission.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Partial
    );
    assert_eq!(
        omission.rows()[0].code().as_str(),
        "worth.query.disclosure.field-omission"
    );
    assert_decision_posture(&omission.rows()[0], None);
    assert_eq!(
        review_required.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Accepted
    );
    assert_eq!(
        review_required.rows()[0].code().as_str(),
        "worth.query.elevation.revoked.review-required"
    );
    assert_decision_posture(&review_required.rows()[0], None);
}

fn assert_decision_posture(
    row: &FoundationalDiagnosticRow,
    denial_class: Option<FoundationalDiagnosticDenialClass>,
) {
    let FoundationalDiagnosticRow::Decision(row) = row else {
        panic!("authorization publication must materialize one decision row");
    };
    assert_eq!(row.denial_class(), denial_class);
    assert_eq!(
        row.locality_claim(),
        FoundationalDiagnosticLocalityClaim::ExactSubject
    );
    assert_eq!(
        row.widened_fallout_posture(),
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened
    );
}

#[test]
fn profile_widening_is_rejected_before_publication_material_exists() {
    let requested = profile(DiagnosticRichnessProfile::Standard);
    let widened = profile(DiagnosticRichnessProfile::Forensic);
    let profile = WorthQueryApplicationAuthorizationPublicationProfile::with_progression(
        requested,
        WorthQueryApplicationAuthorizationProfileStage::new(
            widened,
            Some(FoundationalProfileNarrowingRecord::new(
                FoundationalProfileNarrowingKind::RichnessReduced,
                "test claims narrowing",
            )),
        ),
        WorthQueryApplicationAuthorizationProfileStage::new(widened, None),
    );

    assert_eq!(
        admit_profile(profile),
        Err(
            WorthQueryApplicationAuthorizationPublicationDenial::ProfileAdmission(
                FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayOnlyNarrow,
            )
        )
    );
}

#[test]
fn explicit_profile_progression_can_only_reduce_one_descriptive_family_per_stage() {
    let requested = profile(DiagnosticRichnessProfile::Forensic);
    let admitted = profile(DiagnosticRichnessProfile::Standard);
    let materialized = profile(DiagnosticRichnessProfile::OperationalMinimal);
    let profile = WorthQueryApplicationAuthorizationPublicationProfile::with_progression(
        requested,
        WorthQueryApplicationAuthorizationProfileStage::new(
            admitted,
            Some(FoundationalProfileNarrowingRecord::new(
                FoundationalProfileNarrowingKind::RichnessReduced,
                "consumer requested standard diagnostic publication",
            )),
        ),
        WorthQueryApplicationAuthorizationProfileStage::new(
            materialized,
            Some(FoundationalProfileNarrowingRecord::new(
                FoundationalProfileNarrowingKind::RichnessReduced,
                "delivery retained the operational diagnostic row only",
            )),
        ),
    );

    let boundary = profile::profile_boundary(
        WorthQueryPublishedApplicationAuthorizationKind::RevokedElevationReviewed,
        0,
        profile,
    )
    .unwrap();
    let progression = boundary.payload().profile();

    assert_eq!(
        progression.requested().diagnostic_richness(),
        DiagnosticRichnessProfile::Forensic
    );
    assert_eq!(
        progression.admitted().diagnostic_richness(),
        DiagnosticRichnessProfile::Standard
    );
    assert_eq!(
        progression.materialized().diagnostic_richness(),
        DiagnosticRichnessProfile::OperationalMinimal
    );
}

#[test]
fn publication_identity_depends_only_on_closed_axes_and_profile() {
    let standard = WorthQueryApplicationAuthorizationPublicationProfile::exact(exact_profile());
    let forensic = WorthQueryApplicationAuthorizationPublicationProfile::exact(profile(
        DiagnosticRichnessProfile::Forensic,
    ));
    let first = WorthQueryApplicationAuthorizationBoundaryIdentity::from_closed_publication(
        "closed-family",
        &["axis-a".into(), "2".into()],
        standard,
    );
    let same = WorthQueryApplicationAuthorizationBoundaryIdentity::from_closed_publication(
        "closed-family",
        &["axis-a".into(), "2".into()],
        standard,
    );
    let changed_axis = WorthQueryApplicationAuthorizationBoundaryIdentity::from_closed_publication(
        "closed-family",
        &["axis-b".into(), "2".into()],
        standard,
    );
    let changed_profile =
        WorthQueryApplicationAuthorizationBoundaryIdentity::from_closed_publication(
            "closed-family",
            &["axis-a".into(), "2".into()],
            forensic,
        );

    assert_eq!(first.artifact_id(), same.artifact_id());
    assert_ne!(first.artifact_id(), changed_axis.artifact_id());
    assert_ne!(first.artifact_id(), changed_profile.artifact_id());
}

fn profile(richness: DiagnosticRichnessProfile) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: richness,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .unwrap()
}

fn exact_profile() -> FoundationalProfileSet {
    profile(DiagnosticRichnessProfile::Standard)
}

fn publication_identity(id: u64) -> WorthQueryApplicationAuthorizationBoundaryIdentity {
    WorthQueryApplicationAuthorizationBoundaryIdentity {
        locator: BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(id),
            BoundaryArtifactField::Payload,
        ),
    }
}
