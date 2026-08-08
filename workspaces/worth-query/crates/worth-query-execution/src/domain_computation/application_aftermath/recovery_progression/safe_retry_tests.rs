//! R8.66 / R8.69 — a re-dispatch proof is affine to the handle it was performed
//! for.
//!
//! `safe_retry_recovery_handle` says so in a comment and enforces it by
//! comparing the proof's outbox against the live handle binding's. Nothing
//! performed the substitution: the Bank path mints the proof against the same
//! handle it then retries, so every integration test proves the honest lane
//! only. Both `redispatch_admitted_external_effect` and
//! `safe_retry_recovery_handle` are public, so a host holding two live handles
//! can do exactly what these tests do.

use std::sync::Arc;

use worth_foundational::facade::CanonicalDigestId;
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, InstalledExternalEffectContract,
};
use worth_relational::facade::history::{BranchId, CommitId, CommitReference};
use worth_relational::facade::identity::{EntityId, PartitionId, VersionId};
use worth_relational::facade::transactions::RecordRef;

use super::redispatch::WorthQueryPerformedExternalRedispatch;
use super::safe_retry::safe_retry_recovery_handle;
use super::WorthQueryRecoveryEffectAuthority;
use crate::domain_computation::application_aftermath::aftermath_schema_fixture as fixture;
use crate::domain_computation::application_aftermath::external_effect::{
    derive_external_effect_correlation_identity, dispatch_external_effect,
    ExternalEffectCorrelationBasis, WorthQueryDispatchOutboxRecord,
    WorthQueryExternalDispatchRequest, WorthQueryExternalEffectTransport,
    WorthQueryExternalTransportOutcome,
};
use crate::domain_computation::application_aftermath::recovery_handle::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleBindingAxisProbe, WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryCommittedDispatchOutboxObservation,
};

/// Always completes. The transport is not what these tests are about — the
/// point is that a *real* dispatch, derived the production way, still cannot
/// carry across handles.
struct CompletingTransport;

impl WorthQueryExternalEffectTransport for CompletingTransport {
    fn dispatch(
        &self,
        _request: WorthQueryExternalDispatchRequest<'_>,
    ) -> WorthQueryExternalTransportOutcome {
        WorthQueryExternalTransportOutcome::Completed
    }
}

fn outbox_for(operation_slot: &str) -> WorthQueryDispatchOutboxRecord {
    let correlation = derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family: "estate-death-notice-rail",
        operation_slot,
        operation_version: 1,
        outcome_identity: 1,
        idempotency_key: &[0x55; 32],
        branch: "2",
    })
    .expect("fixture correlation basis derives");
    WorthQueryDispatchOutboxRecord::from_installed_contract(
        correlation,
        &InstalledExternalEffectContract::Declared {
            correlation_family: "estate-death-notice-rail".to_owned(),
            effect: "notify-death-effect".to_owned(),
            rust_payload_type: "fixture::NotifyDeathPayload".to_owned(),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.notify-death-payload"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 1_024,
        },
        vec![0xAB; 8],
        1,
    )
    .expect("the fixture contract declares an external effect")
}

fn probe_handle(outbox: WorthQueryDispatchOutboxRecord) -> WorthQueryRecoveryHandle {
    let schema = ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    );
    let principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
        42,
        schema,
        "safe-retry-affinity",
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
            dispatch_outbox: Some(outbox),
            installed_aftermath: fixture::notify_death(),
            expires_at_unix_ms: Some(u64::MAX),
        });
    WorthQueryRecoveryHandle::axis_probe(binding)
}

fn authority(handle: &WorthQueryRecoveryHandle) -> WorthQueryRecoveryEffectAuthority {
    WorthQueryRecoveryEffectAuthority::mint(handle.runtime_authority(), handle.authority_identity())
}

/// Perform one real dispatch for `outbox` and take the proof the runtime mints.
fn performed_redispatch(
    handle: &WorthQueryRecoveryHandle,
) -> WorthQueryPerformedExternalRedispatch {
    let outbox = handle.binding().dispatch_outbox().unwrap();
    let transport = Arc::new(CompletingTransport);
    let committed = WorthQueryCommittedDispatchOutboxObservation::fixture(
        outbox.clone(),
        CommitReference {
            commit_id: CommitId(10),
            version_id: VersionId(10),
            branch_id: BranchId("2".to_owned()),
            parents: Vec::new(),
        },
        RecordRef::Entity(EntityId::new(PartitionId::main(), 7, 1)),
    );
    let ordinal =
        crate::domain_computation::primary_graph::WorthQueryExternalDispatchAttemptOrdinal::fixture(
            1,
        );
    let admitted = crate::domain_computation::primary_graph::WorthQueryAdmittedExternalDispatchAttempt::fixture(
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity::mint_for_test(),
        committed,
        ordinal,
    );
    let dispatch = dispatch_external_effect(transport.as_ref(), admitted)
        .expect("a completing transport dispatches");
    WorthQueryPerformedExternalRedispatch::record(handle.authority_identity(), dispatch)
}

#[test]
fn redispatch_performed_for_handle_a_cannot_safe_retry_handle_b() {
    let outbox_a = outbox_for("notify-death-a");
    let outbox_b = outbox_for("notify-death-b");
    assert_ne!(
        outbox_a.correlation().bytes(),
        outbox_b.correlation().bytes(),
        "two handles must carry genuinely different co-committed outboxes"
    );
    let handle_a = probe_handle(outbox_a.clone());
    let handle_b = probe_handle(outbox_b);
    let authority_b = authority(&handle_b);
    let redispatch_a = performed_redispatch(&handle_a);

    let denied = safe_retry_recovery_handle(handle_b, &authority_b, redispatch_a)
        .expect_err("a proof performed for handle A cannot retire handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::CorrelationMismatch
    );

    // Positive twin: the same construction against its own handle admits, so
    // the denial above is about affinity and not about the fixture being unable
    // to reach safe-retry at all.
    let authority_a = authority(&handle_a);
    let redispatch_a = performed_redispatch(&handle_a);
    safe_retry_recovery_handle(handle_a, &authority_a, redispatch_a)
        .expect("the handle the re-dispatch was performed for admits");
}

#[test]
fn safe_retry_denies_when_the_handle_carries_no_co_committed_outbox() {
    // The `None` arm is the other half of the comparison: without a bound
    // outbox there is nothing to match, so no proof may substitute for one.
    let outbox = outbox_for("notify-death-a");
    let handle = probe_handle(outbox.clone());
    let binding_without_outbox = {
        let schema = ApplicationSchemaBindingIdentity::from_installed_parts(
            7,
            3,
            CanonicalDigestId::new([0x11; 32]),
            CanonicalDigestId::new([0x22; 32]),
        );
        let principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
            42,
            schema,
            "safe-retry-affinity",
            1,
            10,
            1,
            2,
            20,
            1,
        );
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
            installed_aftermath: fixture::notify_death(),
            expires_at_unix_ms: Some(u64::MAX),
        })
    };
    let redispatch = performed_redispatch(&handle);
    drop(handle);
    let outboxless = WorthQueryRecoveryHandle::axis_probe(binding_without_outbox);
    let authority = authority(&outboxless);
    let denied = safe_retry_recovery_handle(outboxless, &authority, redispatch)
        .expect_err("no bound outbox means no proof can match");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::CorrelationMismatch
    );
}
