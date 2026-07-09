use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, FoundationalPolicyAdmissionReceipt,
};
use worth_store_budgets::CounterEvidenceStrength;
use worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use worth_store_certification::S6LatencyInterferenceEvidence;
use worth_store_contracts::S6QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_s5_1_security_scope_for_s6_io_qos, admit_secure_io_scope_for_scheduler,
    assess_queue_latency_envelope, execute_ready_queue_plan, lower_buffer_pool_queue_declaration,
    BackgroundResourceBudget, CacheResidencyHint, InterferenceCounterName,
    InterferenceCounterRequirement, LatencyEnvelopeClaim, QueueExecutionReadyPlan, QueueSlot,
    ReadAheadWindow, S6IoQosSecurityScopeHandoff, SecureIoOperation, SecureIoPostureRequirement,
    SecureIoPreservationRequest, WorkerPermit,
};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendQueueSpeculativeScope, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};

#[test]
fn real_scheduler_assessment_certifies_without_laundering_sampled_rows() {
    let evidence = certified_latency_evidence(4);
    let queue_depth = evidence
        .rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::QueuePeakDepth)
        .expect("scheduler assessment should carry queue-depth row");

    assert_eq!(queue_depth.value(), 4);
    assert_eq!(queue_depth.strength(), CounterEvidenceStrength::Sampled);
    assert!(!receipt_rows(&evidence)
        .iter()
        .any(|(name, _)| name == "store.s6.queue.peak-depth"));
    assert!(receipt_rows(&evidence)
        .iter()
        .any(|(name, count)| name == "store.s6.queue.submitted-units"
            && *count == point_read_units(point_read_budget())));
    assert_eq!(
        receipt_rows(&evidence).len(),
        evidence
            .rows()
            .iter()
            .filter(|row| row.strength() == CounterEvidenceStrength::Exact)
            .count()
    );
}

#[test]
fn certification_replay_preserves_policy_counter_and_proof_topology() {
    let first = certified_latency_evidence(3);
    let second = certified_latency_evidence(3);

    assert_eq!(first.status(), second.status());
    assert_eq!(first.rows(), second.rows());
    assert_eq!(receipt_rows(&first), receipt_rows(&second));
}

fn certified_latency_evidence(queue_depth: u32) -> S6LatencyInterferenceEvidence {
    let plan = admitted_read_ahead_plan();
    let replay_identity = plan.replay_identity();
    let lane = plan.work().class();
    let completion = completion_for(&plan, queue_depth);
    let outcome = execute_ready_queue_plan(plan, completion);
    let claim =
        LatencyEnvelopeClaim::for_queue_execution(replay_identity, "s6-profile/posix-file", lane)
            .require_counter(InterferenceCounterRequirement::queue_depth());
    let assessment = assess_queue_latency_envelope(&claim, &outcome)
        .expect("real queue execution should assess latency evidence");

    S6LatencyInterferenceEvidence::from_assessment(&assessment)
        .expect("scheduler assessment should certify")
}

fn receipt_rows(evidence: &S6LatencyInterferenceEvidence) -> Vec<(String, u64)> {
    evidence
        .counter_backed_receipt()
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str().to_owned(), row.observed_count()))
        .collect()
}

fn admitted_read_ahead_plan() -> QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        7,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
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
    let security = s6_security_scope_admission();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &security, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("read-ahead secure-I/O should admit");
    let work = work.with_secure_io_scope(secure_io);
    admit_queue_execution_plan(
        worth_store_io_scheduler::QueueExecutionAdmissionRequest::new(
            work,
            &backend,
            policy_receipt(budget),
        ),
    )
    .expect("queue work should admit")
}

fn completion_for(
    plan: &QueueExecutionReadyPlan,
    queue_depth: u32,
) -> BackendQueueExecutionCompletion {
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("backend posture should admit");
    BackendQueueExecutionCompletion::for_certification(
        plan.backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(queue_depth)
    .observe_read_ahead(1, speculative_scope(plan))
}

fn speculative_scope(plan: &QueueExecutionReadyPlan) -> BackendQueueSpeculativeScope {
    BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        plan.grouping_basis().key_scope(),
    )
}

fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn point_read_units(budget: BackgroundResourceBudget) -> u64 {
    budget
        .queue_slots()
        .saturating_add(budget.read_ahead_window())
        .saturating_add(budget.worker_permits())
        .saturating_add(budget.cache_residency_hints())
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
        .expect("backend witness should admit")
}

fn s6_security_scope_admission() -> worth_store_io_scheduler::IoSchedulerS6SecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        worth_store_security::admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness)
        .expect("S.5.1 readiness should hand off to S.6");
    admit_s5_1_security_scope_for_s6_io_qos(handoff)
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
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("policy claim should build");
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
        .expect("policy receipt should build")
}
