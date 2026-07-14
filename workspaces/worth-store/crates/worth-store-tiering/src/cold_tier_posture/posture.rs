use worth_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};
use worth_store_reclaim_policy::{
    ReclaimPermit, ReclaimPolicyCounterSnapshot, ReclaimPolicyExecutionReceipt,
    ReclaimPolicyOperation,
};
use worth_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColdTierIoPosture {
    receipt: ReclaimPolicyExecutionReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColdTierIoPostureDenial {
    NotColdTierIoPosture,
}

impl ColdTierIoPosture {
    pub fn from_reclaim_receipt(
        receipt: ReclaimPolicyExecutionReceipt,
    ) -> Result<Self, ColdTierIoPostureDenial> {
        if receipt.policy().posture().operation() != ReclaimPolicyOperation::ColdTierMovementPosture
        {
            return Err(ColdTierIoPostureDenial::NotColdTierIoPosture);
        }
        Ok(Self { receipt })
    }

    pub const fn reclaim_receipt(&self) -> &ReclaimPolicyExecutionReceipt {
        &self.receipt
    }

    pub fn interpretation(&self) -> ReclaimedByteInterpretation {
        self.receipt.observed_interpretation()
    }

    pub fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.receipt.policy().security_scope().identity()
    }

    pub fn reclaim_region(&self) -> PhysicalReclaimRegion {
        self.receipt.policy().region()
    }

    pub fn reclaim_permit(&self) -> ReclaimPermit {
        self.receipt.policy().permit()
    }

    pub fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.receipt.counters()
    }
}
