//! Per-axis binding drift with positive twins (R8.28 / Gate 8.3 turn 3).

use worth_foundational::facade::CanonicalDigestId;
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_schema::ApplicationExternalEffectProtocol;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, InstalledExternalEffectContract,
};
use worth_relational::facade::history::BranchId;

use super::WorthQueryRecoveryBindingCurrentTruth;
use crate::domain_computation::application_aftermath::recovery_handle::{
    WorthQueryRecoveryHandleBinding, WorthQueryRecoveryHandleBindingAxisProbe,
    WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::application_aftermath::{
    derive_external_effect_correlation_identity, ExternalEffectCorrelationBasis,
    WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::WorthQueryApplicationIdempotencyBinding;

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
        correlation_family: "estate-death-notice-rail",
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
        retained_governed_input_identity: Some([0x66; 32]),
        principal_scope,
        idempotency: WorthQueryApplicationIdempotencyBinding::new([0x55; 32], [0x56; 32]),
        provider_posture: None,
        dispatch_outbox: WorthQueryDispatchOutboxRecord::from_installed_contract(
            correlation,
            &InstalledExternalEffectContract::Declared {
                correlation_family: "estate-death-notice-rail".to_owned(),
                effect: "EstateDeathNotificationEffect".to_owned(),
                rust_payload_type: "fixture::EstateDeathNotificationRequest".to_owned(),
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

fn assert_axis_drift(
    kind: WorthQueryRecoveryHandleDenialKind,
    mutate: impl FnOnce(&mut WorthQueryRecoveryHandleBindingAxisProbe),
) {
    let baseline = baseline_parts();
    let truth = WorthQueryRecoveryBindingCurrentTruth::axis_probe(baseline.clone());
    let matching = WorthQueryRecoveryHandleBinding::axis_probe(baseline);
    truth.check(&matching).expect("positive twin admits");

    let mut drifted_parts = baseline_parts();
    mutate(&mut drifted_parts);
    let drifted = WorthQueryRecoveryHandleBinding::axis_probe(drifted_parts);
    let denied = truth.check(&drifted).expect_err("drifted binding denies");
    assert_eq!(denied.kind(), kind);
}

#[test]
fn schema_mismatch_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::SchemaMismatch,
        |parts| {
            parts.schema_identity = [0x34; 32];
        },
    );
}

#[test]
fn branch_mismatch_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::BranchMismatch,
        |parts| {
            parts.branch = BranchId("102".to_owned());
            parts.application_binding_generation = 4;
        },
    );
}

// The two tests around this one both drift `branch`, and `Branch` is declared
// before `ApplicationBindingGeneration`, so it is the axis that reports. That
// left the generation axis with no test of its own: deleting it from the axis
// list kept the whole suite green. This drifts only schema binding generation.
#[test]
fn application_binding_generation_drift_denies_on_its_own_axis() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::ApplicationBindingGenerationMismatch,
        |parts| {
            parts.application_binding_generation = 4;
        },
    );
}

#[test]
fn foreign_branch_equal_ordinal_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::ForeignBranchEqualOrdinal,
        |parts| {
            parts.branch = BranchId("102".to_owned());
        },
    );
}

#[test]
fn operation_mismatch_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::OperationMismatch,
        |parts| {
            parts.installed_operation = [0x45; 32];
        },
    );
}

#[test]
fn governed_input_mismatch_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::GovernedInputMismatch,
        |parts| {
            parts.retained_governed_input_identity = Some([0x67; 32]);
        },
    );
}

#[test]
fn foreign_principal_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::ForeignPrincipal,
        |parts| {
            let schema = parts.principal_scope.binding_identity().clone();
            parts.principal_scope = WorthQueryOperationScopeBinding::axis_probe_scope(
                parts.principal_scope.runtime_authority(),
                schema,
                parts.principal_scope.operation_authority_identity(),
                9,
                99,
                2,
                parts.principal_scope.scope().partition_id(),
                parts.principal_scope.scope().local_slot(),
                parts.principal_scope.scope().generation(),
            );
        },
    );
}

#[test]
fn installed_aftermath_identity_drift_denies_distinctly() {
    assert_axis_drift(
        WorthQueryRecoveryHandleDenialKind::CompatibilityGenerationMismatch,
        |parts| {
            parts.installed_aftermath =
                crate::domain_computation::application_aftermath::aftermath_schema_fixture::transfer(
                );
        },
    );
}
