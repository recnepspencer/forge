use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    receipt_construction::verified_read_receipt::construct_verified_read_receipt,
    types::read_during_move::{
        BlobMovementVerifiedReadEvidence, BlobReadDuringPlacementMove,
        BlobReadDuringPlacementMoveReceipt,
    },
    verification::verified_read_basis::verify_verified_read_matches_guard_basis,
};

pub(crate) fn transition_admit_verified_read(
    guard: BlobReadDuringPlacementMove,
    read: BlobMovementVerifiedReadEvidence,
) -> Result<BlobReadDuringPlacementMoveReceipt, BlobPlacementMovementDenial> {
    verify_verified_read_matches_guard_basis(&guard, &read)?;
    Ok(construct_verified_read_receipt(
        guard.basis,
        guard.phase,
        read.verified_bytes(),
        guard.counters,
    ))
}
