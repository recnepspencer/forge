use worth_store_physical_backend::ArtifactTreeFailure;
use worth_store_physical_format::{
    DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest, RecordArtifactFile,
};

use super::super::residency::serving_artifacts::ServingRecordArtifacts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecordPublicationResidueObservation {
    staging_catalog_candidate: bool,
    successor_root: bool,
    successor_routing_block: bool,
    successor_segment_membership_block: bool,
    successor_free_space_membership_block: bool,
    successor_free_space: bool,
    next_segment_data: bool,
    reusable_segment_data: bool,
    next_extent_data: bool,
    next_extent_manifest: bool,
}

impl RecordPublicationResidueObservation {
    pub const fn is_empty(self) -> bool {
        !(self.staging_catalog_candidate
            || self.successor_root
            || self.successor_routing_block
            || self.successor_segment_membership_block
            || self.successor_free_space_membership_block
            || self.successor_free_space
            || self.next_segment_data
            || self.reusable_segment_data
            || self.next_extent_data
            || self.next_extent_manifest)
    }

    pub const fn staging_catalog_candidate(self) -> bool {
        self.staging_catalog_candidate
    }
    pub const fn successor_root(self) -> bool {
        self.successor_root
    }
    pub const fn successor_routing_block(self) -> bool {
        self.successor_routing_block
    }
    pub const fn successor_free_space(self) -> bool {
        self.successor_free_space
    }
    pub const fn successor_membership_blocks(self) -> bool {
        self.successor_segment_membership_block || self.successor_free_space_membership_block
    }
    pub const fn next_segment_artifacts(self) -> bool {
        self.next_segment_data
    }
    pub const fn reusable_segment_artifacts(self) -> bool {
        self.reusable_segment_data
    }
    pub const fn next_extent_artifacts(self) -> bool {
        self.next_extent_data || self.next_extent_manifest
    }
}

pub(in crate::physical_runtime::record_serving) fn observe_publication_residue(
    artifacts: &ServingRecordArtifacts<'_>,
    current_root: &DurablePhysicalRootManifest,
    free_space: &DurableFreeSpaceManifestHeader,
    staging_catalog_candidate: bool,
) -> Result<RecordPublicationResidueObservation, ArtifactTreeFailure> {
    let Some(successor_generation) = current_root.generation().checked_add(1) else {
        return Ok(RecordPublicationResidueObservation {
            staging_catalog_candidate,
            ..RecordPublicationResidueObservation::default()
        });
    };
    let mut observation = RecordPublicationResidueObservation {
        staging_catalog_candidate,
        successor_root: exists(
            artifacts,
            RecordArtifactFile::RootManifest {
                generation: successor_generation,
            },
        )?,
        successor_routing_block: exists(
            artifacts,
            RecordArtifactFile::RootRoutingBlock {
                generation: successor_generation,
                block: current_root.next_block(),
            },
        )?,
        successor_segment_membership_block: exists(
            artifacts,
            RecordArtifactFile::SegmentMembershipBlock {
                generation: successor_generation,
                block: current_root.next_segment_block(),
            },
        )?,
        successor_free_space_membership_block: exists(
            artifacts,
            RecordArtifactFile::FreeSpaceMembershipBlock {
                generation: successor_generation,
                block: free_space.next_block(),
            },
        )?,
        successor_free_space: exists(
            artifacts,
            RecordArtifactFile::FreeSpaceManifest {
                generation: successor_generation,
            },
        )?,
        next_segment_data: exists(
            artifacts,
            RecordArtifactFile::Segment {
                segment: free_space.next_segment(),
                generation: 1,
            },
        )?,
        next_extent_data: exists(
            artifacts,
            RecordArtifactFile::Extent {
                extent: free_space.next_extent(),
                generation: 1,
            },
        )?,
        next_extent_manifest: exists(
            artifacts,
            RecordArtifactFile::ExtentManifest {
                extent: free_space.next_extent(),
                generation: 1,
            },
        )?,
        ..RecordPublicationResidueObservation::default()
    };
    if let Some(segment) = current_root.last_inline_segment() {
        if let Some(generation) = segment.generation().get().checked_add(1) {
            observation.reusable_segment_data = exists(
                artifacts,
                RecordArtifactFile::Segment {
                    segment: segment.segment_id().get(),
                    generation,
                },
            )?;
        }
    }
    Ok(observation)
}

fn exists(
    artifacts: &ServingRecordArtifacts<'_>,
    artifact: RecordArtifactFile,
) -> Result<bool, ArtifactTreeFailure> {
    artifacts.file_exists(artifact)
}
