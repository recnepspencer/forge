use worth_store_physical_format::RecordFrameCoordinate;

#[cfg(feature = "certification-test-authority")]
use super::super::frame_work_trace::FrameWorkTrace;

#[derive(Debug)]
#[must_use = "dirty authority must advance through writeback or be deliberately discarded"]
pub struct AdmittedDirtyFrame {
    coordinate: RecordFrameCoordinate,
    frame: worth_store_buffer_pool::DirtyPhysicalFrame,
    #[cfg(feature = "certification-test-authority")]
    source: FrameWorkTrace,
}

#[cfg(feature = "certification-test-authority")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDirtyTransitionFailure {
    StaleOrForeignFrame,
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
}

impl AdmittedDirtyFrame {
    pub(in crate::physical_runtime::record_serving) const fn candidate(
        coordinate: RecordFrameCoordinate,
        frame: worth_store_buffer_pool::DirtyPhysicalFrame,
    ) -> Self {
        Self {
            coordinate,
            frame,
            #[cfg(feature = "certification-test-authority")]
            source: FrameWorkTrace::none(),
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) const fn from_loaded_frame(
        coordinate: RecordFrameCoordinate,
        frame: worth_store_buffer_pool::DirtyPhysicalFrame,
        source: FrameWorkTrace,
    ) -> Self {
        Self {
            coordinate,
            frame,
            source,
        }
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn source_physical_work_count(&self) -> u64 {
        self.source.count()
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn first_source_physical_work(
        &self,
    ) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.source.first()
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn last_source_physical_work(
        &self,
    ) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.source.last()
    }

    pub fn discard(self) -> Result<(), worth_store_buffer_pool::PhysicalResidencyDenial> {
        self.frame.discard_candidate()
    }

    pub(super) fn into_frame(self) -> worth_store_buffer_pool::DirtyPhysicalFrame {
        self.frame
    }
}
