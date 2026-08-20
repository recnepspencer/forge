use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisSequence, CanonicalizationCost, CanonicalizationRuleVersion,
};
use crate::{
    admit_requested_foundational_profile, derive_foundational_profile_identity,
    foundational_profile_progression_authority, request_foundational_profile_set,
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileIdentity,
    FoundationalProfileSet, FoundationalProfileSetInput, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};

fn profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
        execution_objective: ExecutionObjectiveProfile::Balanced,
        observation_activation: ObservationActivationProfile::Continuous,
    })
    .expect("coherent profile")
}

#[test]
fn profile_identity_equality_ignores_canonicalization_cost_counters() {
    let version =
        CanonicalizationRuleVersion::new("m3.profile.identity.eq").expect("valid version");
    let profile = profile();
    let admitted = match admit_requested_foundational_profile(
        request_foundational_profile_set(profile),
        profile,
        None,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("expected admitted profile, got {outcome:?}"),
    };
    let identity = match derive_foundational_profile_identity(version, &admitted) {
        TransitionOutcome::Success(identity) => identity,
        outcome => panic!("expected profile identity, got {outcome:?}"),
    };
    let mutated_basis = CanonicalBasisSequence::new(
        identity.basis.version().clone(),
        CanonicalBasisDomain::Profile,
        identity.basis.entries().to_vec(),
        CanonicalizationCost::new(identity.basis.cost().entry_count(), 99, 0, 0),
    );
    let with_different_cost = FoundationalProfileIdentity {
        basis: mutated_basis,
        digest: identity.digest.clone(),
    };

    assert_eq!(identity, with_different_cost);
}
