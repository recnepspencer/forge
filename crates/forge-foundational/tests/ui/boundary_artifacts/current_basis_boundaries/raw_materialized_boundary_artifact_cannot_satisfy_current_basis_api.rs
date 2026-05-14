use forge_foundational::{
    claim_derived_projection_boundary_surface, materialize_admitted_foundational_profile,
    request_foundational_profile_set, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

fn accepts_current_basis_artifact(
    _: &forge_foundational::CurrentBasisBoundaryArtifact<
        forge_foundational::FoundationalBoundaryArtifactSurface<Vec<u8>>,
    >,
) {
}

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
        _ => panic!("expected admitted profile"),
    };
    let materialized_profile = match materialize_admitted_foundational_profile(
        admitted,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(materialized) => *materialized.payload(),
        _ => panic!("expected materialized profile"),
    };
    let materialized = forge_foundational::materialize_descriptive_boundary_surface(
        claim_derived_projection_boundary_surface(FoundationalBoundaryArtifactSurface::new(
            vec![1_u8, 2, 3],
            2,
        )),
        FoundationalBoundaryMaterializationSource::CompatibilityLowered,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        materialized_profile,
    )
    .expect("materialized artifact");

    accepts_current_basis_artifact(&materialized);
}
