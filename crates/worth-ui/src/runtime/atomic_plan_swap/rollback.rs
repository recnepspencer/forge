use crate::runtime::{
    WorthUiAtomicPlanSwapCounters, WorthUiPlanSwapDenialReason, WorthUiPriorValidPlanObservation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanSwapRollback {
    reason: WorthUiPlanSwapDenialReason,
    prior_valid_plan: WorthUiPriorValidPlanObservation,
    restored_active_artifact_digest: u64,
    restored_active_plan_digest: u64,
    attempted_next_artifact_digest: Option<u64>,
    attempted_next_plan_digest: Option<u64>,
    counters: WorthUiAtomicPlanSwapCounters,
}

impl WorthUiPlanSwapRollback {
    pub(crate) fn new(
        reason: WorthUiPlanSwapDenialReason,
        prior_valid_plan: WorthUiPriorValidPlanObservation,
        attempted_next_artifact_digest: Option<u64>,
        attempted_next_plan_digest: Option<u64>,
        counters: WorthUiAtomicPlanSwapCounters,
    ) -> Self {
        Self {
            reason,
            prior_valid_plan,
            restored_active_artifact_digest: prior_valid_plan.artifact_digest(),
            restored_active_plan_digest: prior_valid_plan.active_plan_digest(),
            attempted_next_artifact_digest,
            attempted_next_plan_digest,
            counters,
        }
    }

    pub fn reason(self) -> WorthUiPlanSwapDenialReason {
        self.reason
    }

    pub fn prior_valid_plan(self) -> WorthUiPriorValidPlanObservation {
        self.prior_valid_plan
    }

    pub fn restored_active_artifact_digest(self) -> u64 {
        self.restored_active_artifact_digest
    }

    pub fn restored_active_plan_digest(self) -> u64 {
        self.restored_active_plan_digest
    }

    pub fn attempted_next_artifact_digest(self) -> Option<u64> {
        self.attempted_next_artifact_digest
    }

    pub fn attempted_next_plan_digest(self) -> Option<u64> {
        self.attempted_next_plan_digest
    }

    pub fn counters(self) -> WorthUiAtomicPlanSwapCounters {
        self.counters
    }
}
