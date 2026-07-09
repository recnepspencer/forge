use crate::reachability::classification::ReachabilityReclaimCase;
use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::reachability::denial::BlobReachabilityDenial;
use crate::reachability::receipt_construction::reclaim_release::construct_reclaim_release;
use crate::reachability::types::{BlobChunkReachabilityRegistry, BlobReachabilityReclaimDecision};
use crate::BlobChunkIdentity;

pub(crate) fn assemble_reclaim_decision(
    identity: &BlobChunkIdentity,
    case: ReachabilityReclaimCase,
    base_counters: BlobReachabilityCounterSnapshot,
) -> BlobReachabilityReclaimDecision {
    let counters = base_counters.with_classified_reclaim_outcome(&case);
    match case {
        ReachabilityReclaimCase::Reachable | ReachabilityReclaimCase::Held => {
            BlobReachabilityReclaimDecision::ReclaimDenied(
                BlobReachabilityDenial::ReclaimBlockedByReferenceEdge { counters },
            )
        }
        ReachabilityReclaimCase::DeniedMissingRelease => {
            BlobReachabilityReclaimDecision::ReclaimDenied(
                BlobReachabilityDenial::MissingReclaimReleaseEvidence { counters },
            )
        }
        ReachabilityReclaimCase::Reclaimable { released_edges } => {
            BlobReachabilityReclaimDecision::ReclaimPermitted(construct_reclaim_release(
                identity.clone(),
                released_edges,
                counters,
            ))
        }
    }
}

pub(crate) fn classify_reclaim_for_identity(
    registry: &BlobChunkReachabilityRegistry,
    identity: &BlobChunkIdentity,
) -> BlobReachabilityReclaimDecision {
    let case =
        crate::reachability::classification::classify_reclaim_eligibility(registry, identity);
    assemble_reclaim_decision(identity, case, registry.stored_counters())
}
