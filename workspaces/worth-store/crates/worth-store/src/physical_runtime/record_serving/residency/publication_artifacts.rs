use worth_store_physical_backend::{
    ArtifactRangeWriteOutcome, ArtifactTreeFailure, QualifiedFilesystemMedia,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::artifact_tree::PhysicalRecordArtifactTree;
use super::candidate_frame_residency::CandidateFramePhysicalWrite;

pub(in crate::physical_runtime::record_serving) struct PublicationRecordArtifacts<'media> {
    tree: PhysicalRecordArtifactTree<'media>,
}

impl<'media> PublicationRecordArtifacts<'media> {
    pub(in crate::physical_runtime::record_serving) fn new(
        media: &'media QualifiedFilesystemMedia,
    ) -> Self {
        Self {
            tree: PhysicalRecordArtifactTree::new(media),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn write_new_frame(
        &self,
        artifact: RecordArtifactFile,
        bytes: &[u8],
    ) -> Result<CandidateFramePhysicalWrite, ArtifactTreeFailure> {
        let length = u32::try_from(bytes.len()).expect("candidate frame length is u32-bounded");
        let coordinate = RecordFrameCoordinate::new(artifact, 0, length)
            .expect("candidate frames are nonempty and offset-bounded");
        let mut file = self.tree.create_new_file(artifact)?;
        classify_candidate_write(file.write_exact_chunk(coordinate, bytes))
    }

    pub(in crate::physical_runtime::record_serving) fn create_new_file(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<worth_store_physical_backend::ArtifactTreeNewFile<'_>, ArtifactTreeFailure> {
        self.tree.create_new_file(artifact)
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

pub(in crate::physical_runtime::record_serving) fn classify_candidate_write(
    outcome: ArtifactRangeWriteOutcome,
) -> Result<CandidateFramePhysicalWrite, ArtifactTreeFailure> {
    match outcome {
        ArtifactRangeWriteOutcome::Completed(receipt) => {
            Ok(CandidateFramePhysicalWrite::completed(receipt))
        }
        ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure) => Err(failure),
        ArtifactRangeWriteOutcome::Indeterminate(indeterminate) => Err(indeterminate.failure()),
    }
}
