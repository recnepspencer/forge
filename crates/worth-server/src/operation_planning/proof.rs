use super::{
    WorthServerOperationExecutionStrategy, WorthServerOperationPlanCounters,
    WorthServerOperationPlanEvidencePolicy, WorthServerOperationPlanReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationPlanProof {
    receipt: WorthServerOperationPlanReceipt,
    counters: WorthServerOperationPlanCounters,
    strategy: WorthServerOperationExecutionStrategy,
    evidence_policy: WorthServerOperationPlanEvidencePolicy,
    canonical_digest: String,
}

impl WorthServerOperationPlanProof {
    pub(crate) fn new(
        receipt: WorthServerOperationPlanReceipt,
        counters: WorthServerOperationPlanCounters,
        strategy: WorthServerOperationExecutionStrategy,
        evidence_policy: WorthServerOperationPlanEvidencePolicy,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-operation-plan-proof-v1|receipt={}|counters={}|strategy={}|evidence={}",
            receipt.canonical_digest(),
            counters.canonical_digest(),
            strategy.as_str(),
            evidence_policy.evidence_identity(),
        );
        Self {
            receipt,
            counters,
            strategy,
            evidence_policy,
            canonical_digest,
        }
    }

    pub fn receipt(&self) -> &WorthServerOperationPlanReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> &WorthServerOperationPlanCounters {
        &self.counters
    }

    pub fn strategy(&self) -> WorthServerOperationExecutionStrategy {
        self.strategy
    }

    pub fn evidence_policy(&self) -> &WorthServerOperationPlanEvidencePolicy {
        &self.evidence_policy
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
