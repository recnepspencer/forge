use forge_store_physical_backend::{BackendCapabilityKind, BackendCapabilitySupportPosture};
use forge_store_security::{
    admitted_wrong_s6_io_qos_security_scope_for_test,
};

use super::support::{
    assert_evidence_denial, externally_guaranteed_witness, valid_security_scope,
    weaker_than_external_evidence, witness_from_basis_and_posture,
};
use crate::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim, IoSchedulerBackendCapabilityDenial,
    IoSchedulerBackendCapabilityRequirement, IoSchedulerSecurityScopeAdmissionDenial,
};

#[test]
fn secure_frame_claim_requires_security_scope_admission() {
    let witness = externally_guaranteed_witness(
        BackendCapabilityKind::SecureFrameIo,
        BackendCapabilitySupportPosture::Supported,
    );

    let denial = admit_backend_capability_for_scheduler_claim(
        &witness,
        IoSchedulerBackendCapabilityRequirement::SecureFrameIo,
    )
    .expect_err("secure-frame admission must require S.5.1 security scope");

    assert_eq!(
        denial,
        IoSchedulerBackendCapabilityDenial::SecureFrameRequiresSecurityScope
    );
}

#[test]
fn secure_frame_claim_admits_through_s5_1_security_scope_handoff() {
    let security_scope = valid_security_scope();
    let witness = externally_guaranteed_witness(
        BackendCapabilityKind::SecureFrameIo,
        BackendCapabilitySupportPosture::Supported,
    );

    let admission =
        admit_secure_frame_backend_capability_for_scheduler_claim(&witness, &security_scope)
            .expect("secure-frame claim should admit with bound security scope");

    assert_eq!(
        admission.requirement(),
        IoSchedulerBackendCapabilityRequirement::SecureFrameIo
    );
    assert!(admission.security_scope_bound());
}

#[test]
fn secure_frame_claim_rejects_wrong_s5_1_scope_identity() {
    let security_scope = admitted_wrong_s6_io_qos_security_scope_for_test();
    let denial = crate::admit_security_scope_for_scheduler(&security_scope)
        .expect_err("wrong admitted security identity must not enter scheduler use");

    assert!(matches!(
        denial,
        IoSchedulerSecurityScopeAdmissionDenial::WrongKeyScope { .. }
    ));
}

#[test]
fn secure_frame_claim_with_scope_still_denies_weak_backend_evidence() {
    let security_scope = valid_security_scope();

    for basis in weaker_than_external_evidence() {
        let witness = witness_from_basis_and_posture(
            BackendCapabilityKind::SecureFrameIo,
            BackendCapabilitySupportPosture::Supported,
            basis,
        );

        let denial =
            admit_secure_frame_backend_capability_for_scheduler_claim(&witness, &security_scope)
                .expect_err("security scope cannot strengthen backend evidence");

        assert_evidence_denial(denial);
    }
}
