use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, FoundationalPolicyAdmissionReceipt,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::admitted_secure_frame_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_queue_execution_plan, admit_secure_frame_backend_capability_for_scheduler_claim,
    admit_secure_io_scope_for_scheduler, admit_security_scope_for_scheduler,
    execute_ready_queue_plan, lower_buffer_pool_read_queue_declaration, BackgroundResourceBudget,
    IoSchedulerSecurityScopeAdmission, QueueExecutionAdmissionRequest, QueueExecutionOutcome,
    QueueExecutionReadyPlan, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationRequest,
};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendQueueSpeculativeScope, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};

#[test]
fn secure_frame_queue_execution_consumes_backend_secure_io_preservation() {
    let plan = secure_frame_read_ahead_plan();
    let scope = speculative_scope(&plan);
    let completion = completion_for(&plan, scope);
    let outcome = execute_ready_queue_plan(plan, completion);

    let QueueExecutionOutcome::Executed(executed) = outcome else {
        panic!("secure-frame completion with matching scope should execute");
    };
    assert_eq!(executed.counters().read_ahead_units(), 1);

    let plan = secure_frame_read_ahead_plan();
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        worth_store_security::StoreKeyScope::BackupExportEnvelope,
    );
    let completion = completion_for(&plan, wrong_scope);
    let outcome = execute_ready_queue_plan(plan, completion);

    let QueueExecutionOutcome::Violation(violation) = outcome else {
        panic!("secure-frame completion with cross-key speculation must violate");
    };
    assert_eq!(violation.counters().read_ahead_units(), 1);
    assert_eq!(violation.counters().violation_events(), 1);
}
fn secure_frame_read_ahead_plan() -> QueueExecutionReadyPlan {
    let reservation = admitted_secure_frame_read_reservation_for_certification_test();
    let producer = worth_store_test_support::read_ahead_declaration_for_real_pool(
        reservation.security_scope_identity(),
        7,
        QueueProducerResourceShape::new()
            .with_queue_slots(1)
            .with_read_ahead_windows(1)
            .with_worker_permits(1),
    );
    let work = lower_buffer_pool_read_queue_declaration(producer, reservation)
        .expect("secure-frame producer work should lower through scheduler");
    let security = io_qos_security_scope_admission();
    let (backend, _) = secure_frame_backend(&security);
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &security, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("secure-frame read-ahead secure-I/O should admit");
    let work = work.with_secure_io_scope(secure_io);
    let policy = worth_store_io_scheduler::admit_queue_policy_receipt(
        work.clone(),
        policy_receipt(work.requested_budget()),
    )
    .expect("policy receipt should bind the exact queue work");
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(work, &backend, policy))
        .expect("secure-frame work should admit into a ready queue plan")
}

fn completion_for(
    plan: &QueueExecutionReadyPlan,
    scope: BackendQueueSpeculativeScope,
) -> BackendQueueExecutionCompletion {
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("secure-frame backend posture should admit");
    BackendQueueExecutionCompletion::for_certification(
        plan.backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(1)
    .observe_read_ahead(1, scope)
}

fn secure_frame_backend(
    security: &IoSchedulerSecurityScopeAdmission,
) -> (
    worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
    AdmittedBackendCapabilityWitness,
) {
    let witness = backend_witness();
    let backend = admit_secure_frame_backend_capability_for_scheduler_claim(&witness, security)
        .expect("secure-frame backend should admit");
    (backend, witness)
}

fn backend_witness() -> AdmittedBackendCapabilityWitness {
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
        .expect("secure-frame backend witness should admit")
}

fn io_qos_security_scope_admission() -> IoSchedulerSecurityScopeAdmission {
    let scope = worth_store_security::admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&scope).expect("scheduler security scope should admit")
}

fn speculative_scope(plan: &QueueExecutionReadyPlan) -> BackendQueueSpeculativeScope {
    BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        plan.grouping_basis().key_scope(),
    )
}

fn policy_receipt(budget: BackgroundResourceBudget) -> FoundationalPolicyAdmissionReceipt {
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
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeRead)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("policy claim should build");
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            breadth_units(budget),
            breadth_units(budget),
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            budget.read_ahead_window() as u32,
            budget.read_ahead_window() as u32,
        )
        .finish()
        .expect("policy receipt should build")
}

fn breadth_units(budget: BackgroundResourceBudget) -> u32 {
    (budget.queue_slots() + budget.worker_permits()) as u32
}
