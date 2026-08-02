use sha2::{Digest, Sha256};

use super::super::super::{
    PhysicalWalFrameWriteDisposition, PhysicalWorkOperationFamily, ResourceAdmittedPhysicalWork,
};
use super::types::{
    require_family, PhysicalExecutorCommand, PhysicalExecutorCommandDenial,
    PhysicalWalAppendExecutorCommand, PhysicalWalBarrierExecutorCommand,
    PhysicalWalSegmentCreateExecutorCommand,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalWalFrameCompletionBinding {
    Create {
        range: worth_store_physical_backend::ArtifactNewWriteRange,
        payload_digest: [u8; 32],
    },
    Append {
        range: worth_store_physical_backend::ArtifactAppendRange,
        payload_digest: [u8; 32],
    },
}

impl PhysicalExecutorCommand {
    pub(in crate::physical_runtime) fn wal_frame_write(
        work: ResourceAdmittedPhysicalWork,
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        payload: impl Into<Box<[u8]>>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::WalAppend)?;
        let scope = work
            .intent()
            .scope()
            .wal_append_target()
            .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
        let payload = payload.into();
        if payload.len() as u64 != scope.byte_count() {
            return Err(PhysicalExecutorCommandDenial::PayloadLengthMismatch);
        }
        let payload_digest = Sha256::digest(&payload).into();
        match scope.disposition() {
            PhysicalWalFrameWriteDisposition::CreateSegment => {
                let range =
                    worth_store_physical_backend::ArtifactNewWriteRange::new(scope.byte_count())
                        .ok_or(
                            PhysicalExecutorCommandDenial::WalSegmentCreateCommandRequiresWalScope,
                        )?;
                Ok(Self::WalSegmentCreate(
                    PhysicalWalSegmentCreateExecutorCommand {
                        work,
                        artifact,
                        range,
                        payload,
                        payload_digest,
                    },
                ))
            }
            PhysicalWalFrameWriteDisposition::AppendExistingSegment => {
                let range = worth_store_physical_backend::ArtifactAppendRange::new(
                    scope.offset(),
                    scope.byte_count(),
                )
                .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
                Ok(Self::WalAppend(PhysicalWalAppendExecutorCommand {
                    work,
                    artifact,
                    range,
                    payload,
                    payload_digest,
                }))
            }
        }
    }

    pub(in crate::physical_runtime) const fn wal_frame_completion_binding(
        &self,
    ) -> Option<PhysicalWalFrameCompletionBinding> {
        match self {
            Self::WalSegmentCreate(command) => Some(PhysicalWalFrameCompletionBinding::Create {
                range: command.range,
                payload_digest: command.payload_digest,
            }),
            Self::WalAppend(command) => Some(PhysicalWalFrameCompletionBinding::Append {
                range: command.range,
                payload_digest: command.payload_digest,
            }),
            _ => None,
        }
    }

    pub(super) fn retry_wal_segment_create(
        work: ResourceAdmittedPhysicalWork,
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        range: worth_store_physical_backend::ArtifactNewWriteRange,
        payload: Box<[u8]>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_retry_range(&work, 0, range.byte_count())?;
        Self::wal_frame_write(work, artifact, payload)
    }

    pub(super) fn retry_wal_append(
        work: ResourceAdmittedPhysicalWork,
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        range: worth_store_physical_backend::ArtifactAppendRange,
        payload: Box<[u8]>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_retry_range(&work, range.offset(), range.byte_count())?;
        Self::wal_frame_write(work, artifact, payload)
    }

    pub(in crate::physical_runtime) fn wal_barrier(
        work: ResourceAdmittedPhysicalWork,
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        binding_digest: [u8; 32],
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::DurabilityBarrier)?;
        work.intent()
            .scope()
            .wal_barrier_target()
            .ok_or(PhysicalExecutorCommandDenial::WalBarrierCommandRequiresWalScope)?;
        Ok(Self::WalBarrier(PhysicalWalBarrierExecutorCommand {
            work,
            artifact,
            binding_digest,
        }))
    }
}

fn require_retry_range(
    work: &ResourceAdmittedPhysicalWork,
    offset: u64,
    byte_count: u64,
) -> Result<(), PhysicalExecutorCommandDenial> {
    let scope = work
        .intent()
        .scope()
        .wal_append_target()
        .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
    (scope.offset() == offset && scope.byte_count() == byte_count)
        .then_some(())
        .ok_or(PhysicalExecutorCommandDenial::RetryRangeMismatch)
}
