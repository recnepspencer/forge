use std::num::NonZeroU64;

use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    candidate_frame_residency::{CandidateFrameSet, StoreCandidateFramePublicationSession},
    frame_loading::{
        CanonicalFrameReadSource, FrameLoadFailure, LoadedPhysicalFrame, ObservedArtifactLength,
    },
    frame_ports::RecordFramePorts,
};

/// The single Store-private capability for ordinary serving-frame residency.
///
/// It keeps the pool port and canonical physical-work source inseparable, so
/// an ordinary reader cannot retain the pool while substituting a direct media
/// source or retain the source while bypassing residency admission.
#[derive(Clone)]
pub(in crate::physical_runtime::record_serving) struct ServingFrameResidency {
    frame_ports: RecordFramePorts,
    source: CanonicalFrameReadSource,
}

impl ServingFrameResidency {
    pub(in crate::physical_runtime::record_serving) const fn new(
        frame_ports: RecordFramePorts,
        source: CanonicalFrameReadSource,
    ) -> Self {
        Self {
            frame_ports,
            source,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn for_scan(mut self) -> Self {
        self.source = self.source.for_scan();
        self
    }

    pub(in crate::physical_runtime::record_serving) fn begin_operation(
        &self,
        scope: worth_store_buffer_pool::PhysicalOperationAllocationScope,
        bytes: NonZeroU64,
    ) -> Result<
        worth_store_buffer_pool::OperationAllocationGrant,
        worth_store_buffer_pool::PhysicalResidencyDenial,
    > {
        self.frame_ports.begin_operation(scope, bytes)
    }

    pub(in crate::physical_runtime::record_serving) fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameLoadFailure> {
        self.frame_ports
            .loader()
            .file_length(&self.source, artifact)
    }

    pub(in crate::physical_runtime::record_serving) fn load_exact(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        coordinate: RecordFrameCoordinate,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        self.frame_ports.loader().load_exact(
            allocation,
            &self.source,
            coordinate.artifact(),
            coordinate.offset(),
            coordinate.length(),
        )
    }

    pub(in crate::physical_runtime::record_serving) fn load_bounded(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        self.frame_ports
            .loader()
            .load_bounded(allocation, &self.source, artifact, limit)
    }

    pub(in crate::physical_runtime::record_serving) fn begin_candidate_publication<'allocation>(
        &self,
        allocation: &'allocation worth_store_buffer_pool::OperationAllocationGrant,
        declaration: CandidateFrameSet,
    ) -> Result<StoreCandidateFramePublicationSession<'allocation>, super::super::RecordAppendDenial>
    {
        StoreCandidateFramePublicationSession::begin(
            self.frame_ports.publisher(),
            allocation,
            declaration,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn counters(
        &self,
    ) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.frame_ports.counters()
    }
}
