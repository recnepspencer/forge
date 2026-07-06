use forge_store_contracts::StableDigest;

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::BlobChunkDedupeCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkDedupeIndexPartition {
    counters: BlobChunkDedupeCounterSnapshot,
    partition_basis: StableDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkDedupeDigestRewriteBasis {
    counters: BlobChunkDedupeCounterSnapshot,
    rewrite_basis: StableDigest,
}

impl BlobChunkDedupeIndexPartition {
    pub(crate) fn from_executed_partition(
        receipt: &BlobChunkCollisionVerificationReceipt,
        partition_basis: StableDigest,
    ) -> Self {
        Self {
            counters: receipt.counters().record_index_partition_denial(),
            partition_basis,
        }
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub const fn partition_basis(&self) -> &StableDigest {
        &self.partition_basis
    }
}

impl BlobChunkDedupeDigestRewriteBasis {
    pub(crate) fn from_executed_rewrite(
        receipt: &BlobChunkCollisionVerificationReceipt,
        rewrite_basis: StableDigest,
    ) -> Self {
        Self {
            counters: receipt.counters().record_digest_rewrite(),
            rewrite_basis,
        }
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub const fn rewrite_basis(&self) -> &StableDigest {
        &self.rewrite_basis
    }
}
