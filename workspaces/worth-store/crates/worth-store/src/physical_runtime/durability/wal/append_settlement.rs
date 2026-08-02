use worth_store_physical_backend::{
    ArtifactAppendRange, ArtifactTreeFile, CompletedArtifactAppend, CompletedArtifactNewWrite,
    MediaOperationIdentity,
};

use crate::physical_runtime::{
    PhysicalWalFrameCompletionBinding, PhysicalWalFrameWriteDisposition, PhysicalWorkIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWalAppendSettlement {
    work: PhysicalWorkIdentity,
    artifact: ArtifactTreeFile,
    range: ArtifactAppendRange,
    payload_digest: [u8; 32],
    disposition: PhysicalWalFrameWriteDisposition,
    backend_operation: MediaOperationIdentity,
    create_operation: Option<MediaOperationIdentity>,
}

pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalAppendSettlement(
    PhysicalWalAppendSettlement,
);

impl PhysicalWalAppendSettlement {
    pub(in crate::physical_runtime) fn completed_append(
        work: PhysicalWorkIdentity,
        physical: &CompletedArtifactAppend,
    ) -> Self {
        Self {
            work,
            artifact: physical.artifact().clone(),
            range: physical.range(),
            payload_digest: physical.payload_digest(),
            disposition: PhysicalWalFrameWriteDisposition::AppendExistingSegment,
            backend_operation: physical.operation(),
            create_operation: None,
        }
    }

    pub(in crate::physical_runtime) fn completed_segment_create(
        work: PhysicalWorkIdentity,
        physical: &CompletedArtifactNewWrite,
    ) -> Self {
        Self {
            work,
            artifact: physical.artifact().clone(),
            range: ArtifactAppendRange::new(0, physical.range().byte_count())
                .expect("completed WAL segment creation writes one nonempty prefix"),
            payload_digest: physical.payload_digest(),
            disposition: PhysicalWalFrameWriteDisposition::CreateSegment,
            backend_operation: physical.write_operation(),
            create_operation: Some(physical.create_operation()),
        }
    }

    pub const fn work_identity(&self) -> PhysicalWorkIdentity {
        self.work
    }

    pub const fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
    }

    pub const fn range(&self) -> ArtifactAppendRange {
        self.range
    }

    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn disposition(&self) -> PhysicalWalFrameWriteDisposition {
        self.disposition
    }

    pub fn matches_completion_binding(
        &self,
        work: PhysicalWorkIdentity,
        artifact: &ArtifactTreeFile,
        range: ArtifactAppendRange,
        payload_digest: [u8; 32],
        disposition: PhysicalWalFrameWriteDisposition,
    ) -> bool {
        self.work == work
            && &self.artifact == artifact
            && self.range == range
            && self.payload_digest == payload_digest
            && self.disposition == disposition
    }

    pub(in crate::physical_runtime) fn bind_completion(
        self,
        work: PhysicalWorkIdentity,
        artifact: &ArtifactTreeFile,
        binding: PhysicalWalFrameCompletionBinding,
    ) -> Option<CompletionBoundPhysicalWalAppendSettlement> {
        self.matches(work, artifact, binding)
            .then_some(CompletionBoundPhysicalWalAppendSettlement(self))
    }

    pub const fn backend_operation(&self) -> MediaOperationIdentity {
        self.backend_operation
    }

    pub const fn create_operation(&self) -> Option<MediaOperationIdentity> {
        self.create_operation
    }

    fn matches(
        &self,
        work: PhysicalWorkIdentity,
        artifact: &ArtifactTreeFile,
        binding: PhysicalWalFrameCompletionBinding,
    ) -> bool {
        if self.work != work || &self.artifact != artifact {
            return false;
        }
        match binding {
            PhysicalWalFrameCompletionBinding::Create {
                range,
                payload_digest,
            } => {
                self.disposition == PhysicalWalFrameWriteDisposition::CreateSegment
                    && self.range.offset() == 0
                    && self.range.byte_count() == range.byte_count()
                    && self.payload_digest == payload_digest
                    && self.create_operation.is_some()
            }
            PhysicalWalFrameCompletionBinding::Append {
                range,
                payload_digest,
            } => {
                self.disposition == PhysicalWalFrameWriteDisposition::AppendExistingSegment
                    && self.range == range
                    && self.payload_digest == payload_digest
                    && self.create_operation.is_none()
            }
        }
    }
}

impl CompletionBoundPhysicalWalAppendSettlement {
    pub(in crate::physical_runtime) fn into_settlement(self) -> PhysicalWalAppendSettlement {
        self.0
    }
}
