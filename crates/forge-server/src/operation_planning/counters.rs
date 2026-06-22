use super::ForgeServerOperationExecutionStrategy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationPlanCounters {
    support_rows_consulted: usize,
    footprint_breadth: usize,
    strategy_choice: ForgeServerOperationExecutionStrategy,
    evidence_policy_lane: String,
    canonical_digest: String,
}

impl ForgeServerOperationPlanCounters {
    pub(crate) fn new(
        support_rows_consulted: usize,
        footprint_breadth: usize,
        strategy_choice: ForgeServerOperationExecutionStrategy,
        evidence_policy_lane: impl Into<String>,
    ) -> Self {
        let evidence_policy_lane = evidence_policy_lane.into();
        let canonical_digest = format!(
            "forge-server-operation-plan-counters-v1|support_rows={support_rows_consulted}|footprint_breadth={footprint_breadth}|strategy={}|evidence={evidence_policy_lane}",
            strategy_choice.as_str(),
        );
        Self {
            support_rows_consulted,
            footprint_breadth,
            strategy_choice,
            evidence_policy_lane,
            canonical_digest,
        }
    }

    pub fn support_rows_consulted(&self) -> usize {
        self.support_rows_consulted
    }

    pub fn footprint_breadth(&self) -> usize {
        self.footprint_breadth
    }

    pub fn strategy_choice(&self) -> ForgeServerOperationExecutionStrategy {
        self.strategy_choice
    }

    pub fn evidence_policy_lane(&self) -> &str {
        &self.evidence_policy_lane
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
