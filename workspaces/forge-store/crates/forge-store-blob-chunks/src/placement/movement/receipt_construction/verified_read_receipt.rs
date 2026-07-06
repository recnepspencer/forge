use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot,
    types::{
        basis::BlobPlacementMovementBasis, read_during_move::BlobMovementReadPhase,
        read_during_move::BlobReadDuringPlacementMoveReceipt,
    },
};

pub(crate) fn construct_verified_read_receipt(
    basis: BlobPlacementMovementBasis,
    phase: BlobMovementReadPhase,
    verified_bytes: u64,
    counters: BlobPlacementMovementCounterSnapshot,
) -> BlobReadDuringPlacementMoveReceipt {
    BlobReadDuringPlacementMoveReceipt {
        basis,
        phase,
        verified_bytes,
        counters,
    }
}