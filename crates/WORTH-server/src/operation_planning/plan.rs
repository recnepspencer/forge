use std::fmt;

use crate::WorthServerQueryHandoff;

use super::{
    WorthServerOperationExecutionStrategy, WorthServerOperationPlanCounters,
    WorthServerOperationPlanEvidencePolicy, WorthServerOperationPlanProof,
    WorthServerOperationPlanReceipt,
};

pub struct WorthServerLoweredOperationPlan {
    query_handoff: WorthServerQueryHandoff,
    strategy: WorthServerOperationExecutionStrategy,
    evidence_policy: WorthServerOperationPlanEvidencePolicy,
    counters: WorthServerOperationPlanCounters,
    receipt: WorthServerOperationPlanReceipt,
    canonical_digest: String,
}

impl fmt::Debug for WorthServerLoweredOperationPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthServerLoweredOperationPlan")
            .field("query_handoff", &self.query_handoff)
            .field("strategy", &self.strategy)
            .field("evidence_policy", &self.evidence_policy)
            .field("counters", &self.counters)
            .field("receipt", &self.receipt)
            .field("canonical_digest", &self.canonical_digest)
            .finish()
    }
}

impl WorthServerLoweredOperationPlan {
    pub(crate) fn new(
        query_handoff: WorthServerQueryHandoff,
        strategy: WorthServerOperationExecutionStrategy,
        evidence_policy: WorthServerOperationPlanEvidencePolicy,
        counters: WorthServerOperationPlanCounters,
        receipt: WorthServerOperationPlanReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-lowered-operation-plan-v1|handoff={}|strategy={}|receipt={}",
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

    pub fn query_handoff(&self) -> &WorthServerQueryHandoff {
        &self.query_handoff
    }

    pub fn strategy(&self) -> WorthServerOperationExecutionStrategy {
        self.strategy
    }

    pub fn evidence_policy(&self) -> &WorthServerOperationPlanEvidencePolicy {
        &self.evidence_policy
    }

    pub fn counters(&self) -> &WorthServerOperationPlanCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &WorthServerOperationPlanReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn proof(&self) -> WorthServerOperationPlanProof {
        WorthServerOperationPlanProof::new(
            self.receipt.clone(),
            self.counters.clone(),
            self.strategy,
            self.evidence_policy.clone(),
        )
    }

    pub(crate) fn into_query_handoff(self) -> WorthServerQueryHandoff {
        self.query_handoff
    }
}
