use crate::reachability::denial::BlobReachabilityDenial;
use crate::reachability::receipt_construction::BlobReachabilityReclaimRelease;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobReachabilityReclaimDecision {
    ReclaimPermitted(BlobReachabilityReclaimRelease),
    ReclaimDenied(BlobReachabilityDenial),
}