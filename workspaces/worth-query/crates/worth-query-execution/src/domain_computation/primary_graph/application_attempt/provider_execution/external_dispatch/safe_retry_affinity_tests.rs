//! A performed re-dispatch remains affine to its exact recovery handle.

use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolVersion, CanonicalDigestId,
};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, InstalledExternalEffectContract,
};
use worth_relational::facade::history::BranchId;

use super::{WorthQueryExternalRedispatchMint, WorthQueryPerformedExternalRedispatchSeal};
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
use crate::domain_computation::application_aftermath::recovery_progression::{
    safe_retry_recovery_handle, WorthQueryPerformedExternalRedispatch,
    WorthQueryRecoveryEffectAuthority,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::{
    commit_observe_and_admit_fixture, WorthQueryApplicationIdempotencyBinding,
};

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
        correlation_family:
            worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                "estate-death-notice-rail",
            )
            .unwrap(),
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
            correlation_family:
                worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                    "estate-death-notice-rail",
                )
                .unwrap(),
            effect: "notify-death-effect".to_owned(),
            rust_payload_type: worth_query_declaration::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "worth.query.test.notify-death-payload.v1",
            ),
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

fn probe_handle(outbox: Option<WorthQueryDispatchOutboxRecord>) -> WorthQueryRecoveryHandle {
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
    let record_ref = outbox.as_ref().map(|_| {
        worth_relational::facade::transactions::RecordRef::Entity(
            worth_relational::facade::identity::EntityId::new(
                worth_relational::facade::identity::PartitionId::main(),
                77,
                1,
            ),
        )
    });
    WorthQueryRecoveryHandle::axis_probe(WorthQueryRecoveryHandleBinding::axis_probe(
        WorthQueryRecoveryHandleBindingAxisProbe {
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
            dispatch_outbox: outbox,
            dispatch_outbox_record_ref: record_ref,
            installed_aftermath: fixture::notify_death(),
            expires_at_unix_ms: Some(u64::MAX),
        },
    ))
}

fn authority(handle: &WorthQueryRecoveryHandle) -> WorthQueryRecoveryEffectAuthority {
    WorthQueryRecoveryEffectAuthority::mint(handle.runtime_authority(), handle.authority_identity())
}

fn performed_redispatch(
    handle: &WorthQueryRecoveryHandle,
) -> WorthQueryPerformedExternalRedispatch {
    let outbox = handle.binding().dispatch_outbox().unwrap();
    let admitted = commit_observe_and_admit_fixture(outbox).0;
    let dispatch = dispatch_external_effect(&CompletingTransport, admitted)
        .expect("production dispatch classifies the completing transport outcome");
    WorthQueryPerformedExternalRedispatch::record(WorthQueryPerformedExternalRedispatchSeal::new(
        WorthQueryExternalRedispatchMint::witness(),
        handle.authority_identity(),
        dispatch,
    ))
}

#[test]
fn redispatch_performed_for_handle_a_cannot_safe_retry_handle_b() {
    let outbox_a = outbox_for("notify-death-a");
    let outbox_b = outbox_for("notify-death-b");
    assert_ne!(
        outbox_a.correlation().bytes(),
        outbox_b.correlation().bytes()
    );
    let handle_a = probe_handle(Some(outbox_a));
    let handle_b = probe_handle(Some(outbox_b));
    let authority_b = authority(&handle_b);
    let redispatch_a = performed_redispatch(&handle_a);

    let denied = safe_retry_recovery_handle(handle_b, &authority_b, redispatch_a)
        .expect_err("a proof performed for handle A cannot retire handle B");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::CorrelationMismatch
    );

    let authority_a = authority(&handle_a);
    let redispatch_a = performed_redispatch(&handle_a);
    safe_retry_recovery_handle(handle_a, &authority_a, redispatch_a)
        .expect("the exact handle admits its own performed re-dispatch");
}

#[test]
fn safe_retry_denies_when_the_handle_carries_no_co_committed_outbox() {
    let source = probe_handle(Some(outbox_for("notify-death-a")));
    let redispatch = performed_redispatch(&source);
    drop(source);
    let outboxless = probe_handle(None);
    let authority = authority(&outboxless);
    let denied = safe_retry_recovery_handle(outboxless, &authority, redispatch)
        .expect_err("no bound outbox means no proof can match");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::CorrelationMismatch
    );
}
