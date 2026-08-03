use worth_store_physical_backend::{ArtifactTreeFile, ArtifactTreeMedia};
use worth_store_wal::{
    inspect_interrupted_wal_segment_start, inspect_verified_wal_active_tail,
    inspect_verified_wal_segment, InterruptedWalSegmentStart, InterruptedWalTail,
    VerifiedWalSegment, WalSegmentArtifactIdentity, WalTopologyDenialKind,
};

use super::PhysicalWalOpenFailure;

pub(super) enum ActiveTailInspection<'segment> {
    Verified {
        prefix: VerifiedWalSegment<'segment>,
        interrupted_tail: Option<InterruptedActiveTail>,
    },
    InterruptedStart(InterruptedActiveSegmentCandidate),
}

pub(super) struct InterruptedActiveTail {
    artifact: ArtifactTreeFile,
    proof: InterruptedWalTail,
}

pub(super) struct InterruptedActiveSegmentCandidate {
    identity: WalSegmentArtifactIdentity,
    artifact: ArtifactTreeFile,
    proof: InterruptedWalSegmentStart,
}

pub(super) struct InterruptedActiveSegment {
    artifact: ArtifactTreeFile,
    proof: InterruptedWalSegmentStart,
}

pub(super) fn inspect<'segment>(
    identity: WalSegmentArtifactIdentity,
    artifact: &ArtifactTreeFile,
    bytes: &'segment [u8],
    is_active: bool,
) -> Result<ActiveTailInspection<'segment>, PhysicalWalOpenFailure> {
    if !is_active {
        let prefix = inspect_verified_wal_segment(identity, bytes)
            .map_err(PhysicalWalOpenFailure::SegmentInspection)?;
        return Ok(ActiveTailInspection::Verified {
            prefix,
            interrupted_tail: None,
        });
    }

    let admitted = match inspect_verified_wal_active_tail(identity, bytes) {
        Ok(admitted) => admitted,
        Err(denial) => {
            return match inspect_interrupted_wal_segment_start(identity, bytes) {
                Ok(proof) => Ok(ActiveTailInspection::InterruptedStart(
                    InterruptedActiveSegmentCandidate {
                        identity,
                        artifact: artifact.clone(),
                        proof,
                    },
                )),
                Err(_) => Err(PhysicalWalOpenFailure::SegmentInspection(denial)),
            };
        }
    };
    let interrupted_tail = admitted
        .interrupted_tail()
        .map(|proof| InterruptedActiveTail {
            artifact: artifact.clone(),
            proof,
        });
    Ok(ActiveTailInspection::Verified {
        prefix: admitted.into_verified_prefix(),
        interrupted_tail,
    })
}

impl InterruptedActiveSegmentCandidate {
    pub(super) fn admit_after(
        self,
        previous: WalSegmentArtifactIdentity,
    ) -> Result<InterruptedActiveSegment, PhysicalWalOpenFailure> {
        if self.identity.generation() != previous.generation() {
            return Err(PhysicalWalOpenFailure::Topology(
                WalTopologyDenialKind::WrongGeneration,
            ));
        }
        let expected = previous.segment().get().checked_add(1);
        if expected != Some(self.identity.segment().get()) {
            return Err(PhysicalWalOpenFailure::Topology(
                WalTopologyDenialKind::NonContiguousSegment,
            ));
        }
        Ok(InterruptedActiveSegment {
            artifact: self.artifact,
            proof: self.proof,
        })
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

impl InterruptedActiveSegment {
    pub(super) fn remove_durably(
        self,
        tree: &ArtifactTreeMedia<'_>,
    ) -> Result<(), PhysicalWalOpenFailure> {
        debug_assert!(self.proof.observed_bytes() > 0);
        tree.remove_file_durably(&self.artifact)
            .map_err(PhysicalWalOpenFailure::Media)
    }
}
