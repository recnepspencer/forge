use crate::BlobStreamingVerifiedRead;

use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    transitions::{
        admit_verified_read::transition_admit_verified_read,
        collect_verified_read_evidence::transition_collect_verified_read_evidence,
    },
    types::{
        AdmittedBlobPlacementMovementPlan, BlobMovementVerifiedReadEvidence,
        BlobPlacementMovementReadHold, BlobReadDuringPlacementMove,
        BlobReadDuringPlacementMoveReceipt,
    },
};

impl BlobMovementVerifiedReadEvidence {
    pub fn from_streaming_verified_read(
        basis: &AdmittedBlobPlacementMovementPlan,
        read_hold: BlobPlacementMovementReadHold,
        streaming_read: &BlobStreamingVerifiedRead,
    ) -> Result<Self, BlobPlacementMovementDenial> {
        transition_collect_verified_read_evidence(basis, read_hold, streaming_read)
    }
}

impl BlobReadDuringPlacementMove {
    pub fn admit_verified_read(
        self,
        read: BlobMovementVerifiedReadEvidence,
    ) -> Result<BlobReadDuringPlacementMoveReceipt, BlobPlacementMovementDenial> {
        transition_admit_verified_read(self, read)
    }
}
