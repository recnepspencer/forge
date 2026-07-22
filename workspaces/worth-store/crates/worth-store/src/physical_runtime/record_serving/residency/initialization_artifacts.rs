use worth_store_physical_backend::{ArtifactTreeFailure, QualifiedFilesystemMedia};
use worth_store_physical_format::RecordArtifactFile;

use super::artifact_tree::{
    PhysicalRecordArtifactTree, RecordFamilyCreationFailure, RecordFamilyInventory,
};

pub(in crate::physical_runtime::record_serving) struct InitializationRecordArtifacts<'media> {
    tree: PhysicalRecordArtifactTree<'media>,
}

impl<'media> InitializationRecordArtifacts<'media> {
    pub(in crate::physical_runtime::record_serving) fn new(
        media: &'media QualifiedFilesystemMedia,
    ) -> Self {
        Self {
            tree: PhysicalRecordArtifactTree::new(media),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn inventory(
        &self,
    ) -> Result<RecordFamilyInventory, ArtifactTreeFailure> {
        self.tree.inventory()
    }

    pub(in crate::physical_runtime::record_serving) fn create_record_family(
        &self,
    ) -> Result<(), RecordFamilyCreationFailure> {
        self.tree.create_record_family()
    }

    pub(in crate::physical_runtime::record_serving) fn write_new(
        &self,
        artifact: RecordArtifactFile,
        bytes: &[u8],
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.write_new(artifact, bytes)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.synchronize_artifact(artifact)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact_parent(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.synchronize_artifact_parent(artifact)
    }

    pub(in crate::physical_runtime::record_serving) fn replace_catalog(
        &self,
        candidate: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.replace_catalog(candidate)
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_record_family(
        &self,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.synchronize_record_family()
    }
}
