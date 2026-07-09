//! # STATE GRAPH
//!
//! Placement admission is the I/O-readiness and backend-capability gate before movement or
//! compaction consume [`AdmittedBlobPlacement`]:
//!
//! - **I/O admission evidence** enters through [`BlobPlacementIntent::readiness`]
//!   ([`S7PlacementIoReadinessSeed`] from io-scheduler/tiering handoff). Verified by
//!   `verify_s6_readiness_readmitted` and `verify_readiness_basis_match` (security scope via
//!   cold-tier posture).
//! - **Reachability basis** enters from [`BlobChunkReachabilityProofSet`] and is matched against
//!   placement intent recoverability for external class.
//! - **Backend capability evidence** enters through per-class `verify_class_backend_capability`.
//!
//! Downstream movement assumes source/target placements admitted here; compaction intent carries
//! the same admitted placement witness.

mod basis;
mod counters;
mod denial;
mod intent;
mod non_claim;
mod orchestration;
mod receipt_construction;
mod types;
mod verification;

#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

pub use counters::BlobPlacementCounterSnapshot;
pub use denial::BlobPlacementAdmissionDenial;
pub use intent::{BlobPlacementClass, BlobPlacementIntent};
pub use non_claim::BlobPlacementNonClaim;
pub use orchestration::BlobPlacementAdmissionAuthority;
pub use types::AdmittedBlobPlacement;
