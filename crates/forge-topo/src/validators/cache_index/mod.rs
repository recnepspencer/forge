//! Cache and index consistency validators.
//!
//! DOMAIN: Validates that reverse indexes (face→halfedges,
//! shell→faces, vertex→halfedges) match loop-walked ground truth.
//!
//! VALIDATORS (from validators.md §13):
//! - ValidateAdjacencyCacheMatchesGroundTruth

mod index_coherence;

pub(crate) use index_coherence::validate_index_coherence;
