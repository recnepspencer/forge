use crate::{BlobResumabilityReceipt, LogicalContentDigest};

use super::super::{BlobPublicationCounterSnapshot, BlobPublicationDenial, BlobPublicationIntent};

pub(crate) fn verify_resumability_digest(
    intent: &BlobPublicationIntent,
    resumability_receipt: &BlobResumabilityReceipt,
    counters: BlobPublicationCounterSnapshot,
) -> Result<(), BlobPublicationDenial> {
    if intent.logical_content_digest() == resumability_receipt.logical_content_digest() {
        Ok(())
    } else {
        Err(BlobPublicationDenial::ReachabilityDigestMismatch { counters })
    }
}

#[allow(dead_code)]
pub(crate) fn resumability_digest(receipt: &BlobResumabilityReceipt) -> LogicalContentDigest {
    receipt.logical_content_digest().clone()
}
