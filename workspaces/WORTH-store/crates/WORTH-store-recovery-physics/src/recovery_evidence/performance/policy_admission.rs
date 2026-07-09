use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass, FoundationalPolicyAdmissionReceipt,
};

use crate::RecoveryCounterSnapshot;

pub(crate) fn policy_admission_receipt(
    counters: RecoveryCounterSnapshot,
) -> FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::RestoreRecovery)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::RebuildCapable)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::RecoveryOnly)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("recovery policy-admission performance claim is legal");
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            counters.scanned_segments() as u32,
            counters.scanned_segments() as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            counters.page_redos() as u32,
            counters.page_redos() as u32,
        )
        .finish()
        .expect("recovery policy-admission performance receipt is legal")
}
