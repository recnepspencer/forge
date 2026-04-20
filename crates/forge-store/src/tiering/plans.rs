#![allow(dead_code)]

use forge_relational::facade::history::BranchId;
use serde::Serialize;

use super::{
    AdaptivePlacementDebtMarker, PlacementArtifactFamily, PlacementBudgetClass,
    PlacementExecutionOrigin, PlacementDemandSummary, PlacementObservationScopeClass,
    RecallAmplificationBudget, RecallCostClass, RecallEligibilityWitness, TierResidenceClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeTierMovePlan {
    artifact_key: String,
    target_residence: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl AuthoritativeTierMovePlan {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
            budget_class,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTierMovePlan {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    target_residence: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl DerivedTierMovePlan {
    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        target_residence: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            target_residence,
            budget_class,
            execution_origin,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallPreparationPlan {
    artifact_key: String,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
    execution_origin: PlacementExecutionOrigin,
}

impl RecallPreparationPlan {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            recall_cost_class,
            amplification_budget,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn recall_cost_class(&self) -> RecallCostClass {
        self.recall_cost_class
    }

    pub fn amplification_budget(&self) -> RecallAmplificationBudget {
        self.amplification_budget
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementStabilityPlan {
    artifact_keys: Vec<String>,
    retained_basis_label: Option<String>,
}

impl PlacementStabilityPlan {
    pub(crate) fn new(
        mut artifact_keys: Vec<String>,
        retained_basis_label: Option<String>,
    ) -> Self {
        artifact_keys.sort();
        artifact_keys.dedup();
        Self {
            artifact_keys,
            retained_basis_label,
        }
    }

    pub fn artifact_keys(&self) -> &[String] {
        &self.artifact_keys
    }

    pub fn retained_basis_label(&self) -> Option<&str> {
        self.retained_basis_label.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BroadenedRecallPlan {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    widened_artifact_keys: Vec<String>,
    execution_origin: PlacementExecutionOrigin,
}

impl BroadenedRecallPlan {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut widened_artifact_keys: Vec<String>,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        widened_artifact_keys.sort();
        widened_artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            widened_artifact_keys,
            execution_origin,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn widened_artifact_keys(&self) -> &[String] {
        &self.widened_artifact_keys
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TierMoveRejection {
    UnsupportedPolicy { marker: AdaptivePlacementDebtMarker },
    IllegalExecutionOrigin { origin: PlacementExecutionOrigin },
    RawLocatorBoundaryViolation { locator: String },
    WitnessConstructionRequired { witness_type: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierLocalityFootprint {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    artifact_keys: Vec<String>,
}

impl TierLocalityFootprint {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut artifact_keys: Vec<String>,
    ) -> Self {
        artifact_keys.sort();
        artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            artifact_keys,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn artifact_keys(&self) -> &[String] {
        &self.artifact_keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FamilyLocalPlacementPlan {
    locality_footprint: TierLocalityFootprint,
    target_residence: TierResidenceClass,
}

impl FamilyLocalPlacementPlan {
    pub(crate) fn new(
        locality_footprint: TierLocalityFootprint,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            locality_footprint,
            target_residence,
        }
    }

    pub fn locality_footprint(&self) -> &TierLocalityFootprint {
        &self.locality_footprint
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedRangePlacementPlan {
    branch_id: BranchId,
    retained_basis_label: String,
    target_residence: TierResidenceClass,
}

impl RetainedRangePlacementPlan {
    pub(crate) fn new(
        branch_id: BranchId,
        retained_basis_label: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            branch_id,
            retained_basis_label: retained_basis_label.into(),
            target_residence,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierMoveBreadthSummary {
    candidate_count: u64,
    admitted_count: u64,
    locality_group_count: u64,
}

impl TierMoveBreadthSummary {
    pub(crate) fn new(candidate_count: u64, admitted_count: u64, locality_group_count: u64) -> Self {
        Self {
            candidate_count,
            admitted_count,
            locality_group_count,
        }
    }

    pub fn candidate_count(&self) -> u64 {
        self.candidate_count
    }

    pub fn admitted_count(&self) -> u64 {
        self.admitted_count
    }

    pub fn locality_group_count(&self) -> u64 {
        self.locality_group_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallBreadthSummary {
    family_local_unit_count: u64,
    widened_artifact_count: u64,
}

impl RecallBreadthSummary {
    pub(crate) fn new(family_local_unit_count: u64, widened_artifact_count: u64) -> Self {
        Self {
            family_local_unit_count,
            widened_artifact_count,
        }
    }

    pub fn family_local_unit_count(&self) -> u64 {
        self.family_local_unit_count
    }

    pub fn widened_artifact_count(&self) -> u64 {
        self.widened_artifact_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkingSetDebtSummary {
    debt_marker: AdaptivePlacementDebtMarker,
    reason: String,
}

impl WorkingSetDebtSummary {
    pub(crate) fn new(
        debt_marker: AdaptivePlacementDebtMarker,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            debt_marker,
            reason: reason.into(),
        }
    }

    pub fn debt_marker(&self) -> AdaptivePlacementDebtMarker {
        self.debt_marker
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativePlacementPlanningReport {
    demand_summary: PlacementDemandSummary,
    retained_range_plan: Option<RetainedRangePlacementPlan>,
    tier_move_plan: Option<AuthoritativeTierMovePlan>,
    locality_footprint: TierLocalityFootprint,
    breadth_summary: TierMoveBreadthSummary,
    rejection: Option<TierMoveRejection>,
    debt: Option<WorkingSetDebtSummary>,
}

impl AuthoritativePlacementPlanningReport {
    pub(crate) fn new(
        demand_summary: PlacementDemandSummary,
        retained_range_plan: Option<RetainedRangePlacementPlan>,
        tier_move_plan: Option<AuthoritativeTierMovePlan>,
        locality_footprint: TierLocalityFootprint,
        breadth_summary: TierMoveBreadthSummary,
        rejection: Option<TierMoveRejection>,
        debt: Option<WorkingSetDebtSummary>,
    ) -> Self {
        Self {
            demand_summary,
            retained_range_plan,
            tier_move_plan,
            locality_footprint,
            breadth_summary,
            rejection,
            debt,
        }
    }

    pub fn demand_summary(&self) -> &PlacementDemandSummary {
        &self.demand_summary
    }

    pub fn retained_range_plan(&self) -> Option<&RetainedRangePlacementPlan> {
        self.retained_range_plan.as_ref()
    }

    pub fn tier_move_plan(&self) -> Option<&AuthoritativeTierMovePlan> {
        self.tier_move_plan.as_ref()
    }

    pub fn locality_footprint(&self) -> &TierLocalityFootprint {
        &self.locality_footprint
    }

    pub fn breadth_summary(&self) -> &TierMoveBreadthSummary {
        &self.breadth_summary
    }

    pub fn rejection(&self) -> Option<&TierMoveRejection> {
        self.rejection.as_ref()
    }

    pub fn debt(&self) -> Option<&WorkingSetDebtSummary> {
        self.debt.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedPlacementPlanningReport {
    demand_summary: PlacementDemandSummary,
    family_local_plan: Option<FamilyLocalPlacementPlan>,
    tier_move_plan: Option<DerivedTierMovePlan>,
    locality_footprint: TierLocalityFootprint,
    breadth_summary: TierMoveBreadthSummary,
    rejection: Option<TierMoveRejection>,
    debt: Option<WorkingSetDebtSummary>,
}

impl DerivedPlacementPlanningReport {
    pub(crate) fn new(
        demand_summary: PlacementDemandSummary,
        family_local_plan: Option<FamilyLocalPlacementPlan>,
        tier_move_plan: Option<DerivedTierMovePlan>,
        locality_footprint: TierLocalityFootprint,
        breadth_summary: TierMoveBreadthSummary,
        rejection: Option<TierMoveRejection>,
        debt: Option<WorkingSetDebtSummary>,
    ) -> Self {
        Self {
            demand_summary,
            family_local_plan,
            tier_move_plan,
            locality_footprint,
            breadth_summary,
            rejection,
            debt,
        }
    }

    pub fn demand_summary(&self) -> &PlacementDemandSummary {
        &self.demand_summary
    }

    pub fn family_local_plan(&self) -> Option<&FamilyLocalPlacementPlan> {
        self.family_local_plan.as_ref()
    }

    pub fn tier_move_plan(&self) -> Option<&DerivedTierMovePlan> {
        self.tier_move_plan.as_ref()
    }

    pub fn locality_footprint(&self) -> &TierLocalityFootprint {
        &self.locality_footprint
    }

    pub fn breadth_summary(&self) -> &TierMoveBreadthSummary {
        &self.breadth_summary
    }

    pub fn rejection(&self) -> Option<&TierMoveRejection> {
        self.rejection.as_ref()
    }

    pub fn debt(&self) -> Option<&WorkingSetDebtSummary> {
        self.debt.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReadPlacementPlanningReport {
    resident_lease: Option<crate::ResidentReadLease>,
    cold_recall_lease: Option<crate::ColdRecallLease>,
    recall_witness: Option<RecallEligibilityWitness>,
    breadth_summary: RecallBreadthSummary,
    rejection: Option<TierMoveRejection>,
}

impl ReadPlacementPlanningReport {
    pub(crate) fn new(
        resident_lease: Option<crate::ResidentReadLease>,
        cold_recall_lease: Option<crate::ColdRecallLease>,
        recall_witness: Option<RecallEligibilityWitness>,
        breadth_summary: RecallBreadthSummary,
        rejection: Option<TierMoveRejection>,
    ) -> Self {
        Self {
            resident_lease,
            cold_recall_lease,
            recall_witness,
            breadth_summary,
            rejection,
        }
    }

    pub fn resident_lease(&self) -> Option<&crate::ResidentReadLease> {
        self.resident_lease.as_ref()
    }

    pub fn cold_recall_lease(&self) -> Option<&crate::ColdRecallLease> {
        self.cold_recall_lease.as_ref()
    }

    pub fn recall_witness(&self) -> Option<&RecallEligibilityWitness> {
        self.recall_witness.as_ref()
    }

    pub fn breadth_summary(&self) -> &RecallBreadthSummary {
        &self.breadth_summary
    }

    pub fn rejection(&self) -> Option<&TierMoveRejection> {
        self.rejection.as_ref()
    }
}
