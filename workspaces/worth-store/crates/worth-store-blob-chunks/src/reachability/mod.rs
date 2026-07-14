mod classification;
mod counters;
mod denial;
mod edges;
mod holds;
mod orchestration;
mod proof;
mod receipt_construction;
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod authority_tests;
#[cfg(test)]
mod checkpoint_tests;
#[cfg(test)]
mod dedupe_release_tests;
#[cfg(test)]
pub(crate) mod hold_test_support;
#[cfg(test)]
mod tests;

pub use counters::BlobReachabilityCounterSnapshot;
pub use denial::{
    reject_backend_residue_as_blob_reachability, reject_copied_refcount_row_as_reachability,
    reject_empty_reference_proof_as_reachability, reject_terminal_projection_as_blob_reachability,
    BlobReachabilityDenial,
};
pub(crate) use edges::BlobReachabilityAuthorityKey;
pub use edges::{BlobReachabilityEdge, BlobReachabilityEdgeKind};
pub use holds::BlobReachabilityProtectedHold;
pub use proof::BlobReachabilityProof;
pub use receipt_construction::{
    BlobReachabilityCanonicalSnapshot, BlobReachabilityEdgeRelease, BlobReachabilityReclaimRelease,
};
pub use types::{
    BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry, BlobReachabilityReclaimDecision,
};
