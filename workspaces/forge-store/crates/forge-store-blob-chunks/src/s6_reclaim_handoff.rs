use forge_store_physical_format::ReclaimedByteInterpretation;
use forge_store_reclaim_policy::{
    ReclaimPolicyCounterSnapshot, ReclaimPolicyExecutionReceipt, ReclaimPolicyOperation,
};
use forge_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6BlobReclaimNonClaimHandoff {
    interpretation: ReclaimedByteInterpretation,
    security_scope: StoreSecurityScopeIdentity,
    counters: ReclaimPolicyCounterSnapshot,
}

impl S6BlobReclaimNonClaimHandoff {
    pub fn from_reclaim_receipt(receipt: ReclaimPolicyExecutionReceipt) -> Self {
        let policy = receipt.policy();
        Self {
            interpretation: receipt.observed_interpretation(),
            security_scope: policy.security_scope().identity(),
            counters: receipt.counters(),
        }
    }

    pub const fn interpretation(self) -> ReclaimedByteInterpretation {
        self.interpretation
    }

    pub const fn security_scope(self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn counters(self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }

    pub const fn carries_blob_lifecycle_claim(self) -> bool {
        false
    }

    pub const fn can_satisfy_blob_lifecycle_receipt(self) -> bool {
        false
    }

    pub const fn source_operation(
        receipt: ReclaimPolicyExecutionReceipt,
    ) -> ReclaimPolicyOperation {
        receipt.policy().posture().operation()
    }
}
