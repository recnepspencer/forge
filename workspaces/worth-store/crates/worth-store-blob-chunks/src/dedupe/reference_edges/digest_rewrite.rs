use worth_store_contracts::StableDigest;

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::BlobChunkDedupeDigestRewriteBasis;

pub(super) fn executed_digest_rewrite(
    receipt: &BlobChunkCollisionVerificationReceipt,
    rewrite_basis: StableDigest,
) -> BlobChunkDedupeDigestRewriteBasis {
    BlobChunkDedupeDigestRewriteBasis::from_executed_rewrite(receipt, rewrite_basis)
}
