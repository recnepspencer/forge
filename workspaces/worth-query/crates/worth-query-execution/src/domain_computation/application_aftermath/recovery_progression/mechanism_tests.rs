//! Handle-owned mechanism and authority-axis evidence.

use worth_foundational::facade::CanonicalDigestId;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryInstalledAftermathContract,
};
use worth_relational::facade::history::BranchId;

use super::{
    compensate_recovery_handle, reconcile_recovery_handle, WorthQueryRecoveryEffectAuthority,
};
use crate::domain_computation::application_aftermath::aftermath_schema_fixture as fixture;
use crate::domain_computation::application_aftermath::recovery_handle::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleBindingAxisProbe, WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::WorthQueryApplicationIdempotencyBinding;

fn probe_handle(aftermath: WorthQueryInstalledAftermathContract) -> WorthQueryRecoveryHandle {
    let schema = ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    );
    let principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
        42,
        schema,
        "mechanism-authority",
        1,
        10,
        1,
        2,
        20,
        1,
    );
    let binding =
        WorthQueryRecoveryHandleBinding::axis_probe(WorthQueryRecoveryHandleBindingAxisProbe {
            runtime_instance_id: 7,
            schema_identity: [0x33; 32],
            branch: BranchId("2".to_owned()),
            application_binding_generation: 3,
            installed_operation: [0x44; 32],
            attempt_commit_id: 10,
            mutation_work: None,
            retained_preimage: None,
            retained_governed_input_identity: None,
            principal_scope,
            idempotency: WorthQueryApplicationIdempotencyBinding::new([0x55; 32], [0x56; 32]),
            provider_posture: None,
            dispatch_outbox: None,
            dispatch_outbox_record_ref: None,
            installed_aftermath: aftermath,
            // Far future on purpose. Recovery authority now re-checks this
            // deadline on every use, so a probe carrying a 1970 timestamp would
            // deny on expiry before reaching the axis this fixture is about.
            expires_at_unix_ms: Some(u64::MAX),
        });
    WorthQueryRecoveryHandle::axis_probe(binding)
}

fn authority(handle: &WorthQueryRecoveryHandle) -> WorthQueryRecoveryEffectAuthority {
    WorthQueryRecoveryEffectAuthority::mint(handle.runtime_authority(), handle.authority_identity())
}

#[test]
fn recorded_inverse_handle_denies_compensation_from_its_own_mechanism() {
    let handle = probe_handle(fixture::freeze_account());
    let authority = authority(&handle);
    let denied = compensate_recovery_handle(handle, &authority)
        .expect_err("recorded inverse is not compensation");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::CompensationNotAdmitted
    );
}

#[test]
fn runtime_alone_handle_denies_reconciliation_from_its_own_authority() {
    let handle = probe_handle(fixture::freeze_account());
    let authority = authority(&handle);
    let denied = reconcile_recovery_handle(handle, &authority)
        .expect_err("runtime-alone authority is not reconciliation");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::ReconciliationNotAdmitted
    );
}
