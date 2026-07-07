use crate::BlobChunkReachabilityProofSet;

use super::super::evidence::BlobPublicationCounterReceiptIdentity;
use super::super::types::reachability_staging::BlobReachabilityStagingIdentity;
use super::super::{BlobPublicationCounterSnapshot, BlobPublicationIntent};

pub(crate) fn from_intent_and_receipt(
    intent: &BlobPublicationIntent,
    reachability: &BlobChunkReachabilityProofSet,
) -> BlobReachabilityStagingIdentity {
    BlobReachabilityStagingIdentity {
        object_id: intent.object_id().clone(),
        generation: intent.generation(),
        chunk_tree_root: intent.chunk_tree_root().clone(),
        logical_content_digest: intent.logical_content_digest().clone(),
        security_metadata: reachability.security_metadata(),
        counter_receipt_identity: BlobPublicationCounterReceiptIdentity::from_reachability_staging(
            intent.counters(),
            reachability.counters(),
        ),
    }
}

pub(crate) fn with_reachability_staged_counters(
    intent: BlobPublicationIntent,
) -> (BlobPublicationIntent, BlobPublicationCounterSnapshot) {
    let counters = intent.counters().with_reachability_staged();
    (intent.with_counters(counters), counters)
}
