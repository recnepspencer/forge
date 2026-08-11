use serde::Serialize;

use super::super::proofs::{RecallEligibilityWitness, RetainedReadPlacementPath, TierMissOutcome};
use super::recall::{ColdRecallPlan, RecallBreadthSummary, RecallDebtSummary};
use super::rejection::TierMoveRejection;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReadPlacementPlanningReport {
    resident_lease: Option<crate::ResidentReadLease>,
    cold_recall_lease: Option<crate::ColdRecallLease>,
    cold_recall_plan: Option<ColdRecallPlan>,
    recall_witness: Option<RecallEligibilityWitness>,
    retained_read_path: Option<RetainedReadPlacementPath>,
    tier_miss_outcome: Option<TierMissOutcome>,
    breadth_summary: RecallBreadthSummary,
    rejection: Option<TierMoveRejection>,
    recall_debt: Option<RecallDebtSummary>,
}

impl ReadPlacementPlanningReport {
    pub(crate) fn new(
        resident_lease: Option<crate::ResidentReadLease>,
        cold_recall_lease: Option<crate::ColdRecallLease>,
        cold_recall_plan: Option<ColdRecallPlan>,
        recall_witness: Option<RecallEligibilityWitness>,
        retained_read_path: Option<RetainedReadPlacementPath>,
        tier_miss_outcome: Option<TierMissOutcome>,
        breadth_summary: RecallBreadthSummary,
        rejection: Option<TierMoveRejection>,
        recall_debt: Option<RecallDebtSummary>,
    ) -> Self {
        Self {
            resident_lease,
            cold_recall_lease,
            cold_recall_plan,
            recall_witness,
            retained_read_path,
            tier_miss_outcome,
            breadth_summary,
            rejection,
            recall_debt,
        }
    }

    pub fn resident_lease(&self) -> Option<&crate::ResidentReadLease> {
        self.resident_lease.as_ref()
    }

    pub fn cold_recall_lease(&self) -> Option<&crate::ColdRecallLease> {
        self.cold_recall_lease.as_ref()
    }

    pub fn cold_recall_plan(&self) -> Option<&ColdRecallPlan> {
        self.cold_recall_plan.as_ref()
    }

    pub fn recall_witness(&self) -> Option<&RecallEligibilityWitness> {
        self.recall_witness.as_ref()
    }

    pub fn retained_read_path(&self) -> Option<RetainedReadPlacementPath> {
        self.retained_read_path
    }

    pub fn tier_miss_outcome(&self) -> Option<TierMissOutcome> {
        self.tier_miss_outcome
    }

    pub fn breadth_summary(&self) -> &RecallBreadthSummary {
        &self.breadth_summary
    }

    pub fn rejection(&self) -> Option<&TierMoveRejection> {
        self.rejection.as_ref()
    }

    pub fn recall_debt(&self) -> Option<&RecallDebtSummary> {
        self.recall_debt.as_ref()
    }
}
