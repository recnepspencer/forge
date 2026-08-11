use serde::Serialize;

use super::super::PlacementDemandSummary;
use super::breadth::TierMoveBreadthSummary;
use super::derived_move::DerivedTierMovePlan;
use super::locality::{FamilyLocalPlacementPlan, TierLocalityFootprint};
use super::rejection::{TierMoveRejection, WorkingSetDebtSummary};

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
