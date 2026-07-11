use forge_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use forge_store_contracts::QueueProducerResourceShape;
use forge_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_secure_io_scope_for_scheduler, lower_buffer_pool_queue_declaration,
    reject_lower_authority_secure_io_scope_source, IoSchedulerBackendCapabilityRequirement,
    QueueExecutionAdmissionRequest, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationDenial, SecureIoPreservationRequest,
};
use forge_store_security::{
    classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_operator_identity_as_security_scope_source,
    classify_terminal_json_label_as_security_scope_source,
};

use super::support::{backend_witness, point_read_budget, scheduler_security_scope};

#[test]
fn secure_io_receipt_is_required_for_secure_queue_admission() {
    let reservation = forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = read_ahead_producer(budget);
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");
    let scope = scheduler_security_scope();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("scope-preserving direct I/O should admit secure-I/O preservation");
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("buffer-pool producer should lower")
        .with_secure_io_scope(secure_io);

    let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        super::support::policy_receipt(budget),
    ))
    .expect("queue work should preserve admitted secure-I/O scope");

    assert_eq!(plan.work().secure_io(), Some(secure_io));
}
#[test]
fn ordinary_read_ahead_queue_admission_requires_secure_io_receipt() {
    let reservation = forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = read_ahead_producer(budget);
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("buffer-pool producer should lower");
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");

    let denial = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        super::support::policy_receipt(budget),
    ))
    .expect_err("read-ahead must not admit without secure-I/O preservation");

    assert_eq!(
        denial,
        forge_store_io_scheduler::QueueExecutionAdmissionDenial::MissingSecureIoPreservation
    );
}

#[test]
fn secure_io_receipt_operation_cannot_be_laundered() {
    let reservation = forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = read_ahead_producer(budget);
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");
    let scope = scheduler_security_scope();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::BackgroundLease, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("background secure-I/O receipt should admit");
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("buffer-pool producer should lower")
        .with_secure_io_scope(secure_io);

    let denial = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        super::support::policy_receipt(budget),
    ))
    .expect_err("background receipt must not satisfy read-ahead work");

    assert_eq!(
        denial,
        forge_store_io_scheduler::QueueExecutionAdmissionDenial::SecureIoDenied(
            SecureIoPreservationDenial::OperationMismatch {
                expected: SecureIoOperation::ReadAhead,
                actual: SecureIoOperation::BackgroundLease,
            }
        )
    );
}

#[test]
fn unsupported_secure_io_posture_denies_typed() {
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");
    let scope = scheduler_security_scope();

    let denial = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::BatchedWrite, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::SecureFrameCompatible),
    )
    .expect_err("ordinary direct I/O must not satisfy secure-frame posture");

    assert_eq!(
        denial,
        SecureIoPreservationDenial::UnsupportedSecureIoPosture {
            operation: SecureIoOperation::BatchedWrite,
            requirement: IoSchedulerBackendCapabilityRequirement::DirectIo,
        }
    );
}

#[test]
fn lower_authority_sources_report_secure_io_classifier_denials() {
    for source in [
        classify_identity_provider_claim_as_security_scope_source(),
        classify_kms_key_id_as_security_scope_source(),
        classify_iam_role_as_security_scope_source(),
        classify_operator_identity_as_security_scope_source(),
        classify_terminal_json_label_as_security_scope_source(),
    ] {
        let denial = reject_lower_authority_secure_io_scope_source(source)
            .expect_err("lower authority source must not admit secure-I/O scope");
        assert!(matches!(
            denial,
            SecureIoPreservationDenial::LowerAuthoritySecurityScopeSource(_)
        ));
    }
}

fn read_ahead_producer(
    budget: forge_store_io_scheduler::BackgroundResourceBudget,
) -> BufferPoolQueueExecutionDeclaration {
    BufferPoolQueueExecutionDeclaration::read_ahead(
        11,
        QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    )
}
