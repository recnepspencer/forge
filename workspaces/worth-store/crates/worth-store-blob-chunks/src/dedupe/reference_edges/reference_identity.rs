use worth_store_contracts::StableDigest;

use crate::BlobChunkDedupeReceipt;

pub(super) fn dedupe_reference_identity(
    receipt: &BlobChunkDedupeReceipt,
    ordinal: u64,
) -> StableDigest {
    StableDigest::new(format!(
        "blob.dedupe.reference:{}:{}:{}:{}",
        receipt.existing_identity().chunk_digest().as_str(),
        receipt.candidate_identity().chunk_digest().as_str(),
        receipt.content_digest().as_str(),
        ordinal
    ))
    .expect("dedupe reference identity is nonempty")
}
