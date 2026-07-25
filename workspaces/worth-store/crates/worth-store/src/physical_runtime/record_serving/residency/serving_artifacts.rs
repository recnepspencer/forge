use worth_store_physical_backend::{ArtifactTreeFailure, QualifiedFilesystemMedia};
use worth_store_physical_format::RecordArtifactFile;

use super::{
    artifact_tree::{PhysicalRecordArtifactTree, RecordFamilyInventory},
    frame_loading::{FrameLoadFailure, LoadedPhysicalFrame},
    frame_ports::FrameLoadPort,
    record_frame_reader::RecordFrameReader,
};

pub(in crate::physical_runtime::record_serving) struct ServingRecordArtifacts<'media> {
    tree: PhysicalRecordArtifactTree<'media>,
    reader: RecordFrameReader<'media>,
}

impl<'media> ServingRecordArtifacts<'media> {
    pub(in crate::physical_runtime::record_serving) fn new(
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn FrameLoadPort + Send + Sync),
    ) -> Self {
        Self {
            tree: PhysicalRecordArtifactTree::new(media),
            reader: RecordFrameReader::bootstrap(media, loader),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn inventory(
        &self,
    ) -> Result<RecordFamilyInventory, ArtifactTreeFailure> {
        self.tree.inventory()
    }

    pub(in crate::physical_runtime::record_serving) fn has_staging_residue(
        &self,
    ) -> Result<bool, ArtifactTreeFailure> {
        self.tree.has_staging_residue()
    }

    pub(in crate::physical_runtime::record_serving) fn file_exists(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<bool, ArtifactTreeFailure> {
        self.tree.file_exists(artifact)
    }

    pub(in crate::physical_runtime::record_serving) fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<u64, FrameLoadFailure> {
        self.reader
            .file_length(artifact)
            .map(|length| length.bytes())
    }

    pub(in crate::physical_runtime::record_serving) fn load_exact(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        self.reader.load_exact(artifact, offset, length)
    }

    pub(in crate::physical_runtime::record_serving) fn load_bounded(
        &self,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        self.reader.load_bounded(artifact, limit)
    }
}
