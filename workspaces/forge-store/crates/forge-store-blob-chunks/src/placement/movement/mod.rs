//! # STATE GRAPH
//!
//! Movement proof transitions consume evidence already admitted at placement and bind three
//! movement-specific lanes before physical execution or verified reads are exposed:
//!
//! - **Stable-read evidence** enters through [`BlobPlacementMovementReadHold`] (S5 stable read
//!   receipt plus chunk-migration interlock from physical-isolation).
//! - **Security-scope evidence** enters through [`BlobPlacementMovementForegroundReservation`]
//!   admitted scope identity matched against the lifecycle declaration.
//! - **Cold-tier posture evidence** enters through [`BlobPlacementMovementColdOutcome`], classified
//!   via tiering [`cold_posture_permits_movement`] before movement planning admits.
//!
//! **I/O scheduler admission** for placement cost enters upstream at
//! [`BlobPlacementAdmissionAuthority::admit`] via [`S7PlacementIoReadinessSeed`]; source and
//! target [`AdmittedBlobPlacement`] values consumed here already carry that admission outcome.
//!
//! Primary entry points:
//! - [`BlobPlacementMovementAuthority::plan_movement`] — classify eligibility via composed
//!   `require_*` verification steps, then construct movement plan.
//! - [`BlobMovementVerifiedReadEvidence::from_streaming_verified_read`] — collect verified read
//!   evidence against an admitted plan.
//! - [`StoreOwnedPlacementMovementExecution::execute_physical_movement`] — bind lower physical
//!   execution receipt to admitted plan basis and interlock.

mod classification;
mod counters;
mod crash_recovery;
mod denial;
mod orchestration;
mod performance;
mod receipt_construction;
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use types::{
    AdmittedBlobPlacementMovementPlan, BlobMovementReadPhase, BlobMovementVerifiedReadEvidence,
    BlobPlacementMovementAuthority, BlobPlacementMovementColdCapsuleOutcome,
    BlobPlacementMovementColdExportOutcome, BlobPlacementMovementColdMaterializationOutcome,
    BlobPlacementMovementColdOutcome, BlobPlacementMovementColdReadOutcome,
    BlobPlacementMovementForegroundReservation, BlobPlacementMovementFreshness,
    BlobPlacementMovementPhysicalExecutionIntent, BlobPlacementMovementReadHold,
    BlobPlacementMovementRequest, BlobReadDuringPlacementMove, BlobReadDuringPlacementMoveReceipt,
    ExecutedBlobPlacementMovementReceipt, PublishedBlobPlacementObservation,
    StoreOwnedPlacementMovementExecution, StoreOwnedPlacementMovementExecutionReceipt,
    StoreOwnedPlacementMovementPublication,
};
pub use counters::BlobPlacementMovementCounterSnapshot;
pub use crash_recovery::{BlobPlacementMovementResidue, BlobPlacementMovementRestartOutcome};
pub use denial::BlobPlacementMovementDenial;
pub(crate) use performance::counter_backed_placement_movement_performance_receipt;
pub use performance::BlobPlacementMovementCounterBackedPerformanceReceipt;