use crate::BlobStreamingVerifiedRead;

use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot, denial::BlobPlacementMovementDenial,
    types::{
        basis::BlobPlacementMovementBasis, plan::AdmittedBlobPlacementMovementPlan,
        read_during_move::{BlobMovementVerifiedReadEvidence, BlobReadDuringPlacementMove},
        read_hold::BlobPlacementMovementReadHold,
    },
};

pub(crate) fn verify_streaming_read_matches_movement_basis(
    basis: &BlobPlacementMovementBasis,
    read_hold: BlobPlacementMovementReadHold,
    streaming_read: &BlobStreamingVerifiedRead,
    counters: BlobPlacementMovementCounterSnapshot,
) -> Result<(), BlobPlacementMovementDenial> {
    if streaming_read.object_id() == basis.object_id()
        && streaming_read.generation() == basis.generation()
        && streaming_read.chunk_tree_root() == basis.chunk_tree_root()
        && streaming_read.logical_content_digest() == basis.logical_content_digest()
        && streaming_read.counters().bytes_read() <= read_hold.guarded_bytes()
    {
        return Ok(());
    }
    Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch {
        counters: counters.record_protected_denial(),
    })
}

pub(crate) fn verify_verified_read_matches_guard_basis(
    guard: &BlobReadDuringPlacementMove,
    read: &BlobMovementVerifiedReadEvidence,
) -> Result<(), BlobPlacementMovementDenial> {
    if guard.basis.matches_verified_basis(read) {
        return Ok(());
    }
    Err(BlobPlacementMovementDenial::VerifiedReadBasisMismatch {
        counters: guard.counters.record_protected_denial(),
    })
}

pub(crate) fn verify_streaming_read_matches_admitted_plan(
    plan: &AdmittedBlobPlacementMovementPlan,
    read_hold: BlobPlacementMovementReadHold,
    streaming_read: &BlobStreamingVerifiedRead,
) -> Result<(), BlobPlacementMovementDenial> {
    verify_streaming_read_matches_movement_basis(
        plan.basis(),
        read_hold,
        streaming_read,
        plan.counters(),
    )
}