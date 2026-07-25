use worth_store_physical_backend::ArtifactTreeFailure;
use worth_store_physical_format::RecordArtifactFile;

use super::{FrameLoadFailure, FrameLoadFailureKind, LoadedPhysicalFrame};
use crate::physical_runtime::{
    record_serving::{residency::frame_work_trace::FrameWorkTrace, CanonicalRecordReadFailure},
    PhysicalWorkIdentity,
};

mod canonical;
mod direct;

pub(in crate::physical_runtime::record_serving) use canonical::CanonicalFrameReadSource;
pub(in crate::physical_runtime::record_serving) use direct::DirectFrameReadSource;

pub(in crate::physical_runtime::record_serving) trait FrameReadSource {
    fn prepare_exact(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<Box<dyn PreparedFrameRead + '_>, FrameReadSourceFailure>;

    fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameReadSourceFailure>;
}

#[derive(Debug)]
pub(in crate::physical_runtime::record_serving) struct ObservedArtifactLength {
    bytes: u64,
    work: FrameWorkTrace,
    projection_failure:
        Option<crate::physical_runtime::instance::PhysicalProjectionFailureCapability>,
}

impl ObservedArtifactLength {
    const fn new(bytes: u64, work: FrameWorkTrace) -> Self {
        Self {
            bytes,
            work,
            projection_failure: None,
        }
    }

    const fn admitted(
        bytes: u64,
        work: FrameWorkTrace,
        projection_failure: crate::physical_runtime::instance::PhysicalProjectionFailureCapability,
    ) -> Self {
        Self {
            bytes,
            work,
            projection_failure: Some(projection_failure),
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub(in crate::physical_runtime::record_serving) const fn work_trace(&self) -> FrameWorkTrace {
        self.work
    }

    pub(in crate::physical_runtime::record_serving) fn reject_structural_damage(
        mut self,
    ) -> FrameWorkTrace {
        if let Some(projection_failure) = self.projection_failure.take() {
            projection_failure.consume();
        }
        self.work
    }
}

pub(in crate::physical_runtime::record_serving) trait PreparedFrameRead {
    fn identity(&self) -> Option<PhysicalWorkIdentity>;

    fn execute(
        self: Box<Self>,
        target: &mut [u8],
    ) -> Result<
        Option<crate::physical_runtime::instance::PhysicalProjectionFailureCapability>,
        FrameReadSourceFailure,
    >;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum FrameReadSourceFailure {
    Backend(ArtifactTreeFailure),
    Work(Box<FrameReadWorkFailure>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct FrameReadWorkFailure {
    failure: CanonicalRecordReadFailure,
    work: FrameWorkTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum FrameReadWorkAdmission {
    EveryAccess,
    ResidencyFaultOnly,
}

impl FrameReadSourceFailure {
    fn work(failure: CanonicalRecordReadFailure, work: FrameWorkTrace) -> Self {
        Self::Work(Box::new(FrameReadWorkFailure { failure, work }))
    }
}

pub(in crate::physical_runtime::record_serving) trait FrameLoadPort {
    fn load_exact(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
        work_admission: FrameReadWorkAdmission,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure>;

    fn load_bounded(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        limit: u32,
        work_admission: FrameReadWorkAdmission,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure>;

    fn file_length(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameLoadFailure>;
}

pub(super) fn frame_source_failure(failure: FrameReadSourceFailure) -> FrameLoadFailure {
    match failure {
        FrameReadSourceFailure::Backend(failure) => {
            FrameLoadFailure::new(FrameLoadFailureKind::Backend(failure))
        }
        FrameReadSourceFailure::Work(failure) => {
            FrameLoadFailure::new(FrameLoadFailureKind::Work(failure.failure))
                .with_work(failure.work)
        }
    }
}
