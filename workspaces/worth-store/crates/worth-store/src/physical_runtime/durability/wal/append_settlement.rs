use worth_store_physical_backend::{
    ArtifactAppendRange, CompletedArtifactAppend, MediaOperationIdentity,
};

use crate::physical_runtime::PhysicalWorkIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalAppendSettlement {
    work: PhysicalWorkIdentity,
    range: ArtifactAppendRange,
    payload_digest: [u8; 32],
    backend_operation: MediaOperationIdentity,
}

pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalAppendSettlement(
    PhysicalWalAppendSettlement,
);

impl PhysicalWalAppendSettlement {
    pub(in crate::physical_runtime) fn completed(
        work: PhysicalWorkIdentity,
        physical: &CompletedArtifactAppend,
    ) -> Self {
        Self {
            work,
            range: physical.range(),
            payload_digest: physical.payload_digest(),
            backend_operation: physical.operation(),
        }
    }

    pub const fn work_identity(self) -> PhysicalWorkIdentity {
        self.work
    }

    pub const fn range(self) -> ArtifactAppendRange {
        self.range
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }

    pub fn matches_completion_binding(
        self,
        work: PhysicalWorkIdentity,
        range: ArtifactAppendRange,
        payload_digest: [u8; 32],
    ) -> bool {
        self.bind_completion(work, range, payload_digest).is_some()
    }

    pub(in crate::physical_runtime) fn bind_completion(
        self,
        work: PhysicalWorkIdentity,
        range: ArtifactAppendRange,
        payload_digest: [u8; 32],
    ) -> Option<CompletionBoundPhysicalWalAppendSettlement> {
        (self.work == work && self.range == range && self.payload_digest == payload_digest)
            .then_some(CompletionBoundPhysicalWalAppendSettlement(self))
    }

    pub const fn backend_operation(self) -> MediaOperationIdentity {
        self.backend_operation
    }
}

impl CompletionBoundPhysicalWalAppendSettlement {
    pub(in crate::physical_runtime) const fn into_settlement(self) -> PhysicalWalAppendSettlement {
        self.0
    }
}
