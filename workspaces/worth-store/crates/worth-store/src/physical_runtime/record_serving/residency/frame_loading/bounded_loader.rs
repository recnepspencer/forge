use worth_store_buffer_pool::{PhysicalFrameKey, PhysicalFrameLoadError, PhysicalResidencyPool};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::read_source::{frame_source_failure, FrameReadSource};
use super::{
    FrameLoadFailure, FrameLoadFailureKind, FrameLoadPort, FrameReadWorkAdmission,
    LoadedPhysicalFrame, ObservedArtifactLength,
};
use crate::physical_runtime::record_serving::residency::frame_work_trace::FrameWorkTrace;

#[derive(Clone)]
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
        work_admission: FrameReadWorkAdmission,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let coordinate = RecordFrameCoordinate::new(artifact, offset, length).ok_or(
            FrameLoadFailure::new(FrameLoadFailureKind::InvalidCoordinate),
        )?;
        let key = PhysicalFrameKey::new(self.pool.store_identity(), coordinate);
        let mut work = FrameWorkTrace::none();
        let mut projection_failure = None;
        let mut prepared = match work_admission {
            FrameReadWorkAdmission::EveryAccess => {
                let prepared =
                    source
                        .prepare_exact(artifact, offset, length)
                        .map_err(|failure| {
                            frame_source_failure(failure).with_work(FrameWorkTrace::none())
                        })?;
                work = FrameWorkTrace::one(prepared.identity());
                Some(prepared)
            }
            FrameReadWorkAdmission::ResidencyFaultOnly => None,
        };
        let lease = self
            .pool
            .load(key, |target| {
                let prepared = match prepared.take() {
                    Some(prepared) => prepared,
                    None => source.prepare_exact(artifact, offset, length)?,
                };
                work = FrameWorkTrace::one(prepared.identity());
                projection_failure = prepared.execute(target)?;
                Ok(())
            })
            .map_err(|failure| match failure {
                PhysicalFrameLoadError::Residency(reason) => {
                    FrameLoadFailure::new(FrameLoadFailureKind::Residency(reason)).with_work(work)
                }
                PhysicalFrameLoadError::Source(reason) => {
                    frame_source_failure(reason).with_work(work)
                }
            })?;
        LoadedPhysicalFrame::bind(
            self.pool.store_identity(),
            coordinate,
            lease,
            work,
            projection_failure,
        )
    }

    fn load_bounded(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        limit: u32,
        work_admission: FrameReadWorkAdmission,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let length = source.file_length(artifact).map_err(frame_source_failure)?;
        if length.bytes() == 0
            || length.bytes() > u64::from(limit)
            || length.bytes() > u64::from(u32::MAX)
        {
            let work = length.reject_structural_damage();
            return Err(
                FrameLoadFailure::new(FrameLoadFailureKind::AccessLimitExceeded).with_work(work),
            );
        }
        let bytes = length.bytes();
        let work = length.work_trace();
        self.load_exact(source, artifact, 0, bytes as u32, work_admission)
            .map(|frame| frame.preceded_by(work))
    }

    fn file_length(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameLoadFailure> {
        source.file_length(artifact).map_err(frame_source_failure)
    }
}
