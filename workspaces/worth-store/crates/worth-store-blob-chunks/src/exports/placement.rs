// --- Capabilities (admission handles, next-step types) ---
pub use crate::placement::{
    AdmittedBlobPlacement, AdmittedBlobPlacementMovementPlan, BlobPlacementAdmissionAuthority,
    BlobPlacementIntent, BlobPlacementMovementAuthority,
    BlobPlacementMovementForegroundReservation, BlobPlacementMovementPhysicalExecutionIntent,
    BlobPlacementMovementRequest, BlobPlacementNonClaim, BlobPlacementProof,
    BlobReadDuringPlacementMove, StoreOwnedPlacementMovementExecution,
    StoreOwnedPlacementMovementPublication,
};
// --- Outcomes (transition receipts) ---
pub use crate::placement::{
    BlobMovementReadPhase, BlobMovementVerifiedReadEvidence,
    BlobPlacementMovementColdCapsuleOutcome, BlobPlacementMovementColdExportOutcome,
    BlobPlacementMovementColdMaterializationOutcome, BlobPlacementMovementColdOutcome,
    BlobPlacementMovementColdReadOutcome, BlobPlacementMovementCounterBackedPerformanceReceipt,
    BlobPlacementMovementFreshness, BlobPlacementMovementReadHold, BlobPlacementMovementResidue,
    BlobPlacementMovementRestartOutcome, BlobReadDuringPlacementMoveReceipt,
    ExecutedBlobPlacementMovementReceipt, PublishedBlobPlacementObservation,
    StoreOwnedPlacementMovementExecutionReceipt,
};
// --- Denials (classified failure enums) ---
pub use crate::placement::{
    BlobPlacementAdmissionDenial, BlobPlacementClass, BlobPlacementMovementDenial,
};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::placement::{BlobPlacementCounterSnapshot, BlobPlacementMovementCounterSnapshot};
