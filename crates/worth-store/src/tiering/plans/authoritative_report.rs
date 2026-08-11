use serde::Serialize;

use super::super::PlacementDemandSummary;
use super::authoritative_move::AuthoritativeTierMovePlan;
use super::breadth::TierMoveBreadthSummary;
use super::locality::{RetainedRangePlacementPlan, TierLocalityFootprint};
use super::rejection::{TierMoveRejection, WorkingSetDebtSummary};

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
