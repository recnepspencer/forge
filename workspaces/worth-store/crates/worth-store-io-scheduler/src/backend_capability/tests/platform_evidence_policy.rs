use worth_store_physical_backend::BackendCapabilitySupportPosture;

use super::support::{
    assert_evidence_denial, externally_guaranteed_witness, platform_requirements,
    weaker_than_external_evidence, witness_from_basis_and_posture,
};
use crate::admit_backend_capability_for_scheduler_claim;

#[test]
fn every_platform_claim_requires_scheduler_owned_evidence_policy() {
    for requirement in platform_requirements() {
        let witness = externally_guaranteed_witness(
            requirement.capability_kind(),
            BackendCapabilitySupportPosture::Supported,
        );

        let admission = admit_backend_capability_for_scheduler_claim(&witness, requirement)
            .expect("externally guaranteed platform claim should admit");

        assert_eq!(admission.requirement(), requirement);
        assert_eq!(admission.evidence_class(), requirement.required_evidence());
    }
}

#[test]
fn every_platform_claim_denies_weaker_evidence_laundering() {
    for requirement in platform_requirements() {
        for basis in weaker_than_external_evidence() {
            let witness = witness_from_basis_and_posture(
                requirement.capability_kind(),
                BackendCapabilitySupportPosture::Supported,
                basis,
            );

            let denial = admit_backend_capability_for_scheduler_claim(&witness, requirement)
                .expect_err("scheduler must not lower platform evidence policy");

            assert_evidence_denial(denial);
        }
    }
}
