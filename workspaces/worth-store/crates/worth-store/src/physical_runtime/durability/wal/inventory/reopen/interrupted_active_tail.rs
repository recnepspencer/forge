use worth_store_physical_backend::{ArtifactTreeFile, ArtifactTreeMedia};
use worth_store_wal::{
    inspect_verified_wal_active_tail, inspect_verified_wal_segment, InterruptedWalTail,
    VerifiedWalSegment, WalSegmentArtifactIdentity,
};

use super::PhysicalWalOpenFailure;

pub(super) struct ActiveTailInspection<'segment> {
    verified_prefix: VerifiedWalSegment<'segment>,
    interrupted_tail: Option<InterruptedActiveTail>,
}

pub(super) struct InterruptedActiveTail {
    artifact: ArtifactTreeFile,
    proof: InterruptedWalTail,
}

pub(super) fn inspect<'segment>(
    identity: WalSegmentArtifactIdentity,
    artifact: &ArtifactTreeFile,
    bytes: &'segment [u8],
    is_active: bool,
) -> Result<ActiveTailInspection<'segment>, PhysicalWalOpenFailure> {
    if !is_active {
        let verified_prefix = inspect_verified_wal_segment(identity, bytes)
            .map_err(PhysicalWalOpenFailure::SegmentInspection)?;
        return Ok(ActiveTailInspection {
            verified_prefix,
            interrupted_tail: None,
        });
    }

    let admitted = inspect_verified_wal_active_tail(identity, bytes)
        .map_err(PhysicalWalOpenFailure::SegmentInspection)?;
    let interrupted_tail = admitted
        .interrupted_tail()
        .map(|proof| InterruptedActiveTail {
            artifact: artifact.clone(),
            proof,
        });
    Ok(ActiveTailInspection {
        verified_prefix: admitted.into_verified_prefix(),
        interrupted_tail,
    })
}

impl<'segment> ActiveTailInspection<'segment> {
    pub(super) fn into_parts(
        self,
    ) -> (VerifiedWalSegment<'segment>, Option<InterruptedActiveTail>) {
        (self.verified_prefix, self.interrupted_tail)
    }
}

impl InterruptedActiveTail {
    pub(super) fn truncate_durably(
        self,
        tree: &ArtifactTreeMedia<'_>,
    ) -> Result<(), PhysicalWalOpenFailure> {
        debug_assert!(self.proof.valid_prefix_bytes() < self.proof.observed_bytes());
        tree.truncate_file_durably(&self.artifact, self.proof.valid_prefix_bytes())
            .map_err(PhysicalWalOpenFailure::Media)
    }
}
