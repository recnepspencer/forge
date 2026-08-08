//! Q8.20 — recovery authority is affine to its owning runtime.

use worth_foundational::facade::CanonicalDigestId;
use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::history::BranchId;

use super::authority::require_inspect_disclosure;
use super::{
    require_fresh_effect_authority, WorthQueryRecoveryEffectAuthority,
    WorthQueryRecoveryInspectAuthority,
};
use crate::domain_computation::application_aftermath::recovery_handle::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleBindingAxisProbe, WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::managed_run::WorthQueryRecoveryResourceTerminal;
use crate::domain_computation::primary_graph::WorthQueryApplicationIdempotencyBinding;

fn probe_handle_with_runtime_instance(runtime_instance_id: u64) -> WorthQueryRecoveryHandle {
    probe_handle_expiring_at(runtime_instance_id, u64::MAX)
}

fn probe_handle_expiring_at(
    runtime_instance_id: u64,
    expires_at_unix_ms: u64,
) -> WorthQueryRecoveryHandle {
    let schema = ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    );
    let principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
        42,
        schema,
        "notify-death-authority",
        1,
        10,
        1,
        2,
        20,
        1,
    );
    WorthQueryRecoveryHandle::axis_probe(WorthQueryRecoveryHandleBinding::axis_probe(
        WorthQueryRecoveryHandleBindingAxisProbe {
            runtime_instance_id,
            schema_identity: [0x33; 32],
            branch: BranchId("2".to_owned()),
            application_binding_generation: 3,
            installed_operation: [0x44; 32],
            attempt_commit_id: 501,
            mutation_work: None,
            retained_preimage: None,
            retained_governed_input_identity: None,
            principal_scope,
            idempotency: WorthQueryApplicationIdempotencyBinding::new([0x55; 32], [0x56; 32]),
            provider_posture: None,
            dispatch_outbox: None,
            installed_aftermath: crate::domain_computation::application_aftermath::aftermath_schema_fixture::notify_death(),
            expires_at_unix_ms: Some(expires_at_unix_ms),
        },
    ))
}

#[test]
fn concurrent_force_termination_cannot_be_reported_as_a_consumed_transition() {
    let handle = probe_handle_with_runtime_instance(91);
    let registry = handle.registry_arc();
    let slot = handle.registry_slot();
    assert!(registry.mark_terminal(slot, WorthQueryRecoveryResourceTerminal::ForceTerminated));

    let denied = handle
        .consume(WorthQueryRecoveryResourceTerminal::Consumed)
        .expect_err("force termination wins the terminal race");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::AlreadyTerminal
    );
    assert_eq!(
        registry.terminal_of(slot),
        Some(WorthQueryRecoveryResourceTerminal::ForceTerminated)
    );
    registry.assert_no_live_handles();
}

// R8.31 — the "fresh" in `require_fresh_effect_authority` has to mean fresh at
// *use*, not fresh at mint. Expiry was checked once, inside
// `admit_recovery_effect_authority`, and never again; the authority object is
// not `Clone` but it is also not consumed, so holding one across the handle's
// deadline kept every transition open indefinitely. Both tests below mint the
// authority exactly the way the runtime does and then present it against a
// handle whose deadline has passed.
#[test]
fn effect_authority_minted_before_expiry_denies_after_the_deadline() {
    let handle = probe_handle_expiring_at(31, 1);
    let authority = WorthQueryRecoveryEffectAuthority::mint(
        handle.runtime_authority(),
        handle.authority_identity(),
    );
    let registry = handle.registry_arc();

    // The handle really is past its deadline against the runtime's own clock,
    // so this is not a test that passes because nothing expired.
    assert!(matches!(
        super::expiry::evaluate_expiry(&handle, registry.clock()).expect("clock samples"),
        super::expiry::WorthQueryRecoveryExpiryEvaluation::Expired(_)
    ));

    let denied = require_fresh_effect_authority(&handle, &authority)
        .expect_err("authority minted before the deadline cannot transition after it");
    assert_eq!(denied.kind(), WorthQueryRecoveryHandleDenialKind::Expired);

    drop(handle);
    registry.assert_no_live_handles();
}

#[test]
fn inspect_authority_minted_before_expiry_denies_after_the_deadline() {
    let handle = probe_handle_expiring_at(32, 1);
    let authority = WorthQueryRecoveryInspectAuthority::mint(
        handle.runtime_authority(),
        handle.authority_identity(),
    );
    let registry = handle.registry_arc();

    let denied = require_inspect_disclosure(&handle, &authority)
        .expect_err("disclosure minted before the deadline cannot inspect after it");
    assert_eq!(denied.kind(), WorthQueryRecoveryHandleDenialKind::Expired);

    drop(handle);
    registry.assert_no_live_handles();
}

#[test]
fn effect_authority_denies_a_different_runtime_with_the_same_registry_slot() {
    let (handle_a, handle_b) = colliding_slot_handles();
    let authority_a = WorthQueryRecoveryEffectAuthority::mint(
        handle_a.runtime_authority(),
        handle_a.authority_identity(),
    );
    let registry_a = handle_a.registry_arc();
    let registry_b = handle_b.registry_arc();

    require_fresh_effect_authority(&handle_a, &authority_a).expect("owner runtime admits");
    let denied = require_fresh_effect_authority(&handle_b, &authority_a)
        .expect_err("equal registry slot cannot substitute for runtime ownership");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    drop(handle_a);
    drop(handle_b);
    registry_a.assert_no_live_handles();
    registry_b.assert_no_live_handles();
}

#[test]
fn inspect_authority_denies_a_different_runtime_with_the_same_registry_slot() {
    let (handle_a, handle_b) = colliding_slot_handles();
    let authority_a = WorthQueryRecoveryInspectAuthority::mint(
        handle_a.runtime_authority(),
        handle_a.authority_identity(),
    );
    let registry_a = handle_a.registry_arc();
    let registry_b = handle_b.registry_arc();

    require_inspect_disclosure(&handle_a, &authority_a).expect("owner runtime admits");
    let denied = require_inspect_disclosure(&handle_b, &authority_a)
        .expect_err("equal registry slot cannot substitute for runtime ownership");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    drop(handle_a);
    drop(handle_b);
    registry_a.assert_no_live_handles();
    registry_b.assert_no_live_handles();
}

#[test]
fn effect_authority_denies_a_different_handle_in_the_same_runtime() {
    let (handle_a, handle_b) = same_runtime_handles();
    let authority_a = WorthQueryRecoveryEffectAuthority::mint(
        handle_a.runtime_authority(),
        handle_a.authority_identity(),
    );
    let registry = handle_a.registry_arc();

    require_fresh_effect_authority(&handle_a, &authority_a).expect("exact handle admits");
    let denied = require_fresh_effect_authority(&handle_b, &authority_a)
        .expect_err("same-runtime handle substitution must deny");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    drop(handle_a);
    drop(handle_b);
    registry.assert_no_live_handles();
}

#[test]
fn inspect_authority_denies_a_different_handle_in_the_same_runtime() {
    let (handle_a, handle_b) = same_runtime_handles();
    let authority_a = WorthQueryRecoveryInspectAuthority::mint(
        handle_a.runtime_authority(),
        handle_a.authority_identity(),
    );
    let registry = handle_a.registry_arc();

    require_inspect_disclosure(&handle_a, &authority_a).expect("exact handle admits");
    let denied = require_inspect_disclosure(&handle_b, &authority_a)
        .expect_err("same-runtime handle substitution must deny");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    drop(handle_a);
    drop(handle_b);
    registry.assert_no_live_handles();
}

// Q8.20-C4 — the two foreignness denials must not collapse into one another.
// The ledger cited `binding_axis::foreign_runtime_drift_denies_distinctly` for
// this, but slice 3B removed the runtime-instance axis and that test went with
// it, leaving the admission-time `ForeignRuntime` cause with no coverage at all
// while only the proof-use `FreshAuthorityDenied` half stayed tested.
#[test]
fn admission_time_foreignness_and_proof_use_foreignness_are_different_causes() {
    let (handle_a, handle_b) = colliding_slot_handles();
    let owner = handle_a.runtime_authority();
    let registry_a = handle_a.registry_arc();
    let registry_b = handle_b.registry_arc();

    // Admission time: the handle presented to obtain authority is not ours.
    let admission_denial =
        super::authority::ensure_same_runtime(owner, handle_b.runtime_authority())
            .expect_err("a handle from another runtime cannot be admitted here");
    assert_eq!(
        admission_denial.kind(),
        WorthQueryRecoveryHandleDenialKind::ForeignRuntime
    );
    super::authority::ensure_same_runtime(owner, handle_a.runtime_authority())
        .expect("our own handle admits");

    // Proof use: the authority *is* ours, but it is not this handle's.
    let authority_a = WorthQueryRecoveryEffectAuthority::mint(owner, handle_a.authority_identity());
    let use_denial = require_fresh_effect_authority(&handle_b, &authority_a)
        .expect_err("our authority does not carry to another handle");
    assert_eq!(
        use_denial.kind(),
        WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied
    );

    assert_ne!(admission_denial.kind(), use_denial.kind());

    drop(handle_a);
    drop(handle_b);
    registry_a.assert_no_live_handles();
    registry_b.assert_no_live_handles();
}

fn colliding_slot_handles() -> (WorthQueryRecoveryHandle, WorthQueryRecoveryHandle) {
    let handle_a = probe_handle_with_runtime_instance(11);
    let handle_b = probe_handle_with_runtime_instance(22);
    assert_eq!(handle_a.registry_slot().as_u64(), 1);
    assert_eq!(handle_b.registry_slot().as_u64(), 1);
    assert_ne!(handle_a.runtime_authority(), handle_b.runtime_authority());
    (handle_a, handle_b)
}

fn same_runtime_handles() -> (WorthQueryRecoveryHandle, WorthQueryRecoveryHandle) {
    use std::sync::Arc;

    use crate::domain_computation::managed_run::WorthQueryRecoveryHandleRegistry;

    let seed = probe_handle_with_runtime_instance(11);
    let binding = seed.binding().clone();
    drop(seed);
    let registry = Arc::new(WorthQueryRecoveryHandleRegistry::new());
    let handle_a =
        WorthQueryRecoveryHandle::axis_probe_in_registry(binding.clone(), Arc::clone(&registry));
    let handle_b = WorthQueryRecoveryHandle::axis_probe_in_registry(binding, registry);
    assert_eq!(handle_a.runtime_authority(), handle_b.runtime_authority());
    assert_ne!(handle_a.registry_slot(), handle_b.registry_slot());
    (handle_a, handle_b)
}
