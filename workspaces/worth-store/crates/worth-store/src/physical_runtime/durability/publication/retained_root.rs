use worth_store_physical_format::{DurablePhysicalRootManifest, RecordArtifactFile};

pub struct RetainedPhysicalRoot {
    manifest: DurablePhysicalRootManifest,
    supporting_artifacts: Box<[RecordArtifactFile]>,
}

impl RetainedPhysicalRoot {
    pub(super) fn from_manifest(manifest: DurablePhysicalRootManifest) -> Self {
        let generation = manifest.generation();
        let mut artifacts = Vec::with_capacity(5);
        artifacts.push(RecordArtifactFile::RootManifest { generation });
        artifacts.push(RecordArtifactFile::FreeSpaceManifest { generation });
        if let Some(reference) = manifest.routing_root() {
            artifacts.push(RecordArtifactFile::RootRoutingBlock {
                generation: reference.generation(),
                block: reference.block(),
            });
        }
        if let Some(reference) = manifest.segment_root() {
            artifacts.push(RecordArtifactFile::SegmentMembershipBlock {
                generation: reference.generation(),
                block: reference.block(),
            });
        }
        if let Some(reference) = manifest.free_space_root() {
            artifacts.push(RecordArtifactFile::FreeSpaceMembershipBlock {
                generation: reference.generation(),
                block: reference.block(),
            });
        }
        artifacts.sort_unstable();
        artifacts.dedup();
        Self {
            manifest,
            supporting_artifacts: artifacts.into_boxed_slice(),
        }
    }

    pub const fn manifest(&self) -> &DurablePhysicalRootManifest {
        &self.manifest
    }

    pub fn supporting_artifacts(&self) -> &[RecordArtifactFile] {
        &self.supporting_artifacts
    }
}
