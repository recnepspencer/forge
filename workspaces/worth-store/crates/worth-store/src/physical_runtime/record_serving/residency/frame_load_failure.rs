use worth_store_buffer_pool::PhysicalResidencyDenial;
use worth_store_physical_backend::ArtifactTreeFailure;

use super::frame_work_trace::FrameWorkTrace;
use crate::physical_runtime::record_serving::CanonicalRecordReadFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum FrameLoadFailureKind {
    Backend(ArtifactTreeFailure),
    Work(CanonicalRecordReadFailure),
    Residency(PhysicalResidencyDenial),
    FaultTerminated {
        terminal: worth_store_buffer_pool::PhysicalFrameLoadTerminal,
        cause: FrameLoadFaultCause,
    },
    AccessLimitExceeded,
    ArtifactLengthMismatch,
    InvalidCoordinate,
    ReturnedFrameIdentityMismatch,
    CoalescedFault(worth_store_buffer_pool::PhysicalFrameLoadTerminal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum FrameLoadFaultCause {
    Backend(ArtifactTreeFailure),
    Work(CanonicalRecordReadFailure),
    Residency(PhysicalResidencyDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct FrameLoadFailure {
    kind: FrameLoadFailureKind,
    work: FrameWorkTrace,
}

impl FrameLoadFailure {
    pub(in crate::physical_runtime::record_serving) const fn new(
        kind: FrameLoadFailureKind,
    ) -> Self {
        Self {
            kind,
            work: FrameWorkTrace::none(),
        }
    }

    pub(super) const fn preceded_by(mut self, work: FrameWorkTrace) -> Self {
        self.work = work.then(self.work);
        self
    }

    pub(super) const fn with_complete_work_trace(mut self, work: FrameWorkTrace) -> Self {
        self.work = work;
        self
    }

    pub(in crate::physical_runtime::record_serving) const fn kind(self) -> FrameLoadFailureKind {
        self.kind
    }

    pub(in crate::physical_runtime::record_serving) const fn work_trace(self) -> FrameWorkTrace {
        self.work
    }
}
