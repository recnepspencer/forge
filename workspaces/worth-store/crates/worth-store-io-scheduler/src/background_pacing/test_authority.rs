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

use crate::foreground_reservation::{
    admit_foreground_reservation, admit_foreground_reservation_capacity,
    ForegroundArbitrationDeclaration, ForegroundLaneDeclaration, ForegroundLatencyEnvelope,
    ForegroundReservationAdmissionRequest, ForegroundReservationCapacityAdmissionRequest,
    ForegroundResourceBudget,
};
use crate::{
    admit_backend_capability_for_scheduler_claim, admit_background_capacity,
    admit_security_scope_for_scheduler, BackgroundCapacityAdmission,
    BackgroundCapacityAdmissionRequest, BackgroundIoPressureShape, BackgroundResourceBudget,
    IoSchedulerBackendCapabilityAdmission, IoSchedulerSecurityScopeAdmission,
};

mod blob_ingest_outcomes;
mod foreground_budgets;
mod producer_pressure;
mod secure_io;
mod verification_outcomes;
pub use blob_ingest_outcomes::{
    blob_ingest_deferred_background_capacity_for_certification_test,
    blob_ingest_denied_background_capacity_for_certification_test,
    blob_ingest_throttled_background_capacity_for_certification_test,
    blob_ingest_zero_admitted_throttle_background_capacity_for_certification_test,
};
use foreground_budgets::{
    full_foreground_capacity, page_write_budget, point_read_budget, wal_write_budget,
};
pub use producer_pressure::{
    execute_background_pressure_for_certification_test,
    mismatched_background_pressure_denial_for_certification_test,
};
use secure_io::secure_io_for_pressure;
pub use verification_outcomes::{
    verification_deferred_background_capacity_for_certification_test,
    verification_denied_background_capacity_for_certification_test,
    verification_throttled_background_capacity_for_certification_test,
    verification_zero_admitted_throttle_background_capacity_for_certification_test,
};
pub fn blob_ingest_background_capacity_for_certification_test(
    budget: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    blob_ingest_page_write_background_capacity_for_certification_test(budget)
}

pub fn blob_ingest_page_write_background_capacity_for_certification_test(
    budget: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let lane = ForegroundLaneDeclaration::ordinary_page_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-blob-ingest-page-write",
            2,
        ))
        .with_budget(page_write_budget());
    blob_ingest_background_capacity_for_lane(lane, budget)
}

pub fn blob_ingest_wal_write_background_capacity_for_certification_test(
    budget: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let lane = ForegroundLaneDeclaration::commit_critical_wal_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-blob-ingest-wal-write",
            2,
        ))
        .with_budget(wal_write_budget());
    blob_ingest_background_capacity_for_lane(lane, budget)
}

pub fn checkpoint_flush_wal_background_capacity_for_certification_test(
    budget: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let lane = ForegroundLaneDeclaration::commit_critical_wal_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "certification-checkpoint-flush-wal-write",
            2,
        ))
        .with_budget(wal_write_budget());
    background_capacity_for_lane(
        BackgroundIoPressureShape::checkpoint_flush().requesting(budget),
        lane,
        budget,
        budget,
        BackgroundResourceBudget::new(),
    )
}

fn blob_ingest_background_capacity_for_lane(
    lane: ForegroundLaneDeclaration,
    budget: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    background_capacity_for_lane(
        BackgroundIoPressureShape::blob_ingest_pressure().requesting(budget),
        lane,
        budget,
        budget,
        BackgroundResourceBudget::new(),
    )
}

fn background_capacity_for_lane(
    pressure: BackgroundIoPressureShape,
    lane: ForegroundLaneDeclaration,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
) -> BackgroundCapacityAdmission {
    let arbitration = ForegroundArbitrationDeclaration::for_lane(lane.lane());
    let security = security_scope_admission();
    let foreground_backend = backend_admission(lane.backend_requirement());
    let background_backend = backend_admission(pressure.backend_requirement());
    let foreground_capacity =
        admit_foreground_reservation_capacity(ForegroundReservationCapacityAdmissionRequest::new(
            lane,
            crate::foreground_reservation::ForegroundReservationCapacityBasis::new(
                &foreground_backend,
                &security,
            ),
            arbitration,
            lane.requested_budget(),
            full_foreground_capacity(),
            foreground_policy_receipt(lane.requested_budget()),
        ))
        .expect("foreground page-write capacity should admit through S.6");
    let foreground = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
        lane,
        &foreground_backend,
        &security,
        arbitration,
        &foreground_capacity,
    ))
    .into_result()
    .expect("foreground page-write reservation should admit through S.6");

    let request = BackgroundCapacityAdmissionRequest::new(
        pressure,
        &foreground,
        &background_backend,
        background_policy_receipt_for(pressure.requested_budget(), policy_admitted),
    )
    .with_idle_available(idle_available)
    .with_policy_admitted(policy_admitted)
    .with_debt_limit(debt_limit);
    let request = if pressure.secure_scope_required() {
        request.with_secure_io_scope(secure_io_for_pressure(
            pressure,
            &background_backend,
            &security,
        ))
    } else {
        request
    };

    admit_background_capacity(request)
        .expect("blob ingest background capacity should admit through S.6")
}
fn security_scope_admission() -> IoSchedulerSecurityScopeAdmission {
    let security_scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&security_scope)
        .expect("test security scope should admit for scheduler use")
}
fn backend_admission(
    requirement: crate::IoSchedulerBackendCapabilityRequirement,
) -> IoSchedulerBackendCapabilityAdmission {
    let evidence_basis = match requirement {
        crate::IoSchedulerBackendCapabilityRequirement::BufferedFile => {
            BackendCapabilityEvidenceBasis::declared_by_config(2)
        }
        crate::IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync => {
            BackendCapabilityEvidenceBasis::established_filesystem_admission_for_certification(1)
        }
        _ => BackendCapabilityEvidenceBasis::externally_guaranteed(1),
    };
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        evidence_basis,
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering()
            .with_fdatasync_durability(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("test backend should admit");
    admit_backend_capability_for_scheduler_claim(&witness, requirement)
        .expect("scheduler backend should admit")
}

fn foreground_policy_receipt(
    budget: ForegroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    policy_receipt(
        (
            breadth_units(budget.queue_slots(), budget.worker_permits()),
            breadth_units(budget.queue_slots(), budget.worker_permits()),
        ),
        (
            density_units(
                budget.bandwidth_tokens(),
                budget.dirty_page_budget(),
                budget.cache_residency_hints(),
            ),
            density_units(
                budget.bandwidth_tokens(),
                budget.dirty_page_budget(),
                budget.cache_residency_hints(),
            ),
        ),
        (
            locality_units(budget.read_ahead_window(), budget.write_back_window(), 0),
            locality_units(budget.read_ahead_window(), budget.write_back_window(), 0),
        ),
        (
            freshness_units(budget.flush_permits(), budget.sync_debt()),
            freshness_units(budget.flush_permits(), budget.sync_debt()),
        ),
    )
}
fn background_policy_receipt_for(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    policy_receipt(
        (
            breadth_units(requested.queue_slots(), requested.worker_permits()),
            breadth_units(admitted.queue_slots(), admitted.worker_permits()),
        ),
        (
            density_units(
                requested.bandwidth_tokens(),
                requested.dirty_page_budget(),
                requested.cache_residency_hints(),
            ),
            density_units(
                admitted.bandwidth_tokens(),
                admitted.dirty_page_budget(),
                admitted.cache_residency_hints(),
            ),
        ),
        (
            locality_units(
                requested.read_ahead_window(),
                requested.write_back_window(),
                requested.reclaim_permits(),
            ),
            locality_units(
                admitted.read_ahead_window(),
                admitted.write_back_window(),
                admitted.reclaim_permits(),
            ),
        ),
        (
            freshness_units(requested.flush_permits(), requested.sync_debt()),
            freshness_units(admitted.flush_permits(), admitted.sync_debt()),
        ),
    )
}

fn policy_receipt(
    breadth: (u32, u32),
    density: (u32, u32),
    locality: (u32, u32),
    freshness: (u32, u32),
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
    let receipt = performance().policy_admission_receipt(claim);
    let receipt = add_budget(receipt, FoundationalPerformanceBudgetKind::Breadth, breadth);
    let receipt = add_budget(receipt, FoundationalPerformanceBudgetKind::Density, density);
    let receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        locality,
    );
    add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness,
    )
    .finish()
    .expect("policy receipt should build")
}

fn add_budget(
    receipt: worth_foundational::FoundationalPolicyAdmissionReceiptBuilder,
    kind: FoundationalPerformanceBudgetKind,
    units: (u32, u32),
) -> worth_foundational::FoundationalPolicyAdmissionReceiptBuilder {
    if units.0 == 0 && units.1 == 0 {
        receipt
    } else {
        receipt.budget_decision(kind, units.0, units.1)
    }
}

fn breadth_units(queue_slots: u64, worker_permits: u64) -> u32 {
    (queue_slots + worker_permits) as u32
}

fn density_units(bandwidth: u64, dirty_pages: u64, cache: u64) -> u32 {
    (bandwidth + dirty_pages + cache) as u32
}

fn locality_units(read_ahead: u64, write_back: u64, reclaim: u64) -> u32 {
    (read_ahead + write_back + reclaim) as u32
}

fn freshness_units(flush: u64, sync_debt: u64) -> u32 {
    (flush + sync_debt) as u32
}
