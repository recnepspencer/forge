use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionDenial,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};

use super::profile::admit_profile;
use super::*;

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
