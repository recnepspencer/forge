use crate::BlobChunkReachabilityProofSet;

use super::super::{BlobPublicationIntent, BlobPublicationDenial};

pub(crate) fn matches_publication_intent(
    reachability: &BlobChunkReachabilityProofSet,
    intent: &BlobPublicationIntent,
    counters: super::super::BlobPublicationCounterSnapshot,
) -> Result<(), BlobPublicationDenial> {
    if reachability.matches_publication_intent(intent) {
        Ok(())
    } else {
        Err(BlobPublicationDenial::ReachabilityDigestMismatch { counters })
    }
}