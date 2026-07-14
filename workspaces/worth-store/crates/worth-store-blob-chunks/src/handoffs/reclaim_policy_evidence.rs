use worth_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};
use worth_store_reclaim_policy::{
    ReclaimPolicyCounterSnapshot, ReclaimPolicyExecutionReceipt, ReclaimPolicyOperation,
};

use crate::BlobChunkSecurityMetadataWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReclaimPolicyEvidence {
    receipt: ReclaimPolicyExecutionReceipt,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobReclaimPolicyScopeDenial {
    receipt_scope: worth_store_security::StoreSecurityScopeIdentity,
    metadata_scope: worth_store_security::StoreSecurityScopeIdentity,
    receipt: worth_store_security::StoreSecurityScopeAdmissionReceipt,
    metadata_receipt: worth_store_security::StoreSecurityScopeAdmissionReceipt,
}

impl BlobReclaimPolicyEvidence {
    pub fn from_reclaim_receipt(
        receipt: ReclaimPolicyExecutionReceipt,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Result<Self, BlobReclaimPolicyScopeDenial> {
        let policy = receipt.policy();
        let receipt_scope = policy.security_scope().identity();
        let policy_receipt = policy.security_scope().receipt();
        if receipt_scope != metadata.identity() || policy_receipt != metadata.receipt() {
            return Err(BlobReclaimPolicyScopeDenial {
                receipt_scope,
                metadata_scope: metadata.identity(),
                receipt: policy_receipt,
                metadata_receipt: metadata.receipt(),
            });
        }

        Ok(Self {
            receipt,
            security_metadata: metadata,
        })
    }

    pub fn receipt(&self) -> &ReclaimPolicyExecutionReceipt {
        &self.receipt
    }

    pub fn region(&self) -> PhysicalReclaimRegion {
        self.receipt.policy().region()
    }

    pub fn interpretation(&self) -> ReclaimedByteInterpretation {
        self.receipt.observed_interpretation()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn security_scope(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }

    pub fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.receipt.counters()
    }

    pub const fn carries_blob_lifecycle_claim(&self) -> bool {
        false
    }

    pub const fn can_satisfy_blob_lifecycle_receipt(&self) -> bool {
        false
    }

    pub fn source_operation(receipt: &ReclaimPolicyExecutionReceipt) -> ReclaimPolicyOperation {
        receipt.policy().posture().operation()
    }
}

impl BlobReclaimPolicyScopeDenial {
    pub const fn receipt_scope(self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.receipt_scope
    }

    pub const fn metadata_scope(self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.metadata_scope
    }

    pub const fn receipt(self) -> worth_store_security::StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn metadata_receipt(
        self,
    ) -> worth_store_security::StoreSecurityScopeAdmissionReceipt {
        self.metadata_receipt
    }
}
