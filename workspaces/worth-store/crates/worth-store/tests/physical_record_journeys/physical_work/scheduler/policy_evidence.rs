use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, FoundationalPolicyAdmissionReceipt,
};
use worth_store_io_scheduler::BackgroundResourceBudget;

pub(crate) fn policy_receipt(
    budget: BackgroundResourceBudget,
) -> FoundationalPolicyAdmissionReceipt {
    policy_receipt_with_breadth_delta(budget, 0)
}

pub(crate) fn mismatched_policy_receipt(
    budget: BackgroundResourceBudget,
) -> FoundationalPolicyAdmissionReceipt {
    policy_receipt_with_breadth_delta(budget, 1)
}

fn policy_receipt_with_breadth_delta(
    budget: BackgroundResourceBudget,
    breadth_delta: u32,
) -> FoundationalPolicyAdmissionReceipt {
    policy_receipt_for(
        budget,
        breadth_delta,
        FoundationalPerformanceWorkClass::AuthoritativeMutation,
    )
}

pub(crate) fn policy_receipt_for(
    budget: BackgroundResourceBudget,
    breadth_delta: u32,
    work_class: FoundationalPerformanceWorkClass,
) -> FoundationalPolicyAdmissionReceipt {
    policy_receipt_with_breadth_posture(budget, breadth_delta, work_class, false)
}

pub(crate) fn exhausted_policy_receipt(
    budget: BackgroundResourceBudget,
    work_class: FoundationalPerformanceWorkClass,
) -> FoundationalPolicyAdmissionReceipt {
    policy_receipt_with_breadth_posture(budget, 0, work_class, true)
}

fn policy_receipt_with_breadth_posture(
    budget: BackgroundResourceBudget,
    breadth_delta: u32,
    work_class: FoundationalPerformanceWorkClass,
    exhaust_breadth: bool,
) -> FoundationalPolicyAdmissionReceipt {
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
        .include_work(work_class)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    let amount = |kind| match kind {
        FoundationalPerformanceBudgetKind::Breadth => {
            (budget.queue_slots() + budget.worker_permits()) as u32 + breadth_delta
        }
        FoundationalPerformanceBudgetKind::Density => {
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32
        }
        FoundationalPerformanceBudgetKind::Locality => {
            (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits())
                as u32
        }
        FoundationalPerformanceBudgetKind::FreshnessSensitive => 0,
    };
    let receipt = performance().policy_admission_receipt(claim);
    [
        FoundationalPerformanceBudgetKind::Breadth,
        FoundationalPerformanceBudgetKind::Density,
        FoundationalPerformanceBudgetKind::Locality,
    ]
    .into_iter()
    .fold(receipt, |receipt, kind| {
        let units = amount(kind);
        if units == 0 {
            receipt
        } else {
            let admitted = if exhaust_breadth && kind == FoundationalPerformanceBudgetKind::Breadth
            {
                0
            } else {
                units
            };
            receipt.budget_decision(kind, units, admitted)
        }
    })
    .finish()
    .unwrap()
}
