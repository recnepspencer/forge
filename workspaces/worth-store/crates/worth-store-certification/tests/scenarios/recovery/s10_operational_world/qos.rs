use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, FoundationalPolicyAdmissionReceipt,
};
use worth_store_certification::courtroom::operational_recovery::S10OperationalQosEvidence;
use worth_store_certification::S6LatencyInterferenceEvidence;
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_background_pacing,
    admit_queue_execution_plan, admit_secure_io_scope_for_scheduler,
    admit_security_scope_for_scheduler, assess_queue_latency_envelope, execute_ready_queue_plan,
    lower_buffer_pool_queue_declaration,
    verification_deferred_background_capacity_for_certification_test,
    BackgroundIdleCapacityLeaseRequest, BackgroundInterferenceEvidence, BackgroundIoPressureClass,
    BackgroundResourceBudget, CacheResidencyHint, LatencyEnvelopeClaim, QueueExecutionReadyPlan,
    QueueSlot, QueueWorkClass, ReadAheadWindow, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationRequest, WorkerPermit,
};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};

pub fn operational_qos() -> S10OperationalQosEvidence {
    let plan = admitted_read_ahead_plan();
    let replay_identity = plan.replay_identity().clone();
    let lane = plan.work().class();
    let completion = completion_for(&plan);
    let outcome = execute_ready_queue_plan(plan, completion);
    let claim =
        LatencyEnvelopeClaim::for_queue_execution(replay_identity, "s10-profile/posix-file", lane);
    let assessment = assess_queue_latency_envelope(&claim, &outcome).unwrap();
    let queue = S6LatencyInterferenceEvidence::from_assessment(&assessment).unwrap();
    let requested = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    let capacity = verification_deferred_background_capacity_for_certification_test(requested);
    let background = BackgroundInterferenceEvidence::from_background_pacing_outcome(
        "s10-profile/posix-file",
        QueueWorkClass::Background(BackgroundIoPressureClass::VerificationPressure),
        admit_background_pacing(BackgroundIdleCapacityLeaseRequest::new(capacity)),
    );
    S10OperationalQosEvidence::from_interference_evidence(queue, background).unwrap()
}

fn admitted_read_ahead_plan() -> QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = worth_store_test_support::read_ahead_declaration_for_real_pool(
        reservation.security_scope_identity(),
        7,
        QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
    let work = lower_buffer_pool_queue_declaration(producer, reservation).unwrap();
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        work.backend_requirement(),
    )
    .unwrap();
    let scope = worth_store_security::admitted_store_internal_security_scope_for_io_qos_test();
    let security = admit_security_scope_for_scheduler(&scope).unwrap();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &security, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .unwrap();
    let work = work.with_secure_io_scope(secure_io);
    let policy =
        worth_store_io_scheduler::admit_queue_policy_receipt(work.clone(), policy_receipt(budget))
            .expect("policy receipt should bind the exact queue work");
    admit_queue_execution_plan(
        worth_store_io_scheduler::QueueExecutionAdmissionRequest::new(work, &backend, policy),
    )
    .unwrap()
}

fn completion_for(plan: &QueueExecutionReadyPlan) -> BackendQueueExecutionCompletion {
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .unwrap();
    BackendQueueExecutionCompletion::for_certification(
        plan.backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(4)
    .observe_read_ahead(
        1,
        worth_store_physical_backend::BackendQueueSpeculativeScope::admitted(
            plan.grouping_basis().security_scope_identity(),
            plan.grouping_basis().tenant_scope(),
            plan.grouping_basis().key_scope(),
        ),
    )
}

fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
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
        .unwrap()
}

fn policy_receipt(budget: BackgroundResourceBudget) -> FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::TraversalLocal)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeRead)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            budget.queue_slots().saturating_add(budget.worker_permits()) as u32,
            budget.queue_slots().saturating_add(budget.worker_permits()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Density,
            budget.cache_residency_hints() as u32,
            budget.cache_residency_hints() as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            budget.read_ahead_window() as u32,
            budget.read_ahead_window() as u32,
        )
        .finish()
        .unwrap()
}
