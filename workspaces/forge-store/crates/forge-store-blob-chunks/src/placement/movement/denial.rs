use forge_store_io_scheduler::foreground_reservation::ReservationViolatedWithCause;
use forge_store_tiering::S7ColdPlacementState;

use super::BlobPlacementMovementCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementDenial {
    StaleMovementPlan {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    MissingMovementReadHold {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    LifecycleSourcePlacementBasisMismatch {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    LifecycleTargetPlacementBasisMismatch {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    ForegroundReservationViolated {
        violation: ReservationViolatedWithCause,
        counters: BlobPlacementMovementCounterSnapshot,
    },
    ForegroundReservationScopeMismatch {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    ColdPlacementUnavailable {
        state: S7ColdPlacementState,
        counters: BlobPlacementMovementCounterSnapshot,
    },
    MovementExecutionReceiptMismatch {
        counters: BlobPlacementMovementCounterSnapshot,
    },
    VerifiedReadBasisMismatch {
        counters: BlobPlacementMovementCounterSnapshot,
    },
}
