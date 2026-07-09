use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

use crate::foreground_reservation::ForegroundResourceBudget;
use crate::BackgroundResourceBudget;

pub(super) fn foreground_policy_receipt(
    requested: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    build_policy_receipt(
        breadth_units(requested.queue_slots(), requested.worker_permits()),
        breadth_units(admitted.queue_slots(), admitted.worker_permits()),
        density_units(
            requested.bandwidth_tokens(),
            0,
            requested.cache_residency_hints(),
        ),
        density_units(
            admitted.bandwidth_tokens(),
            0,
            admitted.cache_residency_hints(),
        ),
        locality_units(requested.read_ahead_window(), 0, 0),
        locality_units(admitted.read_ahead_window(), 0, 0),
        (
            freshness_units(requested.flush_permits(), requested.sync_debt()),
            freshness_units(admitted.flush_permits(), admitted.sync_debt()),
        ),
    )
}

pub(super) fn background_policy_receipt(
    requested: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    build_policy_receipt(
        breadth_units(requested.queue_slots(), requested.worker_permits()),
        breadth_units(admitted.queue_slots(), admitted.worker_permits()),
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
        (
            freshness_units(requested.flush_permits(), requested.sync_debt()),
            freshness_units(admitted.flush_permits(), admitted.sync_debt()),
        ),
    )
}

fn build_policy_receipt(
    requested_breadth: u32,
    admitted_breadth: u32,
    requested_density: u32,
    admitted_density: u32,
    requested_locality: u32,
    admitted_locality: u32,
    freshness: (u32, u32),
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance_claim();
    let mut receipt = performance().policy_admission_receipt(claim);
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        requested_breadth,
        admitted_breadth,
    );
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        requested_density,
        admitted_density,
    );
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        requested_locality,
        admitted_locality,
    );
    receipt = add_budget(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness.0,
        freshness.1,
    );
    receipt.finish().expect("policy receipt should build")
}

fn performance_claim() -> worth_foundational::FoundationalPolicyAdmissionPerformanceClaim {
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
