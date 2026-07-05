use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_isolation::publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test;
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;

use crate::{
    admit_backend_capability_for_scheduler_claim, admit_s5_1_security_scope_for_s6_io_qos,
    admit_secure_frame_backend_capability_for_scheduler_claim,
    admit_store_published_s6_io_qos_isolation_readiness, S6IoQosSecurityScopeHandoff,
};

use super::{
    admit_foreground_reservation, admit_foreground_reservation_capacity, BandwidthToken,
    CacheResidencyHint, ForegroundArbitrationDeclaration, ForegroundLaneDeclaration,
    ForegroundLatencyEnvelope, ForegroundReservationAdmissionRequest,
    ForegroundReservationCapacityAdmissionRequest, ForegroundReservationReceipt,
    ForegroundResourceBudget, QueueSlot, ReadAheadWindow, WorkerPermit,
};

pub fn admitted_point_read_reservation_for_certification_test() -> ForegroundReservationReceipt {
    admitted_reservation_for_certification_test(
        ForegroundLaneDeclaration::point_read()
            .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
                "certification-point-read",
                2,
            ))
            .with_budget(point_read_budget()),
    )
}

pub fn admitted_range_read_reservation_for_certification_test() -> ForegroundReservationReceipt {
    admitted_reservation_for_certification_test(
        ForegroundLaneDeclaration::range_read()
            .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
                "certification-range-read",
                2,
            ))
            .with_budget(point_read_budget()),
    )
}

pub fn admitted_secure_frame_read_reservation_for_certification_test(
) -> ForegroundReservationReceipt {
    let lane = ForegroundLaneDeclaration::secure_frame_internal_foreground_read()
        .expect("secure frame internal read lane should be Store-owned")
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-secure-frame-read",
            2,
        ))
        .with_budget(point_read_budget());
    admitted_secure_frame_reservation_for_certification_test(lane)
}

fn admitted_reservation_for_certification_test(
    lane: ForegroundLaneDeclaration,
) -> ForegroundReservationReceipt {
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane.lane());
    let readiness = s6_readiness_admission();
    let security = security_scope_admission();
    let backend_witness = admitted_backend_witness();
    let backend =
        admit_backend_capability_for_scheduler_claim(&backend_witness, lane.backend_requirement())
            .expect("test backend should admit for scheduler claim");
    let capacity = capacity_admission(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity(),
    );

    admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect("test reservation should admit through production path")
}

fn admitted_secure_frame_reservation_for_certification_test(
    lane: ForegroundLaneDeclaration,
) -> ForegroundReservationReceipt {
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane.lane());
    let readiness = s6_readiness_admission();
    let security = security_scope_admission();
    let backend_witness = admitted_backend_witness();
    let backend =
        admit_secure_frame_backend_capability_for_scheduler_claim(&backend_witness, &security)
            .expect("secure-frame backend should admit for scheduler claim");
    let capacity = capacity_admission(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity(),
    );

    admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &readiness,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect("secure-frame reservation should admit through production path")
}

fn s6_readiness_admission() -> crate::IoSchedulerS6ReadinessAdmission {
    let readiness = publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness through production path");
    admit_store_published_s6_io_qos_isolation_readiness(&readiness)
        .expect("scheduler should admit Store-published S.6 readiness")
}

fn capacity_admission(
    lane: ForegroundLaneDeclaration,
    backend: &crate::IoSchedulerBackendCapabilityAdmission,
    readiness: &crate::IoSchedulerS6ReadinessAdmission,
    security: &crate::IoSchedulerS6SecurityScopeAdmission,
    arbitration: ForegroundArbitrationDeclaration,
    requested: ForegroundResourceBudget,
    available: ForegroundResourceBudget,
) -> super::ForegroundReservationCapacityAdmission {
    admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
        super::ForegroundReservationCapacityAuthority::store_owned(),
        lane,
        backend,
        readiness,
        security,
        arbitration,
        requested,
        available,
        policy_receipt(requested, requested),
    ))
    .expect("test capacity should admit through production policy path")
}

fn policy_receipt(
    requested: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
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
    let mut receipt = performance().policy_admission_receipt(claim);
    receipt = add_budget_decision(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        breadth_units(requested),
        breadth_units(admitted),
    );
    receipt = add_budget_decision(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        density_units(requested),
        density_units(admitted),
    );
    receipt = add_budget_decision(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        locality_units(requested),
        locality_units(admitted),
    );
    receipt = add_budget_decision(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness_units(requested),
        freshness_units(admitted),
    );
    receipt
        .finish()
        .expect("policy admission receipt should build")
}

fn add_budget_decision(
    receipt: forge_foundational::FoundationalPolicyAdmissionReceiptBuilder,
    kind: FoundationalPerformanceBudgetKind,
    requested_units: u32,
    admitted_units: u32,
) -> forge_foundational::FoundationalPolicyAdmissionReceiptBuilder {
    if requested_units == 0 && admitted_units == 0 {
        receipt
    } else {
        receipt.budget_decision(kind, requested_units, admitted_units)
    }
}

fn breadth_units(budget: ForegroundResourceBudget) -> u32 {
    (budget.queue_slots() + budget.worker_permits()) as u32
}

fn density_units(budget: ForegroundResourceBudget) -> u32 {
    (budget.bandwidth_tokens() + budget.dirty_page_budget() + budget.cache_residency_hints()) as u32
}

fn locality_units(budget: ForegroundResourceBudget) -> u32 {
    (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits()) as u32
}

fn freshness_units(budget: ForegroundResourceBudget) -> u32 {
    (budget.flush_permits() + budget.sync_debt()) as u32
}

fn admitted_backend_witness() -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
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
        .expect("test backend should admit")
}

fn security_scope_admission() -> crate::IoSchedulerS6SecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness)
        .expect("test S.5.1 security handoff should admit");
    admit_s5_1_security_scope_for_s6_io_qos(handoff)
}

fn point_read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn full_capacity() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(4).unwrap())
        .with_bandwidth(BandwidthToken::bytes(16_384).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(4).unwrap())
        .with_worker_permits(WorkerPermit::new(4).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(4).unwrap())
}
