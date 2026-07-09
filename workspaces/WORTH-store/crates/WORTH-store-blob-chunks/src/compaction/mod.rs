//! # STATE GRAPH
//!
//! Compaction rewrite admission composes physical interlock, reachability, placement, and pacing
//! evidence before rewrite execution:
//!
//! - **Stable-read evidence** enters through released [`BlobCompactionReadHold`] matched to
//!   [`CompactionReadInterlockPlan`].
//! - **I/O admission evidence** enters through [`BlobCompactionS6Pacing`] — prefer
//!   [`BlobCompactionS6Pacing::from_s10_handoff`] from [`S10CompactionIoReadinessHandoff`] with
//!   readmission verification; certification paths may use [`BlobCompactionS6Pacing::admitted_compaction`].
//! - **Cold-tier posture** enters through [`BlobCompactionColdReadiness`] classified via tiering
//!   [`cold_posture_permits_compaction`].
//! - **Placement witness** must match lifecycle reachability via admitted [`AdmittedBlobPlacement`].

mod classification;
mod counters;
mod denial;
mod equivalence;
mod orchestration;
mod receipt_construction;
mod recovery;
mod rewrite_binding;
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use counters::BlobCompactionCounterSnapshot;
pub use denial::BlobCompactionDenial;
pub use equivalence::BlobCompactionEquivalence;
pub use orchestration::BlobCompactionAuthority;
pub use receipt_construction::published_observation::BlobCompactionPublishedObservation;
pub use recovery::{BlobCompactionResidue, BlobCompactionRestartOutcome};
pub use transitions::execute_rewrite::BlobCompactionRewriteExecution;
pub use types::{
    BlobCompactionColdReadiness, BlobCompactionIntent, BlobCompactionPhysicalInterlock,
    BlobCompactionReadHold, BlobCompactionRewritePlan, BlobCompactionS6Pacing,
};
