use super::budget_policy::{
    CoalescingAdmissionClass, PatchWidthBudget, PatchWidthPolicy, PatchWidthUnit,
    RefreshAdmissionStatus, RefreshCostClass,
};
use super::maintenance_cost::LiveMaintenanceComplexityContract;
use super::performance_status::{DebtPerformance, PerformanceStatus, VerifiedPerformance};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivePerformanceReport {
    complexity_contract: LiveMaintenanceComplexityContract,
    width_budget: PatchWidthBudget,
    width_policy: PatchWidthPolicy,
    coalescing_admission: CoalescingAdmissionClass,
    refresh_cost_class: RefreshCostClass,
    refresh_admission_status: RefreshAdmissionStatus,
    performance_status: &'static str,
}

impl LivePerformanceReport {
    pub fn complexity_contract(&self) -> &LiveMaintenanceComplexityContract {
        &self.complexity_contract
    }

    pub fn width_budget(&self) -> &PatchWidthBudget {
        &self.width_budget
    }

    pub fn width_policy(&self) -> &PatchWidthPolicy {
        &self.width_policy
    }

    pub fn coalescing_admission(&self) -> &CoalescingAdmissionClass {
        &self.coalescing_admission
    }

    pub fn refresh_cost_class(&self) -> &RefreshCostClass {
        &self.refresh_cost_class
    }

    pub fn refresh_admission_status(&self) -> &RefreshAdmissionStatus {
        &self.refresh_admission_status
    }

    pub fn performance_status(&self) -> &'static str {
        self.performance_status
    }

    pub fn digest_parts(&self) -> Vec<String> {
        let mut parts = self.complexity_contract.digest_parts();
        parts.push(format!(
            "patch_width_budget:{}:{}",
            self.width_budget.unit().as_str(),
            self.width_budget.limit()
        ));
        parts.push(format!("patch_width_policy:{}", self.width_policy.as_str()));
        parts.push(format!(
            "coalescing_admission:{}",
            self.coalescing_admission.as_str()
        ));
        parts.push(format!(
            "refresh_cost_class:{}",
            self.refresh_cost_class.as_str()
        ));
        parts.push(format!(
            "refresh_admission_status:{}",
            self.refresh_admission_status.as_str()
        ));
        parts.push(format!("performance_status:{}", self.performance_status));
        parts
    }

    pub fn verified_detail_family() -> Self {
        Self {
            complexity_contract: LiveMaintenanceComplexityContract::detail_patch(),
            width_budget: PatchWidthBudget::new(PatchWidthUnit::ProjectedFieldDelta, 32),
            width_policy: PatchWidthPolicy::DeliverWithinBudget,
            coalescing_admission: CoalescingAdmissionClass::Forbidden,
            refresh_cost_class: RefreshCostClass::NarrowRefresh,
            refresh_admission_status: RefreshAdmissionStatus::Verified,
            performance_status: VerifiedPerformance::LABEL,
        }
    }

    pub fn verified_ordered_collection_family() -> Self {
        Self {
            complexity_contract: LiveMaintenanceComplexityContract::ordered_collection_patch(),
            width_budget: PatchWidthBudget::new(PatchWidthUnit::CollectionRowChange, 64),
            width_policy: PatchWidthPolicy::CoalesceWithinAdmittedClass,
            coalescing_admission: CoalescingAdmissionClass::BasisStableEquivalent,
            refresh_cost_class: RefreshCostClass::NarrowRefresh,
            refresh_admission_status: RefreshAdmissionStatus::Verified,
            performance_status: VerifiedPerformance::LABEL,
        }
    }

    pub fn debt_bounded_materialization_family() -> Self {
        Self {
            complexity_contract: LiveMaintenanceComplexityContract::bounded_materialization_patch(),
            width_budget: PatchWidthBudget::new(PatchWidthUnit::MaterializedNodeChange, 96),
            width_policy: PatchWidthPolicy::RefreshWithinAdmissionMatrix,
            coalescing_admission: CoalescingAdmissionClass::BasisStableEquivalent,
            refresh_cost_class: RefreshCostClass::BroadRefresh,
            refresh_admission_status: RefreshAdmissionStatus::Debt,
            performance_status: DebtPerformance::LABEL,
        }
    }
}
