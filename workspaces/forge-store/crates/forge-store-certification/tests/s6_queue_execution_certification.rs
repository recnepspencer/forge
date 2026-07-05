use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use forge_store_certification::S6CertifiedQueueExecutionEvidence;
use forge_store_contracts::S6QueueProducerResourceShape;
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use forge_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_s5_1_security_scope_for_s6_io_qos, admit_secure_io_scope_for_scheduler,
    execute_grouped_ready_queue_plans, group_ready_queue_pair, lower_buffer_pool_queue_declaration,
    reject_lower_authority_secure_io_scope_source, BackgroundResourceBudget, BandwidthToken,
    CacheResidencyHint, IoSchedulerBackendCapabilityRequirement, QueueExecutionAdmissionRequest,
    QueueGroupingOutcome, QueueSlot, ReadAheadWindow, S6IoQosSecurityScopeHandoff,
    SecureIoOperation, SecureIoPostureRequirement, SecureIoPreservationDenial,
    SecureIoPreservationRequest, WorkerPermit,
};
use forge_store_physical_backend::{
    preserve_secure_io_for_backend_completion, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionBudgetBinding,
    BackendQueueExecutionCompletion, BackendQueueExecutionPlanBinding,
    BackendQueueExecutionPosture, BackendQueueExecutionReplayBinding, BackendQueueSpeculativeScope,
    BackendRebindTriggers, BackendSecureIoPreservationDenial, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::{
    admitted_store_internal_security_scope_for_s6_test, classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_operator_identity_as_security_scope_source,
    classify_terminal_json_label_as_security_scope_source,
};

#[test]
fn grouped_certification_preserves_secondary_replay_identity() {
    let grouping = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = grouping else {
        panic!("equivalent ready plans should group");
    };
    let expected_secondary = grouped.replay_identities()[1];
    let scope = BackendQueueSpeculativeScope::admitted(
        grouped.first().grouping_basis().security_scope_identity(),
        grouped.first().grouping_basis().tenant_scope(),
        grouped.first().grouping_basis().key_scope(),
    );
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("certification backend posture should admit");
    let completion = BackendQueueExecutionCompletion::for_certification(
        grouped
            .backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(1)
    .observe_read_ahead(1, scope);
    let outcome = execute_grouped_ready_queue_plans(grouped, completion);

    let certified = S6CertifiedQueueExecutionEvidence::from_outcome(&outcome)
        .expect("executed grouped outcome should certify");

    assert_eq!(
        certified.secondary_replay_identity(),
        Some(expected_secondary)
    );
    assert_eq!(certified.counters().grouped_writes(), 2);
}

#[test]
fn secure_io_receipt_is_required_for_secure_queue_admission() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        11,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");
    let scope = s6_security_scope();
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
        policy_receipt(budget),
    ))
    .expect("queue work should preserve admitted secure-I/O scope");

    assert_eq!(plan.work().secure_io(), Some(secure_io));
}

#[test]
fn ordinary_read_ahead_queue_admission_requires_secure_io_receipt() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        11,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
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
        policy_receipt(budget),
    ))
    .expect_err("read-ahead must not admit without secure-I/O preservation");

    assert_eq!(
        denial,
        forge_store_io_scheduler::QueueExecutionAdmissionDenial::MissingSecureIoPreservation
    );
}

#[test]
fn secure_io_receipt_operation_cannot_be_laundered() {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        11,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("direct I/O backend should admit");
    let scope = s6_security_scope();
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
        policy_receipt(budget),
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
    let scope = s6_security_scope();

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

#[test]
fn backend_secure_io_preservation_rejects_wrong_read_ahead_scope() {
    let scope = s6_security_scope().permission().identity();
    let binding = secure_backend_binding(scope);
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("backend posture should admit");
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        scope,
        forge_store_security::StoreTenantScope::RepairBlastRadius,
        scope.key_scope(),
    );
    let completion = BackendQueueExecutionCompletion::for_certification(binding, posture)
        .observe_read_ahead(1, wrong_scope);

    let denial = preserve_secure_io_for_backend_completion(completion)
        .expect_err("backend must reject cross-scope read-ahead observation");

    assert_eq!(
        denial,
        BackendSecureIoPreservationDenial::ReadAheadScopeMismatch
    );
}

fn admitted_plan() -> forge_store_io_scheduler::QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        7,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
    let work = lower_buffer_pool_queue_declaration(producer, reservation)
        .expect("buffer-pool producer should lower to queue work");
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        work.backend_requirement(),
    )
    .expect("backend should admit test queue work");
    let scope = s6_security_scope();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("read-ahead secure-I/O scope should admit");
    let work = work.with_secure_io_scope(secure_io);
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
    .expect("queue work should admit")
}

fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn backend_witness() -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering()
            .with_fdatasync_durability(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend witness should admit")
}

fn s6_security_scope() -> forge_store_io_scheduler::IoSchedulerS6SecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness)
        .expect("S.5.1 readiness should hand off to S.6");
    admit_s5_1_security_scope_for_s6_io_qos(handoff)
}

fn secure_backend_binding(
    scope: forge_store_security::StoreSecurityScopeIdentity,
) -> BackendQueueExecutionPlanBinding {
    let replay = BackendQueueExecutionReplayBinding::from_store_queue_replay(
        16,
        8,
        1,
        scope,
        scope.tenant_scope(),
        scope.key_scope(),
        scope.authenticity_requirement(),
        0,
        1,
        1,
        BackendQueueExecutionBudgetBinding::new(1, 4096, 0, 0, 1, 0, 0, 1, 0, 0),
    );
    BackendQueueExecutionPlanBinding::from_store_replay_binding(
        replay,
        None,
        BackendTargetProfile::PosixFileFsyncDirSync,
        forge_store_physical_backend::CapabilityEvidenceClass::CertifiedBackendProfile,
        0,
    )
}

fn policy_receipt(
    budget: BackgroundResourceBudget,
) -> forge_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("policy claim should build");
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            (budget.queue_slots() + budget.worker_permits()) as u32,
            (budget.queue_slots() + budget.worker_permits()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Density,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            budget.read_ahead_window() as u32,
            budget.read_ahead_window() as u32,
        )
        .finish()
        .expect("policy receipt should build")
}
