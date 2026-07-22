use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, ArtifactTreeDirectory, ArtifactTreeFailure,
    ArtifactTreeFile, ArtifactTreeMedia, BackendQueueExecutionAdaptation,
    BackendQueueExecutionPlanBinding, BackendQueueSpeculativeScope, QualifiedFilesystemMedia,
    ScheduledArtifactRangeWriteOutcome,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum RecordFamilyInventory {
    ProvenAbsent,
    Published,
    Residue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum RecordFamilyCreationFailure {
    BeforeEffect(ArtifactTreeFailure),
    AfterEffect(ArtifactTreeFailure),
}

pub(in crate::physical_runtime::record_serving) struct PhysicalRecordArtifactTree<'media> {
    tree: ArtifactTreeMedia<'media>,
    families: ArtifactTreeDirectory,
    staging: ArtifactTreeDirectory,
    record_family: ArtifactTreeDirectory,
    root_manifests: ArtifactTreeDirectory,
    page_segments: ArtifactTreeDirectory,
    segment_manifests: ArtifactTreeDirectory,
    extents: ArtifactTreeDirectory,
    extent_manifests: ArtifactTreeDirectory,
    free_space_manifests: ArtifactTreeDirectory,
    record_staging: ArtifactTreeDirectory,
}

impl<'media> PhysicalRecordArtifactTree<'media> {
    pub(in crate::physical_runtime::record_serving) fn new(
        media: &'media QualifiedFilesystemMedia,
    ) -> Self {
        let families = ArtifactTreeDirectory::families();
        let staging = ArtifactTreeDirectory::staging();
        let record_family = families.child("records").expect("portable Store layout");
        let root_manifests = record_family.child("roots").expect("portable Store layout");
        let page_segments = record_family
            .child("segments")
            .expect("portable Store layout");
        let segment_manifests = record_family
            .child("segment-manifests")
            .expect("portable Store layout");
        let extents = record_family
            .child("extents")
            .expect("portable Store layout");
        let extent_manifests = record_family
            .child("extent-manifests")
            .expect("portable Store layout");
        let free_space_manifests = record_family
            .child("free-space")
            .expect("portable Store layout");
        let record_staging = staging.child("records").expect("portable Store layout");
        Self {
            tree: media.artifact_tree(),
            families,
            staging,
            record_family,
            root_manifests,
            page_segments,
            segment_manifests,
            extents,
            extent_manifests,
            free_space_manifests,
            record_staging,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn inventory(
        &self,
    ) -> Result<RecordFamilyInventory, ArtifactTreeFailure> {
        let family = self.tree.directory_exists(&self.record_family)?;
        let staging = self.tree.directory_exists(&self.record_staging)?;
        if !family && !staging {
            return Ok(RecordFamilyInventory::ProvenAbsent);
        }
        let roots = self.tree.directory_exists(&self.root_manifests)?;
        let segments = self.tree.directory_exists(&self.page_segments)?;
        let segment_manifests = self.tree.directory_exists(&self.segment_manifests)?;
        let extents = self.tree.directory_exists(&self.extents)?;
        let extent_manifests = self.tree.directory_exists(&self.extent_manifests)?;
        let free_space = self.tree.directory_exists(&self.free_space_manifests)?;
        let catalog = self
            .tree
            .file_exists(&self.artifact(RecordArtifactFile::BootstrapCatalog))?;
        if family
            && staging
            && roots
            && segments
            && segment_manifests
            && extents
            && extent_manifests
            && free_space
            && catalog
        {
            Ok(RecordFamilyInventory::Published)
        } else {
            Ok(RecordFamilyInventory::Residue)
        }
    }

    pub(in crate::physical_runtime::record_serving) fn create_record_family(
        &self,
    ) -> Result<(), RecordFamilyCreationFailure> {
        self.tree
            .create_directory(&self.record_family)
            .map_err(RecordFamilyCreationFailure::BeforeEffect)?;
        self.synchronize_created_directory_parent(&self.families)?;
        self.create_and_publish_directory(&self.record_staging, &self.staging)?;
        self.tree
            .create_directory(&self.root_manifests)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.tree
            .create_directory(&self.page_segments)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.tree
            .create_directory(&self.segment_manifests)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.tree
            .create_directory(&self.extents)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.tree
            .create_directory(&self.extent_manifests)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.tree
            .create_directory(&self.free_space_manifests)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.synchronize_created_directory_parent(&self.record_family)?;
        Ok(())
    }

    pub(in crate::physical_runtime::record_serving) fn write_new(
        &self,
        artifact: RecordArtifactFile,
        bytes: &[u8],
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.write_new(&self.artifact(artifact), bytes)
    }

    pub(in crate::physical_runtime::record_serving) fn create_new_file(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<worth_store_physical_backend::ArtifactTreeNewFile<'_>, ArtifactTreeFailure> {
        self.tree.create_new_file(&self.artifact(artifact))
    }

    pub(in crate::physical_runtime::record_serving) fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<u64, ArtifactTreeFailure> {
        self.tree.file_length(&self.artifact(artifact))
    }

    pub(in crate::physical_runtime::record_serving) fn file_exists(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<bool, ArtifactTreeFailure> {
        self.tree.file_exists(&self.artifact(artifact))
    }

    pub(in crate::physical_runtime::record_serving) fn read_exact_at(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        target: &mut [u8],
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree
            .read_exact_at(&self.artifact(artifact), offset, target)
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::physical_runtime::record_serving) fn write_scheduled_exact_at(
        &self,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        scope: BackendQueueSpeculativeScope,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> ScheduledArtifactRangeWriteOutcome {
        self.tree.write_scheduled_exact_at(
            &self.artifact(coordinate.artifact()),
            coordinate,
            bytes,
            binding,
            adaptation,
            scope,
            durability,
        )
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.synchronize_file(&self.artifact(artifact))
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_artifact_parent(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        let directory = self.artifact_directory(artifact);
        self.tree.synchronize_directory(directory)
    }

    pub(in crate::physical_runtime::record_serving) fn replace_catalog(
        &self,
        candidate: RecordArtifactFile,
    ) -> Result<(), ArtifactTreeFailure> {
        let RecordArtifactFile::CatalogCandidate { .. } = candidate else {
            unreachable!("only catalog candidates can become the bootstrap catalog");
        };
        self.tree.replace(
            &self.artifact(candidate),
            &self.artifact(RecordArtifactFile::BootstrapCatalog),
        )
    }

    pub(in crate::physical_runtime::record_serving) fn synchronize_record_family(
        &self,
    ) -> Result<(), ArtifactTreeFailure> {
        self.tree.synchronize_directory(&self.record_family)
    }

    pub(in crate::physical_runtime::record_serving) fn has_staging_residue(
        &self,
    ) -> Result<bool, ArtifactTreeFailure> {
        self.tree.directory_has_entries(&self.record_staging)
    }

    fn artifact(&self, artifact: RecordArtifactFile) -> ArtifactTreeFile {
        self.artifact_directory(artifact)
            .file(&artifact.file_name())
            .expect("encoded Store artifact names are portable")
    }

    fn artifact_directory(&self, artifact: RecordArtifactFile) -> &ArtifactTreeDirectory {
        match artifact {
            RecordArtifactFile::BootstrapCatalog => &self.record_family,
            RecordArtifactFile::CatalogCandidate { .. } => &self.record_staging,
            RecordArtifactFile::RootManifest { .. }
            | RecordArtifactFile::RootRoutingBlock { .. } => &self.root_manifests,
            RecordArtifactFile::Segment { .. } => &self.page_segments,
            RecordArtifactFile::SegmentManifest { .. }
            | RecordArtifactFile::SegmentMembershipBlock { .. } => &self.segment_manifests,
            RecordArtifactFile::Extent { .. } => &self.extents,
            RecordArtifactFile::ExtentManifest { .. } => &self.extent_manifests,
            RecordArtifactFile::FreeSpaceManifest { .. }
            | RecordArtifactFile::FreeSpaceMembershipBlock { .. } => &self.free_space_manifests,
        }
    }

    fn create_and_publish_directory(
        &self,
        directory: &ArtifactTreeDirectory,
        parent: &ArtifactTreeDirectory,
    ) -> Result<(), RecordFamilyCreationFailure> {
        self.tree
            .create_directory(directory)
            .map_err(RecordFamilyCreationFailure::AfterEffect)?;
        self.synchronize_created_directory_parent(parent)
    }

    fn synchronize_created_directory_parent(
        &self,
        parent: &ArtifactTreeDirectory,
    ) -> Result<(), RecordFamilyCreationFailure> {
        self.tree
            .synchronize_directory(parent)
            .map_err(RecordFamilyCreationFailure::AfterEffect)
    }
}
