use forge_foundational::{
    claim_derived_projection_boundary_surface, plan_artifact_boundary_bundle,
    plan_descriptive_boundary_materialization, request_foundational_profile_set,
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundarySummarySurface, FoundationalProfileSet,
    FoundationalProfileSetInput, RetentionDeliveryProfile, SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

fn main() {
    let profile = FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::CertificationReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Durable,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("profile");
    let requested = request_foundational_profile_set(profile);
    let admitted = match forge_foundational::admit_requested_foundational_profile(
        requested,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("expected admitted profile, got {other:?}"),
    };
    let materialized = match forge_foundational::materialize_admitted_foundational_profile(
        admitted,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(materialized) => *materialized.payload(),
        other => panic!("expected materialized profile, got {other:?}"),
    };
    let summary = plan_descriptive_boundary_materialization(
        claim_derived_projection_boundary_surface(
            FoundationalBoundarySummarySurface::new("summary", 1).expect("summary"),
        ),
        forge_foundational::FoundationalBoundaryMaterializationSource::CompatibilityLowered,
        forge_foundational::FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        materialized,
    )
    .expect("summary plan");

    let _ = plan_artifact_boundary_bundle(summary);
}
