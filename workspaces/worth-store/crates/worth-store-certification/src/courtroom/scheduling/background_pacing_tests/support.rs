use super::*;

pub(super) fn producer_yield_outcome() -> BackgroundPacingOutcome {
    admit_background_pacing(request(producer_repair_pressure()).with_foreground_pressure_events(1))
}

pub(super) fn direct_yield_outcome() -> BackgroundPacingOutcome {
    admit_background_pacing(request(direct_repair_pressure()).with_foreground_pressure_events(1))
}

pub(super) fn producer_deferred_outcome() -> BackgroundPacingOutcome {
    deferred_outcome(producer_repair_pressure())
}

pub(super) fn direct_deferred_outcome() -> BackgroundPacingOutcome {
    deferred_outcome(direct_repair_pressure())
}

pub(super) fn producer_denied_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        producer_repair_pressure(),
        BackgroundPacingProgressionDrift::DeniedReadinessCounters,
    )
}

pub(super) fn direct_denied_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        direct_repair_pressure(),
        BackgroundPacingProgressionDrift::DeniedReadinessCounters,
    )
}

pub(super) fn producer_stale_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        producer_repair_pressure(),
        BackgroundPacingProgressionDrift::StaleReadinessCounters,
    )
}

pub(super) fn direct_stale_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        direct_repair_pressure(),
        BackgroundPacingProgressionDrift::StaleReadinessCounters,
    )
}

pub(super) fn deferred_outcome(pressure: BackgroundIoPressureShape) -> BackgroundPacingOutcome {
    let requested = pressure.requested_budget();
    admit_background_pacing(request_with(
        pressure,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&io_qos_readiness()),
    ))
}

pub(super) fn drifted_outcome(
    pressure: BackgroundIoPressureShape,
    drift: BackgroundPacingProgressionDrift,
) -> BackgroundPacingOutcome {
    let requested = pressure.requested_budget();
    admit_background_pacing(request_with(
        pressure,
        requested,
        requested,
        BackgroundResourceBudget::new(),
        progression_drift(drift),
    ))
}

pub(super) fn producer_throttle_outcome() -> BackgroundPacingOutcome {
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    admit_background_pacing(request_with(
        producer_repair_pressure(),
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&io_qos_readiness()),
    ))
}

pub(super) fn direct_throttle_outcome() -> BackgroundPacingOutcome {
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    admit_background_pacing(request_with(
        direct_repair_pressure(),
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&io_qos_readiness()),
    ))
}

pub(super) fn producer_admitted_with_debt_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(producer_repair_pressure(), false)
}

pub(super) fn direct_admitted_with_debt_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(direct_repair_pressure(), false)
}

pub(super) fn producer_violation_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(producer_repair_pressure(), true)
}

pub(super) fn direct_violation_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(direct_repair_pressure(), true)
}

pub(super) fn admitted_or_violation(
    pressure: BackgroundIoPressureShape,
    late_yield: bool,
) -> BackgroundPacingOutcome {
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    let requested = pressure.requested_budget();
    let mut request = request_with(
        pressure,
        admitted,
        admitted,
        requested,
        BackgroundPacingProgressionEvidence::current(&io_qos_readiness()),
    );
    if late_yield {
        request = request.with_foreground_pressure_events(1).with_late_yield();
    }
    admit_background_pacing(request)
}

pub(super) fn request(pressure: BackgroundIoPressureShape) -> BackgroundIdleCapacityLeaseRequest {
    let requested = pressure.requested_budget();
    request_with(
        pressure,
        requested,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&io_qos_readiness()),
    )
}

pub(super) fn request_with(
    pressure: BackgroundIoPressureShape,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
    progression: BackgroundPacingProgressionEvidence,
) -> BackgroundIdleCapacityLeaseRequest {
    let security = Box::leak(Box::new(io_qos_security_scope_admission()));
    let foreground = Box::leak(Box::new(
        admitted_point_read_reservation_for_security_scope_for_certification_test(
            security.permission().identity(),
        ),
    ));
    let backend = Box::leak(Box::new(backend_admission()));
    let readiness = Box::leak(Box::new(io_qos_readiness()));
    let secure_io = admit_secure_io_scope_for_scheduler(SecureIoPreservationRequest::new(
        SecureIoOperation::RepairScan,
        security,
        backend,
    ))
    .expect("repair pressure secure I/O should admit");
    let capacity = admit_background_capacity(
        BackgroundCapacityAdmissionRequest::new(
            pressure,
            foreground,
            backend,
            readiness,
            policy_receipt(pressure.requested_budget(), policy_admitted),
        )
        .with_idle_available(idle_available)
        .with_policy_admitted(policy_admitted)
        .with_debt_limit(debt_limit)
        .with_progression_evidence(progression)
        .with_secure_io_scope(secure_io),
    )
    .expect("background capacity should admit");
    BackgroundIdleCapacityLeaseRequest::new(capacity)
}

pub(super) fn producer_repair_pressure() -> BackgroundIoPressureShape {
    BackgroundIoPressureShape::from_background_pressure_declaration(
        worth_store_operations::repair_background_pressure_shape(2),
    )
}

pub(super) fn direct_repair_pressure() -> BackgroundIoPressureShape {
    BackgroundIoPressureShape::repair_scan()
        .requesting(producer_repair_pressure().requested_budget())
}

pub(super) fn counters_for(
    outcome: &BackgroundPacingOutcome,
) -> worth_store_io_scheduler::BackgroundPacingCounterSnapshot {
    match outcome {
        BackgroundPacingOutcome::Yield(value) => value.counters(),
        BackgroundPacingOutcome::Deferred(value) => value.counters(),
        BackgroundPacingOutcome::Denied(value) => value.counters(),
        BackgroundPacingOutcome::StaleRebindRequired(value) => value.counters(),
        BackgroundPacingOutcome::Throttled(value) => value.counters(),
        BackgroundPacingOutcome::AdmittedWithDebt(value) => value.counters(),
        BackgroundPacingOutcome::Violation(value) => value.counters(),
    }
}

pub(super) fn progression_drift(
    drift: BackgroundPacingProgressionDrift,
) -> BackgroundPacingProgressionEvidence {
    let readiness = io_qos_readiness();
    BackgroundPacingProgressionEvidence::from_readiness_counter_drift(
        &readiness,
        mismatched_counters(readiness.counters()),
        drift,
    )
    .expect("mismatched readiness counters should produce progression evidence")
}

pub(super) fn mismatched_counters(
    counters: worth_store_io_scheduler::IoSchedulerIsolationCounterSnapshot,
) -> worth_store_io_scheduler::IoSchedulerIsolationCounterSnapshot {
    let alternate = publish_scheduler_isolation_capability_for_certification_test(
        counters.wait_count() + 3,
        counters.retry_count() + 1,
    )
    .expect("alternate S.6 readiness should publish");
    admit_store_published_isolation_capability(&alternate)
        .expect("alternate scheduler readiness should admit")
        .counters()
}

pub(super) fn backend_admission() -> worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission
{
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
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend should admit");
    admit_backend_capability_for_scheduler_claim(
        &witness,
        IoSchedulerBackendCapabilityRequirement::DirectIo,
    )
    .expect("scheduler should admit backend")
}

pub(super) fn io_qos_readiness() -> worth_store_io_scheduler::IoSchedulerIsolationAdmission {
    let readiness = publish_scheduler_isolation_capability_for_certification_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness");
    admit_store_published_isolation_capability(&readiness)
        .expect("scheduler should admit readiness")
}

pub(super) fn io_qos_security_scope_admission(
) -> worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission {
    let security_scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&security_scope)
        .expect("Store security scope should admit for scheduler use")
}

pub(super) fn policy_receipt(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance_claim();
    let mut receipt = performance().policy_admission_receipt(claim);
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        (requested.queue_slots() + requested.worker_permits()) as u32,
        (admitted.queue_slots() + admitted.worker_permits()) as u32,
    );
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        requested.bandwidth_tokens() as u32,
        admitted.bandwidth_tokens() as u32,
    );
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        requested.read_ahead_window() as u32,
        admitted.read_ahead_window() as u32,
    );
    receipt.finish().expect("policy receipt should build")
}

pub(super) fn performance_claim() -> worth_foundational::FoundationalPolicyAdmissionPerformanceClaim
{
    performance()
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
        .expect("policy claim should build")
}

pub(super) fn add_budget(
    receipt: worth_foundational::FoundationalPolicyAdmissionReceiptBuilder,
    kind: FoundationalPerformanceBudgetKind,
    requested: u32,
    admitted: u32,
) -> worth_foundational::FoundationalPolicyAdmissionReceiptBuilder {
    if requested == 0 && admitted == 0 {
        receipt
    } else {
        receipt.budget_decision(kind, requested, admitted)
    }
}
