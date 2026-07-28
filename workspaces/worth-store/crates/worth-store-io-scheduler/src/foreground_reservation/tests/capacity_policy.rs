use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

use super::super::*;

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
