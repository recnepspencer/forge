use crate::{
    blob_lifecycle_authority::BlobLifecycleExecutedRecipe, BlobChunkReachabilityProofSet,
    BlobChunkSecurityMetadataWitness, BlobLifecycleCounterSnapshot, BlobLifecycleDeclaration,
    BlobPlacementProof, LogicalContentDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LifecycleReceipt {
    reachability: BlobChunkReachabilityProofSet,
    placement: BlobPlacementProof,
    counters: BlobLifecycleCounterSnapshot,
    executed_proof: BlobLifecycleExecutedRecipe,
}

impl LifecycleReceipt {
    pub(crate) fn new(
        reachability: BlobChunkReachabilityProofSet,
        placement: BlobPlacementProof,
        counters: BlobLifecycleCounterSnapshot,
        executed_proof: BlobLifecycleExecutedRecipe,
    ) -> Self {
        Self {
            reachability,
            placement,
            counters,
            executed_proof,
        }
    }

    pub(crate) fn declaration(&self) -> &BlobLifecycleDeclaration {
        self.executed_proof.payload().declaration()
    }

    pub const fn reachability(&self) -> &BlobChunkReachabilityProofSet {
        &self.reachability
    }

    pub const fn placement(&self) -> &BlobPlacementProof {
        &self.placement
    }

    pub const fn counters(&self) -> BlobLifecycleCounterSnapshot {
        self.counters
    }

    pub fn dedupe_receipt(&self) -> BlobDedupeReceipt {
        BlobDedupeReceipt::from_lifecycle(self)
    }

    pub fn reachability_receipt(&self) -> BlobReachabilityReceipt {
        BlobReachabilityReceipt::from_lifecycle(self)
    }

    pub fn resumability_receipt(&self) -> BlobResumabilityReceipt {
        BlobResumabilityReceipt::from_lifecycle(self)
    }

    pub fn retention_receipt(&self) -> BlobRetentionReceipt {
        BlobRetentionReceipt::from_lifecycle(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobDedupeReceipt {
    logical_content_digest: LogicalContentDigest,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobDedupeReceipt {
    pub(crate) fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        Self {
            logical_content_digest: receipt.declaration().logical_content_digest().clone(),
            counters: receipt.counters(),
        }
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counters(&self) -> BlobLifecycleCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobReachabilityReceipt {
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobReachabilityReceipt {
    pub(crate) fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        Self {
            logical_content_digest: receipt.declaration().logical_content_digest().clone(),
            security_metadata: receipt.reachability().security_metadata(),
            counters: receipt.counters(),
        }
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn counters(&self) -> BlobLifecycleCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobResumabilityReceipt {
    logical_content_digest: LogicalContentDigest,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobResumabilityReceipt {
    pub(crate) fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        Self {
            logical_content_digest: receipt.declaration().logical_content_digest().clone(),
            counters: receipt.counters(),
        }
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counters(&self) -> BlobLifecycleCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobRetentionReceipt {
    logical_content_digest: LogicalContentDigest,
    counters: BlobLifecycleCounterSnapshot,
}

impl BlobRetentionReceipt {
    pub(crate) fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        Self {
            logical_content_digest: receipt.declaration().logical_content_digest().clone(),
            counters: receipt.counters(),
        }
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counters(&self) -> BlobLifecycleCounterSnapshot {
        self.counters
    }
}
