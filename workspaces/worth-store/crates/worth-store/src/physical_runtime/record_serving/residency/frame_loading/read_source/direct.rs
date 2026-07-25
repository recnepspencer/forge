use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::RecordArtifactFile;

use super::{FrameReadSource, FrameReadSourceFailure, ObservedArtifactLength, PreparedFrameRead};
use crate::physical_runtime::record_serving::residency::frame_work_trace::FrameWorkTrace;
use crate::physical_runtime::PhysicalWorkIdentity;

pub(in crate::physical_runtime::record_serving) struct DirectFrameReadSource<'media> {
    media: &'media QualifiedFilesystemMedia,
}

impl<'media> DirectFrameReadSource<'media> {
    pub(in crate::physical_runtime::record_serving) const fn new(
        media: &'media QualifiedFilesystemMedia,
    ) -> Self {
        Self { media }
    }
}

impl FrameReadSource for DirectFrameReadSource<'_> {
    fn prepare_exact(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        _length: u32,
    ) -> Result<Box<dyn PreparedFrameRead + '_>, FrameReadSourceFailure> {
        Ok(Box::new(DirectPreparedFrameRead {
            media: self.media,
            artifact,
            offset,
        }))
    }

    fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameReadSourceFailure> {
        crate::physical_runtime::record_serving::residency::artifact_tree::
            PhysicalRecordArtifactTree::new(self.media)
            .file_length(artifact)
            .map(|bytes| ObservedArtifactLength::new(bytes, FrameWorkTrace::default()))
            .map_err(FrameReadSourceFailure::Backend)
    }
}

struct DirectPreparedFrameRead<'media> {
    media: &'media QualifiedFilesystemMedia,
    artifact: RecordArtifactFile,
    offset: u64,
}

impl PreparedFrameRead for DirectPreparedFrameRead<'_> {
    fn identity(&self) -> Option<PhysicalWorkIdentity> {
        None
    }

    fn execute(self: Box<Self>, target: &mut [u8]) -> Result<(), FrameReadSourceFailure> {
        crate::physical_runtime::record_serving::residency::artifact_tree::
            PhysicalRecordArtifactTree::new(self.media)
            .read_exact_at(self.artifact, self.offset, target)
            .map_err(FrameReadSourceFailure::Backend)
    }
}
