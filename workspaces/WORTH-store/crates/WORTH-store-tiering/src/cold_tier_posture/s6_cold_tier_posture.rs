use worth_store_physical_format::ReclaimedByteInterpretation;
use worth_store_reclaim_policy::{
    ReclaimPolicyCounterSnapshot, ReclaimPolicyExecutionReceipt, ReclaimPolicyOperation,
};
use worth_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6ColdTierIoPosture {
    interpretation: ReclaimedByteInterpretation,
    security_scope: StoreSecurityScopeIdentity,
    counters: ReclaimPolicyCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6ColdTierIoPostureDenial {
    NotColdTierIoPosture,
}

impl S6ColdTierIoPosture {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_test_authority(
        security_scope: StoreSecurityScopeIdentity,
        counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self {
            interpretation: ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            security_scope,
            counters,
        }
    }

    pub fn from_reclaim_receipt(
        receipt: ReclaimPolicyExecutionReceipt,
    ) -> Result<Self, S6ColdTierIoPostureDenial> {
        let policy = receipt.policy();
        if policy.posture().operation() != ReclaimPolicyOperation::ColdTierMovementPosture {
            return Err(S6ColdTierIoPostureDenial::NotColdTierIoPosture);
        }
        Ok(Self {
            interpretation: receipt.observed_interpretation(),
            security_scope: policy.security_scope().identity(),
            counters: receipt.counters(),
        })
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

    pub const fn carries_tier_placement_claim(self) -> bool {
        false
    }

    pub const fn carries_compaction_claim(self) -> bool {
        false
    }
}
