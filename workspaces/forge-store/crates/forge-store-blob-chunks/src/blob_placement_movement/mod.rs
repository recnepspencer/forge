mod cold_lane;
mod counters;
mod crash_recovery;
mod denial;
mod execution;
mod performance;
mod plan;
mod read_during_move;
mod read_hold;
mod request;

#[doc(hidden)]
pub mod compile_fail;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use cold_lane::{
    BlobPlacementMovementColdCapsuleOutcome, BlobPlacementMovementColdExportOutcome,
    BlobPlacementMovementColdMaterializationOutcome, BlobPlacementMovementColdOutcome,
    BlobPlacementMovementColdReadOutcome,
};
pub use counters::BlobPlacementMovementCounterSnapshot;
pub use crash_recovery::{BlobPlacementMovementResidue, BlobPlacementMovementRestartOutcome};
pub use denial::BlobPlacementMovementDenial;
pub use execution::{
    ExecutedBlobPlacementMovementReceipt, PublishedBlobPlacementObservation,
    StoreOwnedPlacementMovementExecution, StoreOwnedPlacementMovementExecutionReceipt,
    StoreOwnedPlacementMovementPublication,
};
pub(crate) use performance::counter_backed_placement_movement_performance_receipt;
pub use performance::BlobPlacementMovementCounterBackedPerformanceReceipt;
pub use plan::{
    AdmittedBlobPlacementMovementPlan, BlobPlacementMovementAuthority,
    BlobPlacementMovementFreshness, BlobPlacementMovementPhysicalExecutionIntent,
};
pub use read_during_move::{
    BlobMovementReadPhase, BlobMovementVerifiedReadEvidence, BlobReadDuringPlacementMove,
    BlobReadDuringPlacementMoveReceipt,
};
pub use read_hold::BlobPlacementMovementReadHold;
pub use request::{BlobPlacementMovementForegroundReservation, BlobPlacementMovementRequest};
