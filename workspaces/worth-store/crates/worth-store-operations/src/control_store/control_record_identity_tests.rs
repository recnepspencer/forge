use worth_store_authority::StoreCurrentAuthorityIdentity;

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
