use crate::BlobStreamingVerifiedRead;

use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    types::{
        plan::AdmittedBlobPlacementMovementPlan, read_during_move::BlobMovementVerifiedReadEvidence,
        read_hold::BlobPlacementMovementReadHold,
    },
    verification::verified_read_basis::verify_streaming_read_matches_admitted_plan,
};

pub(crate) fn transition_collect_verified_read_evidence(
    plan: &AdmittedBlobPlacementMovementPlan,
    read_hold: BlobPlacementMovementReadHold,
    streaming_read: &BlobStreamingVerifiedRead,
) -> Result<BlobMovementVerifiedReadEvidence, BlobPlacementMovementDenial> {
    verify_streaming_read_matches_admitted_plan(plan, read_hold, streaming_read)?;
    Ok(BlobMovementVerifiedReadEvidence::from_basis(
        plan.basis(),
        streaming_read.counters().bytes_read(),
    ))
}