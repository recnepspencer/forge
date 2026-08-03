use worth_store_physical_backend::{ArtifactTreeDirectory, ArtifactTreeFile, ArtifactTreeMedia};
use worth_store_wal::{WalSegmentArtifactIdentity, WalTopologyDenialKind};

use super::{artifact, PhysicalWalOpenFailure};

pub(super) struct TrailingEmptySegment {
    artifact: ArtifactTreeFile,
}

pub(super) fn separate(
    tree: &ArtifactTreeMedia<'_>,
    directory: &ArtifactTreeDirectory,
    segments: &mut Vec<WalSegmentArtifactIdentity>,
) -> Result<Option<TrailingEmptySegment>, PhysicalWalOpenFailure> {
    let trailing = *segments
        .last()
        .expect("reopen calls trailing cleanup only for a nonempty inventory");
    let trailing_artifact = artifact(directory, trailing);
    let trailing_bytes = tree
        .file_length(&trailing_artifact)
        .map_err(PhysicalWalOpenFailure::Media)?;
    if trailing_bytes != 0 {
        return Ok(None);
    }

    let previous = segments
        .get(segments.len().saturating_sub(2))
        .copied()
        .ok_or(PhysicalWalOpenFailure::EmptySegment)?;
    if trailing.generation() != previous.generation() {
        return Err(PhysicalWalOpenFailure::Topology(
            WalTopologyDenialKind::WrongGeneration,
        ));
    }
    let expected_segment = previous.segment().get().checked_add(1);
    if expected_segment != Some(trailing.segment().get()) {
        return Err(PhysicalWalOpenFailure::Topology(
            WalTopologyDenialKind::NonContiguousSegment,
        ));
    }

    segments.pop();
    Ok(Some(TrailingEmptySegment {
        artifact: trailing_artifact,
    }))
}

impl TrailingEmptySegment {
    pub(super) fn remove_durably(
        self,
        tree: &ArtifactTreeMedia<'_>,
    ) -> Result<(), PhysicalWalOpenFailure> {
        tree.remove_file_durably(&self.artifact)
            .map_err(PhysicalWalOpenFailure::Media)
    }
}
