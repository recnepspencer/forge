//! Foreign admitted-read denial with positive twin (Gate 8.3 turn 4 / R8.32).

use worth_foundational::facade::CanonicalDigestId;
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, InstalledExternalEffectContract,
};
use worth_relational::facade::history::BranchId;

use super::{
    resolve_recovery_handle, WorthQueryAdmittedIdempotencyRead, WorthQueryRecoveryEffectAuthority,
};
use crate::domain_computation::application_aftermath::recovery_handle::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleBindingAxisProbe, WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::application_aftermath::{
    derive_external_effect_correlation_identity, ExternalEffectCorrelationBasis,
    WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationIdempotencyResolution,
};

fn baseline_parts() -> WorthQueryRecoveryHandleBindingAxisProbe {
    let schema = ApplicationSchemaBindingIdentity::from_installed_parts(
        7,
        3,
        CanonicalDigestId::new([0x11; 32]),
        CanonicalDigestId::new([0x22; 32]),
    );
    let principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
        42,
        schema.clone(),
        "notify-death-authority",
        1,
        10,
        1,
        2,
        20,
        1,
    );
    let idempotency_key = [0x71; 32];
    let correlation = derive_external_effect_correlation_identity(ExternalEffectCorrelationBasis {
        correlation_family:
            worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                "estate-death-notice-rail",
            )
            .unwrap(),
        operation_slot: "notify-death",
        operation_version: 1,
        outcome_identity: 9,
        idempotency_key: &idempotency_key,
        branch: "2",
    })
    .expect("correlation basis");
    WorthQueryRecoveryHandleBindingAxisProbe {
        runtime_instance_id: 99,
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
        dispatch_outbox: WorthQueryDispatchOutboxRecord::from_installed_contract(
            correlation,
            &InstalledExternalEffectContract::Declared {
                correlation_family: worth_query_installation::facade::WorthQueryExternalEffectCorrelationFamily::new(
                    "estate-death-notice-rail",
                )
                .unwrap(),
                effect: "EstateDeathNotificationEffect".to_owned(),
                rust_payload_type: worth_query_declaration::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    "worth.query.test.estate-death-notification-request.v1",
                ),
                protocol: ApplicationExternalEffectProtocol::new(
                    BoundaryProtocolIdentity::new("test.estate-death-notification"),
                    BoundaryProtocolVersion::new(1),
                ),
                maximum_payload_bytes: 64,
            },
            vec![1, 2, 3],
            9,
        ),
        dispatch_outbox_record_ref: Some(
            worth_relational::facade::transactions::RecordRef::Entity(
                worth_relational::facade::identity::EntityId::new(
                    worth_relational::facade::identity::PartitionId::main(),
                    77,
                    1,
                ),
            ),
        ),
        installed_aftermath:
            crate::domain_computation::application_aftermath::aftermath_schema_fixture::notify_death(
            ),
        // Far future on purpose. Recovery authority now re-checks this
        // deadline on every use, so a probe carrying a 1970 timestamp would
        // deny on expiry before reaching the axis this fixture is about.
        expires_at_unix_ms: Some(u64::MAX),
    }
}

fn probe_handle(idempotency: WorthQueryApplicationIdempotencyBinding) -> WorthQueryRecoveryHandle {
    let mut parts = baseline_parts();
    parts.idempotency = idempotency;
    WorthQueryRecoveryHandle::axis_probe(WorthQueryRecoveryHandleBinding::axis_probe(parts))
}

#[test]
fn foreign_admitted_idempotency_read_denies_distinctly() {
    let binding_a = WorthQueryApplicationIdempotencyBinding::new([0xA1; 32], [0xA2; 32]);
    let binding_b = WorthQueryApplicationIdempotencyBinding::new([0xB1; 32], [0xB2; 32]);
    let handle_b = probe_handle(binding_b);
    let registry = handle_b.registry_arc();
    let authority_b = WorthQueryRecoveryEffectAuthority::mint(
        handle_b.runtime_authority(),
        handle_b.authority_identity(),
    );
    let read_a = WorthQueryAdmittedIdempotencyRead::mint(
        binding_a,
        WorthQueryApplicationIdempotencyResolution::Unseen,
    );

    let denied = resolve_recovery_handle(handle_b, &authority_b, read_a)
        .expect_err("foreign read must deny");
    assert_eq!(
        denied.kind(),
        WorthQueryRecoveryHandleDenialKind::ForeignIdempotencyRead
    );
    registry.assert_no_live_handles();
}

#[test]
fn matching_admitted_idempotency_read_resolves() {
    let binding = WorthQueryApplicationIdempotencyBinding::new([0xC1; 32], [0xC2; 32]);
    let handle = probe_handle(binding);
    let registry = handle.registry_arc();
    let authority = WorthQueryRecoveryEffectAuthority::mint(
        handle.runtime_authority(),
        handle.authority_identity(),
    );
    let read = WorthQueryAdmittedIdempotencyRead::mint(
        binding,
        WorthQueryApplicationIdempotencyResolution::Unseen,
    );

    let resolution =
        resolve_recovery_handle(handle, &authority, read).expect("matching read resolves");
    assert_eq!(
        resolution,
        WorthQueryApplicationIdempotencyResolution::Unseen
    );
    registry.assert_no_live_handles();
}
