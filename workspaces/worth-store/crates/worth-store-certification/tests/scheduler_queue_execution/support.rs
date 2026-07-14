use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_secure_io_scope_for_scheduler, admit_security_scope_for_scheduler,
    lower_buffer_pool_queue_declaration, BackgroundResourceBudget, BandwidthToken,
    CacheResidencyHint, QueueExecutionAdmissionRequest, QueueSlot, ReadAheadWindow,
    SecureIoOperation, SecureIoPostureRequirement, SecureIoPreservationRequest, WorkerPermit,
};
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendQueueExecutionBudgetBinding,
    BackendQueueExecutionPlanBinding, BackendQueueExecutionReplayBinding, BackendRebindTriggers,
    BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

pub(super) fn admitted_plan() -> worth_store_io_scheduler::QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        7,
        QueueProducerResourceShape::new()
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
    let scope = scheduler_security_scope();
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
pub(super) fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

pub(super) fn backend_witness() -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
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

pub(super) fn scheduler_security_scope(
) -> worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission {
    let scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&scope).expect("scheduler security scope should admit")
}

pub(super) fn secure_backend_binding(
    scope: worth_store_security::StoreSecurityScopeIdentity,
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
        worth_store_physical_backend::CapabilityEvidenceClass::CertifiedBackendProfile,
        0,
    )
}

pub(super) fn policy_receipt(
    budget: BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
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
