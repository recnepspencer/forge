use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_isolation::publish_scheduler_isolation_capability_for_certification_test;
use worth_store_security::{
    admitted_store_internal_security_scope_for_io_qos_test, StoreSecurityScopeIdentity,
};

use crate::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_security_scope_for_scheduler,
    admit_store_published_isolation_capability,
};

use super::{
    admit_foreground_reservation, admit_foreground_reservation_capacity, BandwidthToken,
    CacheResidencyHint, DirtyPageBudget, FlushPermit, ForegroundArbitrationDeclaration,
    ForegroundLaneDeclaration, ForegroundLatencyEnvelope, ForegroundReservationAdmissionRequest,
    ForegroundReservationCapacityAdmissionRequest, ForegroundReservationReceipt,
    ForegroundResourceBudget, QueueSlot, ReadAheadWindow, SyncDebt, WorkerPermit, WriteBackWindow,
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

pub fn admitted_point_read_reservation_for_security_scope_for_certification_test(
    security_scope_identity: StoreSecurityScopeIdentity,
) -> ForegroundReservationReceipt {
    let receipt = admitted_point_read_reservation_for_certification_test();
    ForegroundReservationReceipt::admitted(
        receipt.lane(),
        super::ForegroundReservationBackendBasis::new(
            receipt.backend_requirement(),
            receipt.backend_profile(),
            receipt.backend_evidence_class(),
        ),
        receipt.envelope(),
        receipt.arbitration(),
        receipt.counters(),
        security_scope_identity,
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

pub fn admitted_wal_write_reservation_for_certification_test() -> ForegroundReservationReceipt {
    admitted_reservation_for_certification_test(
        ForegroundLaneDeclaration::commit_critical_wal_write()
            .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
                "certification-wal-write",
                2,
            ))
            .with_budget(wal_write_budget()),
    )
}

pub fn admitted_page_write_reservation_for_certification_test() -> ForegroundReservationReceipt {
    admitted_reservation_for_certification_test(
        ForegroundLaneDeclaration::ordinary_page_write()
            .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
                "certification-page-write",
                2,
            ))
            .with_budget(page_write_budget()),
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
    let readiness = io_qos_readiness_admission();
    let security = security_scope_admission();
    let backend_witness = admitted_backend_witness(lane.backend_requirement());
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
    let readiness = io_qos_readiness_admission();
    let security = security_scope_admission();
    let backend_witness = admitted_backend_witness(lane.backend_requirement());
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

fn io_qos_readiness_admission() -> crate::IoSchedulerIsolationAdmission {
    let readiness = publish_scheduler_isolation_capability_for_certification_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness through production path");
    admit_store_published_isolation_capability(&readiness)
        .expect("scheduler should admit Store-published S.6 readiness")
}

fn capacity_admission(
    lane: ForegroundLaneDeclaration,
    backend: &crate::IoSchedulerBackendCapabilityAdmission,
    readiness: &crate::IoSchedulerIsolationAdmission,
    security: &crate::IoSchedulerSecurityScopeAdmission,
    arbitration: ForegroundArbitrationDeclaration,
    requested: ForegroundResourceBudget,
    available: ForegroundResourceBudget,
) -> super::ForegroundReservationCapacityAdmission {
    admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
        lane,
        super::ForegroundReservationCapacityBasis::new(backend, readiness, security),
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
    receipt: worth_foundational::FoundationalPolicyAdmissionReceiptBuilder,
    kind: FoundationalPerformanceBudgetKind,
    requested_units: u32,
    admitted_units: u32,
) -> worth_foundational::FoundationalPolicyAdmissionReceiptBuilder {
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

fn admitted_backend_witness(
    requirement: crate::IoSchedulerBackendCapabilityRequirement,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        backend_evidence_basis(requirement),
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

fn backend_evidence_basis(
    requirement: crate::IoSchedulerBackendCapabilityRequirement,
) -> BackendCapabilityEvidenceBasis {
    match requirement.required_evidence() {
        worth_store_physical_backend::CapabilityEvidenceClass::DeclaredByConfig => {
            BackendCapabilityEvidenceBasis::declared_by_config(1)
        }
        worth_store_physical_backend::CapabilityEvidenceClass::CertifiedBackendProfile
        | worth_store_physical_backend::CapabilityEvidenceClass::ExternallyGuaranteed => {
            BackendCapabilityEvidenceBasis::certified_backend_profile()
        }
        worth_store_physical_backend::CapabilityEvidenceClass::ObservedByProbe => {
            BackendCapabilityEvidenceBasis::observed_by_probe(1)
        }
        worth_store_physical_backend::CapabilityEvidenceClass::EstablishedByFilesystemAdmission => {
            BackendCapabilityEvidenceBasis::established_filesystem_admission_for_certification(1)
        }
        worth_store_physical_backend::CapabilityEvidenceClass::UnverifiableAssumption => {
            BackendCapabilityEvidenceBasis::unverifiable_assumption()
        }
    }
}

fn security_scope_admission() -> crate::IoSchedulerSecurityScopeAdmission {
    let security_scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&security_scope)
        .expect("test security scope should admit for scheduler use")
}

fn point_read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn wal_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_flush_permits(FlushPermit::new(1).unwrap())
        .with_sync_debt(SyncDebt::units(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

fn page_write_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_write_back(WriteBackWindow::pages(1).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

fn full_capacity() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(4).unwrap())
        .with_bandwidth(BandwidthToken::bytes(16_384).unwrap())
        .with_flush_permits(FlushPermit::new(4).unwrap())
        .with_sync_debt(SyncDebt::units(4).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(4).unwrap())
        .with_write_back(WriteBackWindow::pages(4).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(4).unwrap())
        .with_worker_permits(WorkerPermit::new(4).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(4).unwrap())
}
