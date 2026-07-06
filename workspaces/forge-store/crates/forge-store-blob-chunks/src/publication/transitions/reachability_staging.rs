use crate::BlobChunkReachabilityProofSet;

use super::super::receipt_construction::staging_identity;
use super::super::types::reachability_staging::BlobReachabilityStaging;
use super::super::types::root_candidate::BlobRootCandidateForPublication;
use super::super::verification::reachability_match;
use super::super::{BlobPublicationDenial, BlobPublicationIntent};

pub(crate) fn stage(
    candidate: BlobRootCandidateForPublication,
    reachability: BlobChunkReachabilityProofSet,
) -> Result<BlobReachabilityStaging, BlobPublicationDenial> {
    let intent = candidate.into_intent();
    let counters = intent.counters();
    reachability_match::matches_publication_intent(&reachability, &intent, counters)?;
    let (staged_intent, _) = staging_identity::with_reachability_staged_counters(intent);
    Ok(BlobReachabilityStaging {
        staging_identity: staging_identity::from_intent_and_receipt(&staged_intent, &reachability),
        staged_digest: staged_intent.logical_content_digest().clone(),
        security_metadata: reachability.security_metadata(),
        reachability_counters: reachability.counters(),
        intent: staged_intent,
    })
}

#[allow(dead_code)]
type _Intent = BlobPublicationIntent;