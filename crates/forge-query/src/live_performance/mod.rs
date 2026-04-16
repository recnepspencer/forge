use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveMaintenanceCostClass {
    DetailPatch,
    OrderedCollectionPatch,
    BoundedMaterializationPatch,
    RefreshFallback,
}

impl LiveMaintenanceCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailPatch => "detail_patch",
            Self::OrderedCollectionPatch => "ordered_collection_patch",
            Self::BoundedMaterializationPatch => "bounded_materialization_patch",
            Self::RefreshFallback => "refresh_fallback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveMaintenanceWorkUnit {
    ProjectedFieldDeltaCount,
    DerivedFieldRecomputationCount,
    MembershipDeltaCount,
    OrderingRepositionCount,
    PageLocalMoveCount,
    CrossPageMoveCount,
    InScopeNodeDeltaCount,
    InScopeEdgeDeltaCount,
    ScopeExpansionCount,
    ScopeContractionCount,
}

impl LiveMaintenanceWorkUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedFieldDeltaCount => "projected_field_delta_count",
            Self::DerivedFieldRecomputationCount => "derived_field_recomputation_count",
            Self::MembershipDeltaCount => "membership_delta_count",
            Self::OrderingRepositionCount => "ordering_reposition_count",
            Self::PageLocalMoveCount => "page_local_move_count",
            Self::CrossPageMoveCount => "cross_page_move_count",
            Self::InScopeNodeDeltaCount => "in_scope_node_delta_count",
            Self::InScopeEdgeDeltaCount => "in_scope_edge_delta_count",
            Self::ScopeExpansionCount => "scope_expansion_count",
            Self::ScopeContractionCount => "scope_contraction_count",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveMaintenanceComplexityContract {
    cost_class: LiveMaintenanceCostClass,
    work_units: Vec<LiveMaintenanceWorkUnit>,
}

impl LiveMaintenanceComplexityContract {
    pub fn cost_class(&self) -> &LiveMaintenanceCostClass {
        &self.cost_class
    }

    pub fn work_units(&self) -> &[LiveMaintenanceWorkUnit] {
        &self.work_units
    }

    pub fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![format!("live_cost_class:{}", self.cost_class.as_str())];
        parts.extend(
            self.work_units
                .iter()
                .map(|unit| format!("live_work_unit:{}", unit.as_str())),
        );
        parts
    }

    pub fn detail_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::DetailPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::ProjectedFieldDeltaCount,
                LiveMaintenanceWorkUnit::DerivedFieldRecomputationCount,
            ],
        }
    }

    pub fn ordered_collection_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::OrderedCollectionPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::MembershipDeltaCount,
                LiveMaintenanceWorkUnit::OrderingRepositionCount,
                LiveMaintenanceWorkUnit::PageLocalMoveCount,
                LiveMaintenanceWorkUnit::CrossPageMoveCount,
            ],
        }
    }

    pub fn bounded_materialization_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::BoundedMaterializationPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::InScopeNodeDeltaCount,
                LiveMaintenanceWorkUnit::InScopeEdgeDeltaCount,
                LiveMaintenanceWorkUnit::ScopeExpansionCount,
                LiveMaintenanceWorkUnit::ScopeContractionCount,
            ],
        }
    }

    pub fn refresh_fallback() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::RefreshFallback,
            work_units: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IncrementalMaintenanceClass {
    Incremental,
    RefreshAdmitted,
    Forbidden,
}

impl IncrementalMaintenanceClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Incremental => "incremental",
            Self::RefreshAdmitted => "refresh_admitted",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncrementalPatchEligibility {
    maintenance_class: IncrementalMaintenanceClass,
    reason: String,
}

impl IncrementalPatchEligibility {
    pub fn maintenance_class(&self) -> &IncrementalMaintenanceClass {
        &self.maintenance_class
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn incremental(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::Incremental,
            reason: reason.into(),
        }
    }

    pub fn refresh_admitted(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::RefreshAdmitted,
            reason: reason.into(),
        }
    }

    pub fn forbidden(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::Forbidden,
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PatchWidthUnit {
    ProjectedFieldDelta,
    CollectionRowChange,
    MaterializedNodeChange,
}

impl PatchWidthUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedFieldDelta => "projected_field_delta",
            Self::CollectionRowChange => "collection_row_change",
            Self::MaterializedNodeChange => "materialized_node_change",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PatchWidthBudget {
    unit: PatchWidthUnit,
    limit: usize,
}

impl PatchWidthBudget {
    pub fn unit(&self) -> &PatchWidthUnit {
        &self.unit
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn new(unit: PatchWidthUnit, limit: usize) -> Self {
        Self { unit, limit }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PatchWidthPolicy {
    DeliverWithinBudget,
    CoalesceWithinAdmittedClass,
    RefreshWithinAdmissionMatrix,
    RejectOverflow,
}

impl PatchWidthPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeliverWithinBudget => "deliver_within_budget",
            Self::CoalesceWithinAdmittedClass => "coalesce_within_admitted_class",
            Self::RefreshWithinAdmissionMatrix => "refresh_within_admission_matrix",
            Self::RejectOverflow => "reject_overflow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoalescingAdmissionClass {
    Forbidden,
    BasisStableEquivalent,
}

impl CoalescingAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forbidden => "forbidden",
            Self::BasisStableEquivalent => "basis_stable_equivalent",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshCostClass {
    NarrowRefresh,
    BroadRefresh,
}

impl RefreshCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NarrowRefresh => "narrow_refresh",
            Self::BroadRefresh => "broad_refresh",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RefreshAdmissionStatus {
    Verified,
    Debt,
    Forbidden,
}

impl RefreshAdmissionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
            Self::Forbidden => "forbidden",
        }
    }
}

pub trait PerformanceStatus {
    const LABEL: &'static str;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VerifiedPerformance;

impl PerformanceStatus for VerifiedPerformance {
    const LABEL: &'static str = "verified";
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DebtPerformance;

impl PerformanceStatus for DebtPerformance {
    const LABEL: &'static str = "debt";
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForbiddenPerformance;

impl PerformanceStatus for ForbiddenPerformance {
    const LABEL: &'static str = "forbidden";
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerformanceStatusMarker<S: PerformanceStatus> {
    _marker: PhantomData<S>,
}

impl<S: PerformanceStatus> PerformanceStatusMarker<S> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn label(&self) -> &'static str {
        S::LABEL
    }
}

impl<S: PerformanceStatus> Default for PerformanceStatusMarker<S> {
    fn default() -> Self {
        Self::new()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_complexity_contract_names_field_delta_units() {
        let contract = LiveMaintenanceComplexityContract::detail_patch();
        assert_eq!(
            contract.cost_class(),
            &LiveMaintenanceCostClass::DetailPatch
        );
        assert!(contract
            .work_units()
            .contains(&LiveMaintenanceWorkUnit::ProjectedFieldDeltaCount));
    }

    #[test]
    fn performance_report_digest_includes_budget_and_status() {
        let report = LivePerformanceReport::verified_detail_family();
        let parts = report.digest_parts();

        assert!(parts
            .iter()
            .any(|part| part == "patch_width_policy:deliver_within_budget"));
        assert!(parts
            .iter()
            .any(|part| part == "performance_status:verified"));
    }

    #[test]
    fn ordered_collection_report_uses_collection_row_budget_units() {
        let report = LivePerformanceReport::verified_ordered_collection_family();
        assert_eq!(
            report.width_budget().unit(),
            &PatchWidthUnit::CollectionRowChange
        );
    }

    #[test]
    fn bounded_materialization_report_declares_debt_status() {
        let report = LivePerformanceReport::debt_bounded_materialization_family();
        assert_eq!(report.performance_status(), DebtPerformance::LABEL);
        assert_eq!(
            report.refresh_admission_status(),
            &RefreshAdmissionStatus::Debt
        );
    }
}
