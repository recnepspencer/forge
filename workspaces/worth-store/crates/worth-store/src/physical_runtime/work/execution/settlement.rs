use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactRangeWriteDurability, ArtifactTreeFailure, CompletedArtifactAppend,
    CompletedArtifactMetadataRead, CompletedArtifactNewWrite, CompletedArtifactRangeRead,
    CompletedArtifactRangeWrite, MediaOperationRole,
};

use super::super::{
    PhysicalWorkRecoveryDisposition, PhysicalWorkRecoveryTarget, SettledPhysicalWork,
};
use super::{CompletedPhysicalPublicationEffect, CompletedPhysicalWalBarrier};

mod classification;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkEffectFate {
    ProvenNoEffect,
    ReadCompleted,
    ReadIncomplete,
    WriteCompleted,
    PublicationCompleted,
    WrittenButSchedulerRejected,
    Indeterminate,
    StaleOrForeignOutcome,
}

pub enum PhysicalWorkSettlementEvidence {
    NoEffect(PhysicalWorkNoEffectEvidence),
    Metadata {
        physical: CompletedArtifactMetadataRead,
        scheduler: QueueExecutionOutcome,
    },
    Read {
        physical: CompletedArtifactRangeRead,
        bytes: Box<[u8]>,
        scheduler: QueueExecutionOutcome,
    },
    Write {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
    },
    Publication {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
    },
    NewArtifact {
        physical: CompletedArtifactNewWrite,
        scheduler: QueueExecutionOutcome,
    },
    PublicationEffect {
        physical: CompletedPhysicalPublicationEffect,
        scheduler: QueueExecutionOutcome,
    },
    WalAppend {
        physical: CompletedArtifactAppend,
        scheduler: QueueExecutionOutcome,
    },
    WalBarrier {
        physical: CompletedPhysicalWalBarrier,
        scheduler: QueueExecutionOutcome,
    },
    TerminalFailure(PhysicalWorkTerminalFailure),
    StaleOrForeign,
}

pub struct PhysicalWorkNoEffectEvidence {
    failure: ArtifactTreeFailure,
    pub(in crate::physical_runtime) retry: super::PhysicalRetryPayload,
}

impl PhysicalWorkNoEffectEvidence {
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSchedulerPosture {
    NotObserved,
    Executed,
    RejectedAfterEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkPublicationResiduePosture {
    NotApplicable,
    NoneObserved,
    MayExist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkTerminalCause {
    Backend(ArtifactTreeFailure),
    IncompleteRead { expected: u64, completed: u64 },
    SchedulerRejectedAfterEffect,
}

pub struct PhysicalWorkTerminalFailure {
    identity: super::super::PhysicalWorkIdentity,
    effect_fate: PhysicalWorkEffectFate,
    target: PhysicalWorkRecoveryTarget,
    completed_bytes: u64,
    backend_operation: worth_store_physical_backend::MediaOperationIdentity,
    backend_role: MediaOperationRole,
    scheduler: PhysicalWorkSchedulerPosture,
    publication_residue: PhysicalWorkPublicationResiduePosture,
    recovery: PhysicalWorkRecoveryDisposition,
    cause: PhysicalWorkTerminalCause,
}

pub struct PhysicalWorkHealthRevocation {
    identity: super::super::PhysicalWorkIdentity,
    fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
}

pub(in crate::physical_runtime) struct PhysicalWorkSettlement;

pub(in crate::physical_runtime) struct PhysicalWorkSettlementResult {
    settled: SettledPhysicalWork,
    health_revocation: Option<PhysicalWorkHealthRevocation>,
    effect_activity: super::super::submission::PhysicalEffectActivity,
    residency_writeback: Option<super::PhysicalResidencyWritebackCompletion>,
}

impl PhysicalWorkSettlement {
    pub(in crate::physical_runtime) fn settle(
        dispatch: super::PhysicalExecutorDispatch,
    ) -> PhysicalWorkSettlementResult {
        let (mut dispatched, outcome, recovery_obligation, residency_writeback) =
            dispatch.into_parts();
        let effect_activity = dispatched.take_effect_activity();
        let evidence = classification::classify(&dispatched, outcome);
        let residency_writeback = match residency_writeback {
            Some(completion)
                if completion.identity() == dispatched.intent().identity()
                    && matches!(evidence, PhysicalWorkSettlementEvidence::Write { .. }) =>
            {
                Some(completion)
            }
            _ => None,
        };
        let health_revocation =
            classification::health_revocation(&dispatched, &evidence).or_else(|| {
                recovery_obligation
                    .is_retained()
                    .then_some(PhysicalWorkHealthRevocation {
                        identity: dispatched.intent().identity(),
                        fate: evidence.fate(),
                        recovery: PhysicalWorkRecoveryDisposition::InspectionRequired,
                    })
            });
        PhysicalWorkSettlementResult {
            settled: SettledPhysicalWork::from_settlement(
                dispatched,
                evidence,
                recovery_obligation,
            ),
            health_revocation,
            effect_activity,
            residency_writeback,
        }
    }
}

impl PhysicalWorkSettlementResult {
    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        SettledPhysicalWork,
        Option<PhysicalWorkHealthRevocation>,
        super::super::submission::PhysicalEffectActivity,
        Option<super::PhysicalResidencyWritebackCompletion>,
    ) {
        (
            self.settled,
            self.health_revocation,
            self.effect_activity,
            self.residency_writeback,
        )
    }
}

impl PhysicalWorkSettlementEvidence {
    pub(in crate::physical_runtime) const fn backend_role(&self) -> Option<MediaOperationRole> {
        match self {
            Self::NoEffect(_) | Self::StaleOrForeign => None,
            Self::Metadata { .. } => Some(MediaOperationRole::ReadMetadata),
            Self::Read { .. } => Some(MediaOperationRole::PositionedRead),
            Self::Write { .. } | Self::Publication { .. } | Self::NewArtifact { .. } => {
                Some(MediaOperationRole::PositionedWrite)
            }
            Self::WalAppend { .. } => Some(MediaOperationRole::PositionedWrite),
            Self::WalBarrier { .. } => Some(MediaOperationRole::SynchronizeFileState),
            Self::PublicationEffect { physical, .. } => {
                Some(publication_effect_role(physical.effect()))
            }
            Self::TerminalFailure(failure) => Some(failure.backend_role),
        }
    }

    pub const fn fate(&self) -> PhysicalWorkEffectFate {
        match self {
            Self::NoEffect(_) => PhysicalWorkEffectFate::ProvenNoEffect,
            Self::Metadata { .. } => PhysicalWorkEffectFate::ReadCompleted,
            Self::Read { .. } => PhysicalWorkEffectFate::ReadCompleted,
            Self::Write { scheduler, .. } => {
                if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
                    PhysicalWorkEffectFate::WriteCompleted
                } else {
                    PhysicalWorkEffectFate::WrittenButSchedulerRejected
                }
            }
            Self::Publication { scheduler, .. } => {
                if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
                    PhysicalWorkEffectFate::PublicationCompleted
                } else {
                    PhysicalWorkEffectFate::WrittenButSchedulerRejected
                }
            }
            Self::NewArtifact { scheduler, .. } | Self::PublicationEffect { scheduler, .. } => {
                if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
                    PhysicalWorkEffectFate::PublicationCompleted
                } else {
                    PhysicalWorkEffectFate::WrittenButSchedulerRejected
                }
            }
            Self::WalAppend { scheduler, .. } => {
                if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
                    PhysicalWorkEffectFate::WriteCompleted
                } else {
                    PhysicalWorkEffectFate::WrittenButSchedulerRejected
                }
            }
            Self::WalBarrier { scheduler, .. } => {
                if matches!(scheduler, QueueExecutionOutcome::Executed(_)) {
                    PhysicalWorkEffectFate::PublicationCompleted
                } else {
                    PhysicalWorkEffectFate::WrittenButSchedulerRejected
                }
            }
            Self::TerminalFailure(failure) => failure.effect_fate,
            Self::StaleOrForeign => PhysicalWorkEffectFate::StaleOrForeignOutcome,
        }
    }

    pub const fn completed_payload_bytes(&self) -> u64 {
        match self {
            Self::NoEffect(_) | Self::Metadata { .. } | Self::StaleOrForeign => 0,
            Self::Read { physical, .. } => physical.completed_bytes(),
            Self::Write { physical, .. } | Self::Publication { physical, .. } => {
                physical.completed_bytes()
            }
            Self::NewArtifact { physical, .. } => physical.write().completed_bytes(),
            Self::WalAppend { physical, .. } => physical.range().byte_count(),
            Self::WalBarrier { .. } => 0,
            Self::PublicationEffect { .. } => 0,
            Self::TerminalFailure(failure) => failure.completed_bytes,
        }
    }

    pub(in crate::physical_runtime::work) const fn recovery_disposition(
        &self,
        declared: PhysicalWorkRecoveryDisposition,
    ) -> PhysicalWorkRecoveryDisposition {
        match self {
            Self::TerminalFailure(failure) => failure.recovery,
            Self::StaleOrForeign => PhysicalWorkRecoveryDisposition::InspectionRequired,
            Self::NoEffect(_) => declared,
            Self::Metadata { .. } | Self::Read { .. } => PhysicalWorkRecoveryDisposition::NoEffect,
            Self::Write { .. }
            | Self::Publication { .. }
            | Self::NewArtifact { .. }
            | Self::PublicationEffect { .. }
            | Self::WalAppend { .. }
            | Self::WalBarrier { .. } => PhysicalWorkRecoveryDisposition::ContinueSettlement,
        }
    }
}

impl PhysicalWorkTerminalFailure {
    pub const fn identity(&self) -> super::super::PhysicalWorkIdentity {
        self.identity
    }

    pub const fn effect_fate(&self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn target(&self) -> PhysicalWorkRecoveryTarget {
        self.target
    }

    pub const fn coordinate(&self) -> Option<worth_store_physical_format::RecordFrameCoordinate> {
        match self.target {
            PhysicalWorkRecoveryTarget::Range(coordinate) => Some(coordinate),
            _ => None,
        }
    }

    pub const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }

    pub const fn backend_operation(&self) -> worth_store_physical_backend::MediaOperationIdentity {
        self.backend_operation
    }

    pub const fn backend_role(&self) -> MediaOperationRole {
        self.backend_role
    }

    pub const fn recovery(&self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }

    pub const fn scheduler(&self) -> PhysicalWorkSchedulerPosture {
        self.scheduler
    }

    pub const fn publication_residue(&self) -> PhysicalWorkPublicationResiduePosture {
        self.publication_residue
    }

    pub const fn cause(&self) -> PhysicalWorkTerminalCause {
        self.cause
    }
}

pub(in crate::physical_runtime) const fn publication_effect_role(
    effect: super::PhysicalPublicationEffect,
) -> MediaOperationRole {
    match effect {
        super::PhysicalPublicationEffect::SynchronizeArtifact => {
            MediaOperationRole::SynchronizeFileState
        }
        super::PhysicalPublicationEffect::SynchronizeArtifactParent
        | super::PhysicalPublicationEffect::SynchronizeRecordFamily => {
            MediaOperationRole::SynchronizeDirectoryPublication
        }
        super::PhysicalPublicationEffect::ReplaceCatalog => MediaOperationRole::AtomicReplace,
    }
}

impl PhysicalWorkHealthRevocation {
    pub const fn identity(&self) -> super::super::PhysicalWorkIdentity {
        self.identity
    }

    pub const fn fate(&self) -> PhysicalWorkEffectFate {
        self.fate
    }

    pub const fn recovery(&self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}

pub(in crate::physical_runtime::work) fn durability_satisfies(
    declared: super::super::PhysicalWorkDurabilityRequirement,
    observed: ArtifactRangeWriteDurability,
) -> bool {
    use super::super::PhysicalWorkDurabilityRequirement;
    use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
    match declared {
        PhysicalWorkDurabilityRequirement::ReadOnly => false,
        PhysicalWorkDurabilityRequirement::WalAppend => false,
        PhysicalWorkDurabilityRequirement::WalDurabilityBarrier => false,
        PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(requirement) => match requirement {
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite => true,
            ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization => {
                observed == ArtifactRangeWriteDurability::FileDataSynchronized
            }
        },
    }
}
