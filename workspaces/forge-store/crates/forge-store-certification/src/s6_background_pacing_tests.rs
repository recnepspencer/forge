use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use forge_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_background_capacity,
    admit_background_pacing, admit_store_published_s6_io_qos_isolation_readiness,
    BackgroundCapacityAdmissionRequest, BackgroundDebtKind, BackgroundIdleCapacityLeaseRequest,
    BackgroundIoPressureShape, BackgroundPacingOutcome, BackgroundPacingProgressionDrift,
    BackgroundPacingProgressionEvidence, BackgroundResourceBudget,
    IoSchedulerBackendCapabilityRequirement, QueueSlot,
};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_isolation::publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test;

use crate::{
    certify_s6_background_pacing, S6BackgroundPacingCertificationDenial,
    S6BackgroundPacingOutcomeKind,
};

#[test]
fn s6_background_pacing_certification_preserves_all_phase3_outcomes() {
    let cases = [
        (
            producer_yield_outcome(),
            direct_yield_outcome(),
            S6BackgroundPacingOutcomeKind::Yield,
            None,
        ),
        (
            producer_deferred_outcome(),
            direct_deferred_outcome(),
            S6BackgroundPacingOutcomeKind::Deferred,
            None,
        ),
        (
            producer_denied_outcome(),
            direct_denied_outcome(),
            S6BackgroundPacingOutcomeKind::Denied,
            None,
        ),
        (
            producer_stale_outcome(),
            direct_stale_outcome(),
            S6BackgroundPacingOutcomeKind::StaleRebindRequired,
            None,
        ),
        (
            producer_throttle_outcome(),
            direct_throttle_outcome(),
            S6BackgroundPacingOutcomeKind::Throttled,
            None,
        ),
        (
            producer_admitted_with_debt_outcome(),
            direct_admitted_with_debt_outcome(),
            S6BackgroundPacingOutcomeKind::AdmittedWithDebt,
            Some(BackgroundDebtKind::RepairPressure),
        ),
        (
            producer_violation_outcome(),
            direct_violation_outcome(),
            S6BackgroundPacingOutcomeKind::Violation,
            Some(BackgroundDebtKind::RepairPressure),
        ),
    ];

    for (actual, expected, expected_kind, expected_debt) in cases {
        let expected_counters = counters_for(expected);
        let evidence = certify_s6_background_pacing(actual, expected)
            .expect("independently built equivalent background pacing should certify");
        assert_eq!(evidence.outcome(), expected_kind);
        assert_eq!(evidence.counters(), expected_counters);
        assert_eq!(evidence.debt().map(|debt| debt.kind()), expected_debt);
    }
}

#[test]
fn s6_background_pacing_certification_denies_mismatched_outcomes() {
    assert_eq!(
        certify_s6_background_pacing(producer_yield_outcome(), direct_throttle_outcome()),
        Err(S6BackgroundPacingCertificationDenial::OutcomeMismatch)
    );
}

fn producer_yield_outcome() -> BackgroundPacingOutcome {
    admit_background_pacing(request(producer_repair_pressure()).with_foreground_pressure_events(1))
}

fn direct_yield_outcome() -> BackgroundPacingOutcome {
    admit_background_pacing(request(direct_repair_pressure()).with_foreground_pressure_events(1))
}

fn producer_deferred_outcome() -> BackgroundPacingOutcome {
    deferred_outcome(producer_repair_pressure())
}

fn direct_deferred_outcome() -> BackgroundPacingOutcome {
    deferred_outcome(direct_repair_pressure())
}

fn producer_denied_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        producer_repair_pressure(),
        BackgroundPacingProgressionDrift::DeniedReadinessCounters,
    )
}

fn direct_denied_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        direct_repair_pressure(),
        BackgroundPacingProgressionDrift::DeniedReadinessCounters,
    )
}

fn producer_stale_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        producer_repair_pressure(),
        BackgroundPacingProgressionDrift::StaleReadinessCounters,
    )
}

fn direct_stale_outcome() -> BackgroundPacingOutcome {
    drifted_outcome(
        direct_repair_pressure(),
        BackgroundPacingProgressionDrift::StaleReadinessCounters,
    )
}

fn deferred_outcome(pressure: BackgroundIoPressureShape) -> BackgroundPacingOutcome {
    let requested = pressure.requested_budget();
    admit_background_pacing(request_with(
        pressure,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&s6_readiness()),
    ))
}

fn drifted_outcome(
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

fn producer_throttle_outcome() -> BackgroundPacingOutcome {
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    admit_background_pacing(request_with(
        producer_repair_pressure(),
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&s6_readiness()),
    ))
}

fn direct_throttle_outcome() -> BackgroundPacingOutcome {
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    admit_background_pacing(request_with(
        direct_repair_pressure(),
        admitted,
        admitted,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&s6_readiness()),
    ))
}

fn producer_admitted_with_debt_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(producer_repair_pressure(), false)
}

fn direct_admitted_with_debt_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(direct_repair_pressure(), false)
}

fn producer_violation_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(producer_repair_pressure(), true)
}

fn direct_violation_outcome() -> BackgroundPacingOutcome {
    admitted_or_violation(direct_repair_pressure(), true)
}

fn admitted_or_violation(
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
        BackgroundPacingProgressionEvidence::current(&s6_readiness()),
    );
    if late_yield {
        request = request.with_foreground_pressure_events(1).with_late_yield();
    }
    admit_background_pacing(request)
}

fn request(pressure: BackgroundIoPressureShape) -> BackgroundIdleCapacityLeaseRequest {
    let requested = pressure.requested_budget();
    request_with(
        pressure,
        requested,
        requested,
        BackgroundResourceBudget::new(),
        BackgroundPacingProgressionEvidence::current(&s6_readiness()),
    )
}

fn request_with(
    pressure: BackgroundIoPressureShape,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
    progression: BackgroundPacingProgressionEvidence,
) -> BackgroundIdleCapacityLeaseRequest {
    let foreground = Box::leak(Box::new(
        admitted_point_read_reservation_for_certification_test(),
    ));
    let backend = Box::leak(Box::new(backend_admission()));
    let readiness = Box::leak(Box::new(s6_readiness()));
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
        .with_progression_evidence(progression),
    )
    .expect("background capacity should admit");
    BackgroundIdleCapacityLeaseRequest::new(capacity)
}

fn producer_repair_pressure() -> BackgroundIoPressureShape {
    BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        forge_store_operations::repair_background_pressure_shape(2),
    )
}

fn direct_repair_pressure() -> BackgroundIoPressureShape {
    BackgroundIoPressureShape::repair_scan()
        .requesting(producer_repair_pressure().requested_budget())
}

fn counters_for(
    outcome: BackgroundPacingOutcome,
) -> forge_store_io_scheduler::BackgroundPacingCounterSnapshot {
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

fn progression_drift(
    drift: BackgroundPacingProgressionDrift,
) -> BackgroundPacingProgressionEvidence {
    let readiness = s6_readiness();
    BackgroundPacingProgressionEvidence::from_readiness_counter_drift(
        &readiness,
        mismatched_counters(readiness.counters()),
        drift,
    )
    .expect("mismatched readiness counters should produce progression evidence")
}

fn mismatched_counters(
    counters: forge_store_io_scheduler::IoSchedulerS6CounterSnapshot,
) -> forge_store_io_scheduler::IoSchedulerS6CounterSnapshot {
    let alternate = publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(
        counters.wait_count() + 3,
        counters.retry_count() + 1,
    )
    .expect("alternate S.6 readiness should publish");
    admit_store_published_s6_io_qos_isolation_readiness(&alternate)
        .expect("alternate scheduler readiness should admit")
        .counters()
}

fn backend_admission() -> forge_store_io_scheduler::IoSchedulerBackendCapabilityAdmission {
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

fn s6_readiness() -> forge_store_io_scheduler::IoSchedulerS6ReadinessAdmission {
    let readiness = publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness");
    admit_store_published_s6_io_qos_isolation_readiness(&readiness)
        .expect("scheduler should admit readiness")
}

fn policy_receipt(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> forge_foundational::FoundationalPolicyAdmissionReceipt {
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

fn performance_claim() -> forge_foundational::FoundationalPolicyAdmissionPerformanceClaim {
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

fn add_budget(
    receipt: forge_foundational::FoundationalPolicyAdmissionReceiptBuilder,
    kind: FoundationalPerformanceBudgetKind,
    requested: u32,
    admitted: u32,
) -> forge_foundational::FoundationalPolicyAdmissionReceiptBuilder {
    if requested == 0 && admitted == 0 {
        receipt
    } else {
        receipt.budget_decision(kind, requested, admitted)
    }
}
