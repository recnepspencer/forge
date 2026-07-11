use forge_store_physical_backend::BackendCapabilityKind;

use super::support::{
    assert_scheduler_posture_denial, externally_guaranteed_witness, non_current_postures,
    platform_requirements, valid_security_scope,
};
use crate::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim,
};

#[test]
fn every_platform_claim_denies_all_non_current_postures() {
    for requirement in platform_requirements() {
        for posture in non_current_postures() {
            let witness = externally_guaranteed_witness(requirement.capability_kind(), posture);

            let denial = admit_backend_capability_for_scheduler_claim(&witness, requirement)
                .expect_err("scheduler must not consume non-current platform capability");

            assert_scheduler_posture_denial(denial, posture);
        }
    }
}

#[test]
fn secure_frame_claim_with_scope_denies_all_non_current_postures() {
    let security_scope = valid_security_scope();

    for posture in non_current_postures() {
        let witness = externally_guaranteed_witness(BackendCapabilityKind::SecureFrameIo, posture);

        let denial =
            admit_secure_frame_backend_capability_for_scheduler_claim(&witness, &security_scope)
                .expect_err("scheduler must not consume non-current secure-frame capability");

        assert_scheduler_posture_denial(denial, posture);
    }
}
