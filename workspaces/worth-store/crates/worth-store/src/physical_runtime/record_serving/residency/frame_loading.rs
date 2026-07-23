use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalFrameLease, PhysicalFrameLoadError, PhysicalResidencyDenial,
    PhysicalResidencyPool,
};
use worth_store_physical_backend::{ArtifactTreeFailure, QualifiedFilesystemMedia};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

pub(in crate::physical_runtime::record_serving) struct LoadedPhysicalFrame {
    lease: PhysicalFrameLease,
}

impl LoadedPhysicalFrame {
    fn bind(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        lease: PhysicalFrameLease,
    ) -> Result<Self, FrameLoadFailure> {
        if lease.key() != PhysicalFrameKey::new(store, coordinate) {
            return Err(FrameLoadFailure::ReturnedFrameIdentityMismatch);
        }
        Ok(Self { lease })
    }

    pub(in crate::physical_runtime::record_serving) fn copy_range_into(
        &self,
        range: std::ops::Range<usize>,
        target: &mut [u8],
    ) {
        self.lease.copy_range_into(range, target);
    }
}

impl std::ops::Deref for LoadedPhysicalFrame {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.lease
    }
}

pub(in crate::physical_runtime::record_serving) trait FrameReadSource {
    fn read_exact_at(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        target: &mut [u8],
    ) -> Result<(), ArtifactTreeFailure>;

    fn file_length(&self, artifact: RecordArtifactFile) -> Result<u64, ArtifactTreeFailure>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum FrameLoadFailure {
    Backend(ArtifactTreeFailure),
    Residency(PhysicalResidencyDenial),
    AccessLimitExceeded,
    InvalidCoordinate,
    ReturnedFrameIdentityMismatch,
}

pub(in crate::physical_runtime::record_serving) trait FrameLoadPort {
    fn load_exact(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure>;

    fn load_bounded(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure>;

    fn file_length(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
    ) -> Result<u64, FrameLoadFailure>;
}

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
    fn read_exact_at(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        target: &mut [u8],
    ) -> Result<(), ArtifactTreeFailure> {
        super::artifact_tree::PhysicalRecordArtifactTree::new(self.media)
            .read_exact_at(artifact, offset, target)
    }

    fn file_length(&self, artifact: RecordArtifactFile) -> Result<u64, ArtifactTreeFailure> {
        super::artifact_tree::PhysicalRecordArtifactTree::new(self.media).file_length(artifact)
    }
}

pub(in crate::physical_runtime::record_serving) struct BoundedFrameLoader {
    pool: PhysicalResidencyPool,
}

impl BoundedFrameLoader {
    pub(in crate::physical_runtime::record_serving) const fn new(
        pool: PhysicalResidencyPool,
    ) -> Self {
        Self { pool }
    }
}

impl FrameLoadPort for BoundedFrameLoader {
    fn load_exact(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let coordinate = RecordFrameCoordinate::new(artifact, offset, length)
            .ok_or(FrameLoadFailure::InvalidCoordinate)?;
        let key = PhysicalFrameKey::new(self.pool.store_identity(), coordinate);
        let lease = self
            .pool
            .load(key, |target| source.read_exact_at(artifact, offset, target))
            .map_err(|failure| match failure {
                PhysicalFrameLoadError::Residency(reason) => FrameLoadFailure::Residency(reason),
                PhysicalFrameLoadError::Source(reason) => FrameLoadFailure::Backend(reason),
            })?;
        LoadedPhysicalFrame::bind(self.pool.store_identity(), coordinate, lease)
    }

    fn load_bounded(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let length = source
            .file_length(artifact)
            .map_err(FrameLoadFailure::Backend)?;
        if length == 0 || length > u64::from(limit) || length > u64::from(u32::MAX) {
            return Err(FrameLoadFailure::AccessLimitExceeded);
        }
        self.load_exact(source, artifact, 0, length as u32)
    }

    fn file_length(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
    ) -> Result<u64, FrameLoadFailure> {
        source
            .file_length(artifact)
            .map_err(FrameLoadFailure::Backend)
    }
}
