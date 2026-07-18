use worth_store_authority::{
    RecoveryAuthorityAdmissionPolicy, RecoveryAuthorityAdmissionPosture,
    RecoveryAuthorityRegionPosture, StoreCurrentAuthorityIdentity,
};

use super::*;
use crate::{OperationalOperationId, OperationalTransitionId, OperationalWorkflowKind};

#[test]
fn stable_fingerprint_binds_the_complete_control_payload() {
    let authority = StoreCurrentAuthorityIdentity::from_persisted_fingerprint([7; 32]);
    let operation = OperationalOperationId::new("same-operation").unwrap();
    let transition = OperationalTransitionId::new("same-transition").unwrap();
    let backup = OperationalControlRecord::workflow_opened(
        authority,
        operation.clone(),
        transition.clone(),
        OperationalWorkflowKind::Backup,
    );
    let repair = OperationalControlRecord::workflow_opened(
        authority,
        operation,
        transition,
        OperationalWorkflowKind::Repair,
    );

    assert_ne!(backup.stable_fingerprint(), repair.stable_fingerprint());
}

#[test]
fn publication_binding_fingerprint_binds_authority_posture_and_policy() {
    let trusted = posture(1, 0);
    let residual = posture(1, 1);
    let strict = RecoveryAuthorityAdmissionPolicy::fully_trusted_only();
    let admitted = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        residual, [9; 32],
    )
    .unwrap();

    let trusted_binding = publication_binding(trusted, strict);
    let residual_binding = publication_binding(residual, admitted);

    assert_ne!(
        binding_fingerprint(&trusted_binding),
        binding_fingerprint(&residual_binding)
    );
}

#[test]
fn optional_execution_plan_identity_has_an_explicit_presence_tag() {
    let mut absent = Sha256::new();
    fingerprint_optional_identity(&mut absent, None);
    let mut present_zero = Sha256::new();
    fingerprint_optional_identity(&mut present_zero, Some([0; 32]));

    assert_ne!(
        <[u8; 32]>::from(absent.finalize()),
        <[u8; 32]>::from(present_zero.finalize())
    );
}

fn publication_binding(
    posture: RecoveryAuthorityAdmissionPosture,
    policy: RecoveryAuthorityAdmissionPolicy,
) -> RecoveryPublicationControlBinding {
    RecoveryPublicationControlBinding::from_persisted(
        1, [1; 32], [2; 32], [3; 32], [4; 32], [5; 32], [6; 32], posture, policy,
    )
}

fn binding_fingerprint(binding: &RecoveryPublicationControlBinding) -> [u8; 32] {
    let mut digest = Sha256::new();
    fingerprint_publication_binding(binding, &mut digest);
    digest.finalize().into()
}

fn posture(trusted: u64, unavailable: u64) -> RecoveryAuthorityAdmissionPosture {
    let region = |tag: u8, count: u64| {
        RecoveryAuthorityRegionPosture::observed(
            if count == 0 { [0; 32] } else { [tag; 32] },
            count,
        )
        .unwrap()
    };
    RecoveryAuthorityAdmissionPosture::from_independent_post_verification(
        [7; 32],
        [
            region(1, trusted),
            region(2, 0),
            region(3, 0),
            region(4, 0),
            region(5, unavailable),
        ],
    )
    .unwrap()
}
