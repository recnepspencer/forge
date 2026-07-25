use worth_store_buffer_pool::{
    DirtyPhysicalFrame, PhysicalFrameKey, PhysicalFrameLease, PhysicalResidencyDenial,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::{FrameLoadFailure, FrameLoadFailureKind};
use crate::physical_runtime::record_serving::residency::frame_work_trace::FrameWorkTrace;

pub(in crate::physical_runtime::record_serving) struct LoadedPhysicalFrame {
    lease: PhysicalFrameLease,
    work: FrameWorkTrace,
    projection_failure:
        Option<crate::physical_runtime::instance::PhysicalProjectionFailureCapability>,
}

impl LoadedPhysicalFrame {
    pub(super) fn bind(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        lease: PhysicalFrameLease,
        work: FrameWorkTrace,
        projection_failure: Option<
            crate::physical_runtime::instance::PhysicalProjectionFailureCapability,
        >,
    ) -> Result<Self, FrameLoadFailure> {
        if lease.key() != PhysicalFrameKey::new(store, coordinate) {
            if let Some(projection_failure) = projection_failure {
                projection_failure.consume();
            }
            return Err(
                FrameLoadFailure::new(FrameLoadFailureKind::ReturnedFrameIdentityMismatch)
                    .with_work(work),
            );
        }
        Ok(Self {
            lease,
            work,
            projection_failure,
        })
    }

    pub(in crate::physical_runtime::record_serving) const fn work_trace(&self) -> FrameWorkTrace {
        self.work
    }

    pub(super) fn preceded_by(mut self, work: FrameWorkTrace) -> Self {
        self.work = work.then(self.work);
        self
    }

    pub(in crate::physical_runtime::record_serving) fn reject_projection_failure(mut self) {
        if let Some(projection_failure) = self.projection_failure.take() {
            projection_failure.consume();
        }
    }

    pub(in crate::physical_runtime::record_serving) fn copy_range_into(
        &self,
        range: std::ops::Range<usize>,
        target: &mut [u8],
    ) {
        self.lease.copy_range_into(range, target);
    }

    pub(in crate::physical_runtime::record_serving) fn replace_with_dirty_candidate(
        self,
        bytes: Vec<u8>,
    ) -> Result<(DirtyPhysicalFrame, FrameWorkTrace), PhysicalResidencyDenial> {
        let work = self.work;
        self.lease
            .replace_with_dirty_candidate(bytes)
            .map(|dirty| (dirty, work))
    }
}

impl std::ops::Deref for LoadedPhysicalFrame {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.lease
    }
}
