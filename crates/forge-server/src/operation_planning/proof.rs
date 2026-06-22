use super::{
    ForgeServerOperationExecutionStrategy, ForgeServerOperationPlanCounters,
    ForgeServerOperationPlanEvidencePolicy, ForgeServerOperationPlanReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationPlanProof {
    receipt: ForgeServerOperationPlanReceipt,
    counters: ForgeServerOperationPlanCounters,
    strategy: ForgeServerOperationExecutionStrategy,
    evidence_policy: ForgeServerOperationPlanEvidencePolicy,
    canonical_digest: String,
}

impl ForgeServerOperationPlanProof {
    pub(crate) fn new(
        receipt: ForgeServerOperationPlanReceipt,
        counters: ForgeServerOperationPlanCounters,
        strategy: ForgeServerOperationExecutionStrategy,
        evidence_policy: ForgeServerOperationPlanEvidencePolicy,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-operation-plan-proof-v1|receipt={}|counters={}|strategy={}|evidence={}",
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

    pub fn receipt(&self) -> &ForgeServerOperationPlanReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> &ForgeServerOperationPlanCounters {
        &self.counters
    }

    pub fn strategy(&self) -> ForgeServerOperationExecutionStrategy {
        self.strategy
    }

    pub fn evidence_policy(&self) -> &ForgeServerOperationPlanEvidencePolicy {
        &self.evidence_policy
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
