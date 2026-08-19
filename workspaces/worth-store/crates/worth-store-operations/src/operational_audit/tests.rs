use crate::{
    OperationalControlRecord, OperationalControlRecordKind, OperationalOperationId,
    OperationalTransitionId, OperationalWorkflowKind,
};
use worth_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, FoundationalProfileSet,
    FoundationalProfileSetInput, ObservationActivationProfile, RetentionDeliveryProfile,
    SupportPostureProfile,
};

use super::{
    assemble_operational_audit_records, derive_operational_audit_records, AuditCompletenessDenial,
    ExpectedAuditTransitionSet, OperationalEvidenceExport,
};

#[test]
fn duplicate_and_reordered_delivery_assembles_to_the_same_causal_stream() {
    let durable = durable_history();
    let canonical = derive_operational_audit_records(&durable).unwrap();
    let delivered = vec![
        canonical[2].clone(),
        canonical[0].clone(),
        canonical[1].clone(),
        canonical[1].clone(),
    ];

    let assembled = assemble_operational_audit_records(delivered).unwrap();

    assert_eq!(assembled, canonical);
}

#[test]
fn typed_exports_are_deterministic_and_preserve_canonical_semantics() {
    let durable = durable_history();
    let operation = durable[0].operation_id().clone();
    let records = derive_operational_audit_records(&durable).unwrap();
    let completeness =
        ExpectedAuditTransitionSet::from_durable_control_records(operation, &durable)
            .unwrap()
            .verify(&records)
            .unwrap();
    let reordered = vec![records[2].clone(), records[0].clone(), records[1].clone()];

    let canonical =
        OperationalEvidenceExport::from_complete_audit(&completeness, &records).unwrap();
    let replayed =
        OperationalEvidenceExport::from_complete_audit(&completeness, &reordered).unwrap();

    assert_eq!(canonical.export_identity(), replayed.export_identity());
    assert_eq!(canonical.rows(), replayed.rows());
    assert_eq!(canonical.rows().len(), 3);
    assert_eq!(canonical.rows()[1].transition_id(), "authorized");
    assert_eq!(
        canonical.rows()[1].transition_kind(),
        super::OperationalAuditTransitionKind::AuthorizationConsumed
    );
}

#[test]
fn completeness_is_derived_from_durable_truth_and_localizes_the_missing_transition() {
    let durable = durable_history();
    let operation = durable[0].operation_id().clone();
    let expected =
        ExpectedAuditTransitionSet::from_durable_control_records(operation, &durable).unwrap();
    let mut delivered = derive_operational_audit_records(&durable).unwrap();
    let missing = delivered.remove(1).transition_id().as_str().to_owned();

    assert_eq!(
        expected.verify(&delivered),
        Err(AuditCompletenessDenial::MissingTransition(missing))
    );
}

#[test]
fn complete_reordered_delivery_preserves_the_terminal_identity() {
    let durable = durable_history();
    let operation = durable[0].operation_id().clone();
    let expected =
        ExpectedAuditTransitionSet::from_durable_control_records(operation, &durable).unwrap();
    let canonical = derive_operational_audit_records(&durable).unwrap();
    let delivered = vec![
        canonical[2].clone(),
        canonical[0].clone(),
        canonical[1].clone(),
    ];

    let receipt = expected.verify(&delivered).unwrap();

    assert_eq!(receipt.transition_count(), 3);
    assert_eq!(
        receipt.terminal_record_identity(),
        canonical[2].record_identity()
    );
}

#[test]
fn support_widening_uses_the_foundational_requested_admitted_materialized_progression() {
    let record = derive_operational_audit_records(&durable_history())
        .unwrap()
        .remove(0);
    let profile = FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::CertificationReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Durable,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
        execution_objective: ExecutionObjectiveProfile::Throughput,
        observation_activation: ObservationActivationProfile::Continuous,
    })
    .unwrap();

    let plan = record
        .request_support_projection(profile)
        .plan_materialization()
        .unwrap();
    assert!(plan.cost().requested_surface_count() > 0);
    assert_eq!(
        plan.cost().inventory_surface_count(),
        plan.availability_decisions().len() as u32
    );

    let materialized = plan.materialize();
    assert_eq!(materialized.payload().operation_id(), "audit-operation");
    assert_eq!(
        materialized.payload().record_identity(),
        record.record_identity()
    );
    let boundary = materialized.prepare_foundational_boundary_bundle();
    assert_eq!(
        boundary.payload().payload().record_identity(),
        record.record_identity()
    );
}

fn durable_history() -> Vec<OperationalControlRecord> {
    let operation = OperationalOperationId::new("audit-operation").unwrap();
    vec![
        record(
            operation.clone(),
            "opened",
            OperationalControlRecordKind::WorkflowOpened {
                workflow: OperationalWorkflowKind::ReplicaBootstrap,
            },
        ),
        record(
            operation.clone(),
            "authorized",
            OperationalControlRecordKind::AuthorizationConsumed {
                authorization_identity: [2; 32],
                plan_fingerprint: [3; 32],
                operation_tag: 10,
                execution_plan_fingerprint: Some([4; 32]),
                assertion_identity: [5; 32],
                expires_at: 99,
                replay_same_operation_identity: true,
            },
        ),
        record(
            operation,
            "transferred",
            OperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
                authorization_plan_fingerprint: [3; 32],
                execution_plan_fingerprint: [4; 32],
                receipt_identity: [6; 32],
                durable_target_identity: [7; 32],
                source_lease_identity: [8; 32],
                source_bytes_read: 1024,
                output_bytes_written: 1024,
                backend_requests: 4,
                maximum_resident_buffer_bytes: 256,
            },
        ),
    ]
}

fn record(
    operation_id: OperationalOperationId,
    transition: &str,
    kind: OperationalControlRecordKind,
) -> OperationalControlRecord {
    OperationalControlRecord::from_persisted_parts(
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint([1; 32]),
        operation_id,
        OperationalTransitionId::new(transition).unwrap(),
        kind,
    )
}
