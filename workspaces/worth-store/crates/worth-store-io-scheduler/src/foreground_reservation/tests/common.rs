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
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use crate::{
    admit_backend_capability_for_scheduler_claim, IoSchedulerBackendCapabilityRequirement,
};

use super::super::*;

pub(super) fn admit_point_read_reservation() -> ForegroundReservationReceipt {
    let security = io_qos_security_scope_admission();
    let backend = backend_admission(IoSchedulerBackendCapabilityRequirement::DirectIo);
    let lane = point_read_lane();
    let arbitration = ForegroundArbitrationDeclaration::for_lane(ForegroundIoLaneKind::PointRead);
    let capacity = capacity_admission(
        lane,
        &backend,
        &security,
        arbitration,
        lane.requested_budget(),
        full_capacity_budget(),
    );

    admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &backend,
        &security,
        arbitration,
        &capacity,
    ))
    .into_result()
    .expect("point read reservation should admit")
}

pub(super) fn point_read_lane() -> ForegroundLaneDeclaration {
    ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "point-read",
            2,
        ))
        .with_budget(read_budget())
}

pub(super) fn read_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

pub(super) fn full_capacity_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(8).unwrap())
        .with_bandwidth(BandwidthToken::bytes(1_048_576).unwrap())
        .with_flush_permits(FlushPermit::new(8).unwrap())
        .with_sync_debt(SyncDebt::units(8).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(8).unwrap())
        .with_write_back(WriteBackWindow::pages(8).unwrap())
        .with_dirty_pages(DirtyPageBudget::pages(8).unwrap())
        .with_worker_permits(WorkerPermit::new(8).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(8).unwrap())
        .with_reclaim_permits(ReclaimPermit::new(8).unwrap())
}

pub(super) fn capacity_admission(
    lane: ForegroundLaneDeclaration,
    backend: &crate::IoSchedulerBackendCapabilityAdmission,
    security: &crate::IoSchedulerSecurityScopeAdmission,
    arbitration: ForegroundArbitrationDeclaration,
    requested: ForegroundResourceBudget,
    available: ForegroundResourceBudget,
) -> ForegroundReservationCapacityAdmission {
    admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
        lane,
        ForegroundReservationCapacityBasis::new(backend, security),
        arbitration,
        requested,
        available,
        policy_receipt(requested, requested),
    ))
    .expect("capacity should admit through production policy path")
}

pub(super) fn policy_receipt(
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

pub(super) fn backend_admission(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> crate::IoSchedulerBackendCapabilityAdmission {
    let witness = admitted_backend_witness_for(requirement);
    admit_backend_capability_for_scheduler_claim(&witness, requirement)
        .expect("backend capability should admit for scheduler claim")
}

pub(super) fn admitted_backend_witness(
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    admitted_backend_witness_for(IoSchedulerBackendCapabilityRequirement::DirectIo)
}

fn admitted_backend_witness_for(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        backend_evidence_basis_for(requirement),
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

const fn backend_evidence_basis_for(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> BackendCapabilityEvidenceBasis {
    match requirement {
        IoSchedulerBackendCapabilityRequirement::BufferedFile => {
            BackendCapabilityEvidenceBasis::declared_by_config(2)
        }
        _ => BackendCapabilityEvidenceBasis::certified_backend_profile(),
    }
}

pub(super) fn io_qos_security_scope_admission() -> crate::IoSchedulerSecurityScopeAdmission {
    io_qos_security_scope_admission_from(admitted_store_internal_security_scope_for_io_qos_test())
}

fn io_qos_security_scope_admission_from(
    scope: worth_store_security::StoreAdmittedSecurityScope,
) -> crate::IoSchedulerSecurityScopeAdmission {
    crate::admit_security_scope_for_scheduler(&scope)
        .expect("test security scope should admit for scheduler use")
}
