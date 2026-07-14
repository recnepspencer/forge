//! # STATE GRAPH
//!
//! Placement admission is the I/O-readiness and backend-capability gate before movement or
//! compaction consume [`AdmittedBlobPlacement`]:
//!
//! - **I/O admission evidence** enters through [`BlobPlacementIntent::readiness`] as an ordinary
//!   scheduler isolation admission paired with cold-tier posture. The placement boundary verifies
//!   its security scope against the reachability basis.
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

#[cfg(any(test, feature = "certification-test-authority"))]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

pub use counters::BlobPlacementCounterSnapshot;
pub use denial::BlobPlacementAdmissionDenial;
pub use intent::{BlobPlacementClass, BlobPlacementIntent};
pub use non_claim::BlobPlacementNonClaim;
pub use orchestration::BlobPlacementAdmissionAuthority;
pub use types::AdmittedBlobPlacement;
