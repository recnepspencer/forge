use sha2::{Digest, Sha256};
use worth_store_buffer_pool::PhysicalWritebackClaim;
use worth_store_physical_format::RecordFrameCoordinate;

use super::super::{PhysicalWorkOperationFamily, ResourceAdmittedPhysicalWork};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalExecutorCommandDenial {
    OperationFamilyMismatch,
    ExactCommandRequiresOneRange,
    PayloadLengthMismatch,
    RetryIdentityMismatch,
    RetryRangeMismatch,
    ResidencyRetryRequiresClaim,
    ArtifactCommandRequiresArtifactScope,
    WalAppendCommandRequiresWalScope,
    WalBarrierCommandRequiresWalScope,
}

pub enum PhysicalExecutorCommand {
    Metadata(PhysicalMetadataExecutorCommand),
    Read(PhysicalReadExecutorCommand),
    ExactWrite(PhysicalWriteExecutorCommand),
    Publication(PhysicalWriteExecutorCommand),
    NewArtifact(PhysicalWriteExecutorCommand),
    PublicationEffect(PhysicalPublicationExecutorCommand),
    ResidencyWriteback(PhysicalResidencyWritebackExecutorCommand),
    WalAppend(PhysicalWalAppendExecutorCommand),
    WalBarrier(PhysicalWalBarrierExecutorCommand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPublicationEffect {
    SynchronizeArtifact,
    SynchronizeArtifactParent,
    ReplaceCatalog,
    SynchronizeRecordFamily,
}

pub struct PhysicalRetryCommand {
    identity: super::super::PhysicalWorkIdentity,
    payload: PhysicalRetryPayload,
}

pub(in crate::physical_runtime) enum PhysicalRetryPayload {
    Metadata,
    Read,
    ExactWrite(Box<[u8]>),
    Publication(Box<[u8]>),
    NewArtifact(Box<[u8]>),
    PublicationEffect(PhysicalPublicationEffect),
    ResidencyWriteback,
    WalAppend {
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        range: worth_store_physical_backend::ArtifactAppendRange,
        payload: Box<[u8]>,
    },
    WalBarrier {
        artifact: worth_store_physical_backend::ArtifactTreeFile,
        binding_digest: [u8; 32],
    },
}

pub struct PhysicalMetadataExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) artifact: worth_store_physical_format::RecordArtifactFile,
}

pub struct PhysicalReadExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) coordinate: RecordFrameCoordinate,
    pub(in crate::physical_runtime) destination: Box<[u8]>,
}

pub struct PhysicalWriteExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) coordinate: RecordFrameCoordinate,
    pub(in crate::physical_runtime) payload: Box<[u8]>,
    pub(in crate::physical_runtime) payload_digest: [u8; 32],
}

pub struct PhysicalResidencyWritebackExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) claim: PhysicalWritebackClaim,
}

pub struct PhysicalPublicationExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) artifact: worth_store_physical_format::RecordArtifactFile,
    pub(in crate::physical_runtime) effect: PhysicalPublicationEffect,
}

pub struct PhysicalWalAppendExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) artifact: worth_store_physical_backend::ArtifactTreeFile,
    pub(in crate::physical_runtime) range: worth_store_physical_backend::ArtifactAppendRange,
    pub(in crate::physical_runtime) payload: Box<[u8]>,
    pub(in crate::physical_runtime) payload_digest: [u8; 32],
}

pub struct PhysicalWalBarrierExecutorCommand {
    pub(in crate::physical_runtime) work: ResourceAdmittedPhysicalWork,
    pub(in crate::physical_runtime) artifact: worth_store_physical_backend::ArtifactTreeFile,
    pub(in crate::physical_runtime) binding_digest: [u8; 32],
}

impl PhysicalExecutorCommand {
    pub fn metadata(
        work: ResourceAdmittedPhysicalWork,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactMetadataRead)?;
        let artifact = work
            .intent()
            .scope()
            .artifact_target()
            .ok_or(PhysicalExecutorCommandDenial::ArtifactCommandRequiresArtifactScope)?;
        Ok(Self::Metadata(PhysicalMetadataExecutorCommand {
            work,
            artifact,
        }))
    }

    pub fn read(work: ResourceAdmittedPhysicalWork) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactRangeRead)?;
        let coordinate = exact_coordinate(&work)?;
        Ok(Self::Read(PhysicalReadExecutorCommand {
            work,
            coordinate,
            destination: vec![0_u8; coordinate.length() as usize].into_boxed_slice(),
        }))
    }

    pub fn exact_write(
        work: ResourceAdmittedPhysicalWork,
        payload: impl Into<Box<[u8]>>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactRangeWrite)?;
        Ok(Self::ExactWrite(PhysicalWriteExecutorCommand::new(
            work, payload,
        )?))
    }

    pub fn publication(
        work: ResourceAdmittedPhysicalWork,
        payload: impl Into<Box<[u8]>>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactPublication)?;
        Ok(Self::Publication(PhysicalWriteExecutorCommand::new(
            work, payload,
        )?))
    }

    pub fn new_artifact(
        work: ResourceAdmittedPhysicalWork,
        payload: impl Into<Box<[u8]>>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactPublication)?;
        Ok(Self::NewArtifact(PhysicalWriteExecutorCommand::new(
            work, payload,
        )?))
    }

    pub fn publication_effect(
        work: ResourceAdmittedPhysicalWork,
        effect: PhysicalPublicationEffect,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::ArtifactPublication)?;
        let artifact = work
            .intent()
            .scope()
            .artifact_target()
            .ok_or(PhysicalExecutorCommandDenial::ArtifactCommandRequiresArtifactScope)?;
        if matches!(effect, PhysicalPublicationEffect::ReplaceCatalog)
            && !matches!(
                artifact,
                worth_store_physical_format::RecordArtifactFile::CatalogCandidate { .. }
            )
        {
            return Err(PhysicalExecutorCommandDenial::ArtifactCommandRequiresArtifactScope);
        }
        if matches!(effect, PhysicalPublicationEffect::SynchronizeRecordFamily)
            && artifact != worth_store_physical_format::RecordArtifactFile::BootstrapCatalog
        {
            return Err(PhysicalExecutorCommandDenial::ArtifactCommandRequiresArtifactScope);
        }
        Ok(Self::PublicationEffect(
            PhysicalPublicationExecutorCommand {
                work,
                artifact,
                effect,
            },
        ))
    }

    pub(in crate::physical_runtime) fn wal_append(
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
        let range = worth_store_physical_backend::ArtifactAppendRange::new(
            scope.offset(),
            scope.byte_count(),
        )
        .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
        let payload = payload.into();
        if payload.len() as u64 != range.byte_count() {
            return Err(PhysicalExecutorCommandDenial::PayloadLengthMismatch);
        }
        let payload_digest = Sha256::digest(&payload).into();
        Ok(Self::WalAppend(PhysicalWalAppendExecutorCommand {
            work,
            artifact,
            range,
            payload,
            payload_digest,
        }))
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

    pub const fn identity(&self) -> super::super::PhysicalWorkIdentity {
        match self {
            Self::Metadata(command) => command.work.intent().identity(),
            Self::Read(command) => command.work.intent().identity(),
            Self::ExactWrite(command) | Self::Publication(command) => {
                command.work.intent().identity()
            }
            Self::NewArtifact(command) => command.work.intent().identity(),
            Self::PublicationEffect(command) => command.work.intent().identity(),
            Self::ResidencyWriteback(command) => command.work.intent().identity(),
            Self::WalAppend(command) => command.work.intent().identity(),
            Self::WalBarrier(command) => command.work.intent().identity(),
        }
    }

    pub(in crate::physical_runtime) const fn intent(&self) -> &super::super::PhysicalWorkIntent {
        match self {
            Self::Metadata(command) => command.work.intent(),
            Self::Read(command) => command.work.intent(),
            Self::ExactWrite(command) | Self::Publication(command) => command.work.intent(),
            Self::NewArtifact(command) => command.work.intent(),
            Self::PublicationEffect(command) => command.work.intent(),
            Self::ResidencyWriteback(command) => command.work.intent(),
            Self::WalAppend(command) => command.work.intent(),
            Self::WalBarrier(command) => command.work.intent(),
        }
    }

    pub(in crate::physical_runtime) fn is_cancelled(&self) -> bool {
        match self {
            Self::Metadata(command) => command.work.is_cancelled(),
            Self::Read(command) => command.work.is_cancelled(),
            Self::ExactWrite(command) | Self::Publication(command) => command.work.is_cancelled(),
            Self::NewArtifact(command) => command.work.is_cancelled(),
            Self::PublicationEffect(command) => command.work.is_cancelled(),
            Self::ResidencyWriteback(command) => command.work.is_cancelled(),
            Self::WalAppend(command) => command.work.is_cancelled(),
            Self::WalBarrier(command) => command.work.is_cancelled(),
        }
    }

    pub(in crate::physical_runtime) const fn wal_append_completion_binding(
        &self,
    ) -> Option<(worth_store_physical_backend::ArtifactAppendRange, [u8; 32])> {
        match self {
            Self::WalAppend(command) => Some((command.range, command.payload_digest)),
            _ => None,
        }
    }

    pub(in crate::physical_runtime) const fn wal_barrier_completion_binding(
        &self,
    ) -> Option<(super::super::PhysicalWalBarrierScope, [u8; 32])> {
        match self {
            Self::WalBarrier(command) => Some((
                command
                    .work
                    .intent()
                    .scope()
                    .wal_barrier_target()
                    .expect("WAL barrier commands retain exact WAL barrier scope"),
                command.binding_digest,
            )),
            _ => None,
        }
    }

    pub(in crate::physical_runtime) fn residency_writeback(
        work: ResourceAdmittedPhysicalWork,
        claim: PhysicalWritebackClaim,
    ) -> Self {
        Self::ResidencyWriteback(PhysicalResidencyWritebackExecutorCommand { work, claim })
    }
}

impl PhysicalRetryCommand {
    pub(in crate::physical_runtime) const fn new(
        identity: super::super::PhysicalWorkIdentity,
        payload: PhysicalRetryPayload,
    ) -> Self {
        Self { identity, payload }
    }

    pub const fn identity(&self) -> super::super::PhysicalWorkIdentity {
        self.identity
    }

    pub fn bind(
        self,
        work: ResourceAdmittedPhysicalWork,
    ) -> Result<PhysicalExecutorCommand, PhysicalExecutorCommandDenial> {
        if work.intent().identity() != self.identity {
            return Err(PhysicalExecutorCommandDenial::RetryIdentityMismatch);
        }
        match self.payload {
            PhysicalRetryPayload::Metadata => PhysicalExecutorCommand::metadata(work),
            PhysicalRetryPayload::Read => PhysicalExecutorCommand::read(work),
            PhysicalRetryPayload::ExactWrite(payload) => {
                PhysicalExecutorCommand::exact_write(work, payload)
            }
            PhysicalRetryPayload::Publication(payload) => {
                PhysicalExecutorCommand::publication(work, payload)
            }
            PhysicalRetryPayload::NewArtifact(payload) => {
                PhysicalExecutorCommand::new_artifact(work, payload)
            }
            PhysicalRetryPayload::PublicationEffect(effect) => {
                PhysicalExecutorCommand::publication_effect(work, effect)
            }
            PhysicalRetryPayload::ResidencyWriteback => {
                Err(PhysicalExecutorCommandDenial::ResidencyRetryRequiresClaim)
            }
            PhysicalRetryPayload::WalAppend {
                artifact,
                range,
                payload,
            } => {
                let scope = work
                    .intent()
                    .scope()
                    .wal_append_target()
                    .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
                let rebound_range = worth_store_physical_backend::ArtifactAppendRange::new(
                    scope.offset(),
                    scope.byte_count(),
                )
                .ok_or(PhysicalExecutorCommandDenial::WalAppendCommandRequiresWalScope)?;
                if range != rebound_range {
                    return Err(PhysicalExecutorCommandDenial::RetryRangeMismatch);
                }
                PhysicalExecutorCommand::wal_append(work, artifact, payload)
            }
            PhysicalRetryPayload::WalBarrier {
                artifact,
                binding_digest,
            } => PhysicalExecutorCommand::wal_barrier(work, artifact, binding_digest),
        }
    }

    pub(in crate::physical_runtime) fn admit_residency_retry(
        self,
        work: &ResourceAdmittedPhysicalWork,
    ) -> Result<(), PhysicalExecutorCommandDenial> {
        if work.intent().identity() != self.identity {
            return Err(PhysicalExecutorCommandDenial::RetryIdentityMismatch);
        }
        matches!(self.payload, PhysicalRetryPayload::ResidencyWriteback)
            .then_some(())
            .ok_or(PhysicalExecutorCommandDenial::ResidencyRetryRequiresClaim)
    }
}

impl PhysicalWriteExecutorCommand {
    fn new(
        work: ResourceAdmittedPhysicalWork,
        payload: impl Into<Box<[u8]>>,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        let coordinate = exact_coordinate(&work)?;
        let payload = payload.into();
        if payload.len() != coordinate.length() as usize {
            return Err(PhysicalExecutorCommandDenial::PayloadLengthMismatch);
        }
        let payload_digest = Sha256::digest(&payload).into();
        Ok(Self {
            work,
            coordinate,
            payload,
            payload_digest,
        })
    }
}

fn require_family(
    work: &ResourceAdmittedPhysicalWork,
    expected: PhysicalWorkOperationFamily,
) -> Result<(), PhysicalExecutorCommandDenial> {
    (work.intent().operation() == expected)
        .then_some(())
        .ok_or(PhysicalExecutorCommandDenial::OperationFamilyMismatch)
}

fn exact_coordinate(
    work: &ResourceAdmittedPhysicalWork,
) -> Result<RecordFrameCoordinate, PhysicalExecutorCommandDenial> {
    let [coordinate] = work.intent().scope().coordinates() else {
        return Err(PhysicalExecutorCommandDenial::ExactCommandRequiresOneRange);
    };
    Ok(*coordinate)
}
