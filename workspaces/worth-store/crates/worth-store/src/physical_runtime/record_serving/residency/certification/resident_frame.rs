use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::{LifecycleGeneration, PhysicalWorkIdentity, RuntimeIdentity};

use super::{super::frame_loading::LoadedPhysicalFrame, probe::CertificationResidencyBinding};

/// A pinned frame exposed only to certification code.
///
/// Dropping this value releases the pool-owned pin lease.
pub struct CertificationResidentFrame {
    binding: CertificationResidencyBinding,
    coordinate: RecordFrameCoordinate,
    frame: LoadedPhysicalFrame,
}

impl CertificationResidentFrame {
    pub(super) const fn bind(
        binding: CertificationResidencyBinding,
        coordinate: RecordFrameCoordinate,
        frame: LoadedPhysicalFrame,
    ) -> Self {
        Self {
            binding,
            coordinate,
            frame,
        }
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub fn bytes(&self) -> &[u8] {
        &self.frame
    }

    pub fn len(&self) -> usize {
        self.frame.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frame.is_empty()
    }

    pub fn copy_range_into(&self, range: std::ops::Range<usize>, target: &mut [u8]) {
        self.frame.copy_range_into(range, target);
    }

    pub const fn physical_work_count(&self) -> u64 {
        self.frame.work_trace().count()
    }

    pub const fn first_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.frame.work_trace().first()
    }

    pub const fn last_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.frame.work_trace().last()
    }

    pub fn reject_projection_failure(self) {
        self.frame.reject_projection_failure();
    }

    pub(in crate::physical_runtime::record_serving) fn belongs_to(
        &self,
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
    ) -> bool {
        self.binding.matches(store, runtime, generation)
    }

    pub(in crate::physical_runtime::record_serving) fn into_dirty_candidate<F>(
        self,
        allocation: &worth_store_buffer_pool::ForegroundWriteAllocationGrant,
        fill: F,
    ) -> Result<
        (
            worth_store_buffer_pool::DirtyPhysicalFrame,
            super::super::frame_work_trace::FrameWorkTrace,
        ),
        worth_store_buffer_pool::PhysicalResidencyDenial,
    >
    where
        F: FnOnce(&[u8], &mut [u8]),
    {
        self.frame.fill_dirty_candidate(allocation, fill)
    }
}
