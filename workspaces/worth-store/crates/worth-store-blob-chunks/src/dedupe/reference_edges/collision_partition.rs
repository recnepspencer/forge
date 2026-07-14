use worth_store_contracts::StableDigest;

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::BlobChunkDedupeIndexPartition;

pub(super) fn executed_collision_partition(
    receipt: &BlobChunkCollisionVerificationReceipt,
    partition_basis: StableDigest,
) -> BlobChunkDedupeIndexPartition {
    BlobChunkDedupeIndexPartition::from_executed_partition(receipt, partition_basis)
}
