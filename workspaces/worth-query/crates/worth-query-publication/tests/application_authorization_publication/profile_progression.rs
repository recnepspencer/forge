use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionDenial,
    FoundationalProfileSet, FoundationalProfileSetInput, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_query_publication::facade::domain_computation::{
    publish_application_authorization_denial, WorthQueryApplicationAuthorizationProfileStage,
    WorthQueryApplicationAuthorizationPublicationDenial,
    WorthQueryApplicationAuthorizationPublicationProfile,
};

use super::installed_composition::{real_denial, CompositionScenario};

#[test]
fn public_denial_publisher_retains_explicit_two_stage_profile_narrowing() {
    let denial = real_denial(CompositionScenario::MissingAuthorization);
    let requested = profile(DiagnosticRichnessProfile::Forensic);
    let admitted = profile(DiagnosticRichnessProfile::Standard);
    let materialized = profile(DiagnosticRichnessProfile::OperationalMinimal);
    let publication_profile =
        WorthQueryApplicationAuthorizationPublicationProfile::with_progression(
            requested,
            stage(admitted, "admission retained standard diagnostics"),
            stage(materialized, "delivery retained operational diagnostics"),
        );

    let published = publish_application_authorization_denial(&denial, publication_profile).unwrap();
    let progression = published.boundary().payload().profile();
    assert_eq!(progression.requested(), &requested);
    assert_eq!(progression.admitted(), &admitted);
    assert_eq!(progression.materialized(), &materialized);
    assert!(progression.requested_to_admitted_narrowing().is_some());
    assert!(progression.admitted_to_materialized_narrowing().is_some());
}

#[test]
fn public_denial_publisher_rejects_admission_and_materialization_widening_separately() {
    let denial = real_denial(CompositionScenario::MissingAuthorization);
    let standard = profile(DiagnosticRichnessProfile::Standard);
    let forensic = profile(DiagnosticRichnessProfile::Forensic);
    let claimed_narrowing = || {
        Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::RichnessReduced,
            "the hostile profile falsely claims narrowing",
        ))
    };

    let admission = WorthQueryApplicationAuthorizationPublicationProfile::with_progression(
        standard,
        WorthQueryApplicationAuthorizationProfileStage::new(forensic, claimed_narrowing()),
        WorthQueryApplicationAuthorizationProfileStage::new(forensic, None),
    );
    assert_eq!(
        publish_application_authorization_denial(&denial, admission).unwrap_err(),
        WorthQueryApplicationAuthorizationPublicationDenial::ProfileAdmission(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayOnlyNarrow,
        )
    );

    let materialization = WorthQueryApplicationAuthorizationPublicationProfile::with_progression(
        standard,
        WorthQueryApplicationAuthorizationProfileStage::new(standard, None),
        WorthQueryApplicationAuthorizationProfileStage::new(forensic, claimed_narrowing()),
    );
    assert_eq!(
        publish_application_authorization_denial(&denial, materialization).unwrap_err(),
        WorthQueryApplicationAuthorizationPublicationDenial::ProfileMaterialization(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayOnlyNarrow,
        )
    );
}

fn stage(
    profile: FoundationalProfileSet,
    reason: &'static str,
) -> WorthQueryApplicationAuthorizationProfileStage {
    WorthQueryApplicationAuthorizationProfileStage::new(
        profile,
        Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::RichnessReduced,
            reason,
        )),
    )
}

fn profile(richness: DiagnosticRichnessProfile) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: richness,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        execution_objective: ExecutionObjectiveProfile::Balanced,
        observation_activation: ObservationActivationProfile::Continuous,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .unwrap()
}
