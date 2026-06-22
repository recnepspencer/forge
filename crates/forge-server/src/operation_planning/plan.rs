use std::fmt;

use crate::ForgeServerQueryHandoff;

use super::{
    ForgeServerOperationExecutionStrategy, ForgeServerOperationPlanCounters,
    ForgeServerOperationPlanEvidencePolicy, ForgeServerOperationPlanProof,
    ForgeServerOperationPlanReceipt,
};

pub struct ForgeServerLoweredOperationPlan {
    query_handoff: ForgeServerQueryHandoff,
    strategy: ForgeServerOperationExecutionStrategy,
    evidence_policy: ForgeServerOperationPlanEvidencePolicy,
    counters: ForgeServerOperationPlanCounters,
    receipt: ForgeServerOperationPlanReceipt,
    canonical_digest: String,
}

impl fmt::Debug for ForgeServerLoweredOperationPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerLoweredOperationPlan")
            .field("query_handoff", &self.query_handoff)
            .field("strategy", &self.strategy)
            .field("evidence_policy", &self.evidence_policy)
            .field("counters", &self.counters)
            .field("receipt", &self.receipt)
            .field("canonical_digest", &self.canonical_digest)
            .finish()
    }
}

impl ForgeServerLoweredOperationPlan {
    pub(crate) fn new(
        query_handoff: ForgeServerQueryHandoff,
        strategy: ForgeServerOperationExecutionStrategy,
        evidence_policy: ForgeServerOperationPlanEvidencePolicy,
        counters: ForgeServerOperationPlanCounters,
        receipt: ForgeServerOperationPlanReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-lowered-operation-plan-v1|handoff={}|strategy={}|receipt={}",
            query_handoff.canonical_digest(),
            strategy.as_str(),
            receipt.plan_identity(),
        );
        Self {
            query_handoff,
            strategy,
            evidence_policy,
            counters,
            receipt,
            canonical_digest,
        }
    }

    pub fn query_handoff(&self) -> &ForgeServerQueryHandoff {
        &self.query_handoff
    }

    pub fn strategy(&self) -> ForgeServerOperationExecutionStrategy {
        self.strategy
    }

    pub fn evidence_policy(&self) -> &ForgeServerOperationPlanEvidencePolicy {
        &self.evidence_policy
    }

    pub fn counters(&self) -> &ForgeServerOperationPlanCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &ForgeServerOperationPlanReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn proof(&self) -> ForgeServerOperationPlanProof {
        ForgeServerOperationPlanProof::new(
            self.receipt.clone(),
            self.counters.clone(),
            self.strategy,
            self.evidence_policy.clone(),
        )
    }

    pub(crate) fn into_query_handoff(self) -> ForgeServerQueryHandoff {
        self.query_handoff
    }
}
