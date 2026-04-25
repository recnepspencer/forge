use super::epochs::SupportTrustEpoch;
use super::performance::SupportTrustPerformancePlan;
use super::taxonomy::{SupportRoleTrustPosture, SupportTrustClass, SupportTrustDowngradeReason};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustClassificationCostSurface {
    classified_artifacts: u64,
    receipts_consumed: u64,
    drift_checks_performed: u64,
    equivalence_checks_performed: u64,
    index_probes: u64,
    allocation_count: u64,
    clone_count: u64,
    stale_rejection_count: u64,
    coverage_drift_count: u64,
    placement_advisory_count: u64,
    global_scan_debt_count: u64,
}

impl SupportTrustClassificationCostSurface {
    pub fn new(
        classified_artifacts: u64,
        receipts_consumed: u64,
        drift_checks_performed: u64,
        equivalence_checks_performed: u64,
        index_probes: u64,
        allocation_count: u64,
        clone_count: u64,
        stale_rejection_count: u64,
        coverage_drift_count: u64,
        placement_advisory_count: u64,
        global_scan_debt_count: u64,
    ) -> Self {
        Self {
            classified_artifacts,
            receipts_consumed,
            drift_checks_performed,
            equivalence_checks_performed,
            index_probes,
            allocation_count,
            clone_count,
            stale_rejection_count,
            coverage_drift_count,
            placement_advisory_count,
            global_scan_debt_count,
        }
    }

    pub(crate) fn phase1_zero() -> Self {
        Self::new(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn classified_artifacts(&self) -> u64 {
        self.classified_artifacts
    }

    pub fn receipts_consumed(&self) -> u64 {
        self.receipts_consumed
    }

    pub fn drift_checks_performed(&self) -> u64 {
        self.drift_checks_performed
    }

    pub fn equivalence_checks_performed(&self) -> u64 {
        self.equivalence_checks_performed
    }

    pub fn index_probes(&self) -> u64 {
        self.index_probes
    }

    pub fn stale_rejection_count(&self) -> u64 {
        self.stale_rejection_count
    }

    pub fn coverage_drift_count(&self) -> u64 {
        self.coverage_drift_count
    }

    pub fn placement_advisory_count(&self) -> u64 {
        self.placement_advisory_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustClassificationCounterSnapshot {
    classified_artifacts: u64,
    exact_trust_count: u64,
    degraded_trust_count: u64,
    rebuild_derived_trust_count: u64,
    rejected_trust_count: u64,
    receipts_consumed: u64,
    drift_checks_performed: u64,
    equivalence_checks_performed: u64,
    forbidden_exact_overclaim_count: u64,
    stale_rejection_count: u64,
    coverage_drift_count: u64,
    placement_advisory_count: u64,
    global_scan_debt_count: u64,
}

impl SupportTrustClassificationCounterSnapshot {
    pub fn new(
        classified_artifacts: u64,
        exact_trust_count: u64,
        degraded_trust_count: u64,
        rebuild_derived_trust_count: u64,
        rejected_trust_count: u64,
        receipts_consumed: u64,
        drift_checks_performed: u64,
        equivalence_checks_performed: u64,
        forbidden_exact_overclaim_count: u64,
        stale_rejection_count: u64,
        coverage_drift_count: u64,
        placement_advisory_count: u64,
        global_scan_debt_count: u64,
    ) -> Self {
        Self {
            classified_artifacts,
            exact_trust_count,
            degraded_trust_count,
            rebuild_derived_trust_count,
            rejected_trust_count,
            receipts_consumed,
            drift_checks_performed,
            equivalence_checks_performed,
            forbidden_exact_overclaim_count,
            stale_rejection_count,
            coverage_drift_count,
            placement_advisory_count,
            global_scan_debt_count,
        }
    }

    pub fn exact_trust_count(&self) -> u64 {
        self.exact_trust_count
    }

    pub fn degraded_trust_count(&self) -> u64 {
        self.degraded_trust_count
    }

    pub fn rebuild_derived_trust_count(&self) -> u64 {
        self.rebuild_derived_trust_count
    }

    pub fn rejected_trust_count(&self) -> u64 {
        self.rejected_trust_count
    }

    pub fn forbidden_exact_overclaim_count(&self) -> u64 {
        self.forbidden_exact_overclaim_count
    }

    pub fn stale_rejection_count(&self) -> u64 {
        self.stale_rejection_count
    }

    pub fn coverage_drift_count(&self) -> u64 {
        self.coverage_drift_count
    }

    pub fn placement_advisory_count(&self) -> u64 {
        self.placement_advisory_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustClassificationPlan {
    posture: SupportRoleTrustPosture,
    epoch: SupportTrustEpoch,
    performance_plan: SupportTrustPerformancePlan,
    cost_surface: SupportTrustClassificationCostSurface,
}

impl SupportTrustClassificationPlan {
    pub fn new(
        posture: SupportRoleTrustPosture,
        epoch: SupportTrustEpoch,
        performance_plan: SupportTrustPerformancePlan,
    ) -> Self {
        let cost_surface = SupportTrustClassificationCostSurface::new(
            1,
            0,
            0,
            0,
            performance_plan.expected_index_probes(),
            performance_plan.expected_allocation_count(),
            performance_plan.expected_clone_count(),
            0,
            0,
            0,
            0,
        );
        Self {
            posture,
            epoch,
            performance_plan,
            cost_surface,
        }
    }

    pub fn posture(&self) -> &SupportRoleTrustPosture {
        &self.posture
    }

    pub fn epoch(&self) -> SupportTrustEpoch {
        self.epoch
    }

    pub fn performance_plan(&self) -> &SupportTrustPerformancePlan {
        &self.performance_plan
    }

    pub fn cost_surface(&self) -> SupportTrustClassificationCostSurface {
        self.cost_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustClassificationReport {
    posture: SupportRoleTrustPosture,
    trust_class: SupportTrustClass,
    downgrade_reason: Option<SupportTrustDowngradeReason>,
    epoch: SupportTrustEpoch,
    performance_plan: SupportTrustPerformancePlan,
    cost_surface: SupportTrustClassificationCostSurface,
}

impl SupportTrustClassificationReport {
    #[allow(dead_code)]
    pub(crate) fn from_plan(
        plan: SupportTrustClassificationPlan,
        trust_class: SupportTrustClass,
        downgrade_reason: Option<SupportTrustDowngradeReason>,
    ) -> Self {
        Self {
            posture: plan.posture,
            trust_class,
            downgrade_reason,
            epoch: plan.epoch,
            performance_plan: plan.performance_plan,
            cost_surface: plan.cost_surface,
        }
    }

    pub fn posture(&self) -> &SupportRoleTrustPosture {
        &self.posture
    }

    pub fn trust_class(&self) -> SupportTrustClass {
        self.trust_class
    }

    pub fn downgrade_reason(&self) -> Option<SupportTrustDowngradeReason> {
        self.downgrade_reason
    }

    pub fn epoch(&self) -> SupportTrustEpoch {
        self.epoch
    }

    pub fn performance_plan(&self) -> &SupportTrustPerformancePlan {
        &self.performance_plan
    }

    pub fn cost_surface(&self) -> SupportTrustClassificationCostSurface {
        self.cost_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustClassificationWitness {
    report: SupportTrustClassificationReport,
}

impl SupportTrustClassificationWitness {
    #[allow(dead_code)]
    pub(crate) fn new(report: SupportTrustClassificationReport) -> Self {
        Self { report }
    }

    pub fn report(&self) -> &SupportTrustClassificationReport {
        &self.report
    }
}
