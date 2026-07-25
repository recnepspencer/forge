use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactTreeFailure, CompletedArtifactMetadataRead, CompletedArtifactNewWrite,
    CompletedArtifactRangeRead, CompletedArtifactRangeWrite,
    CompletedArtifactTreePublicationEffect, IndeterminateArtifactNewWrite,
    IndeterminateArtifactRangeWrite, IndeterminateArtifactTreePublicationEffect,
};
use worth_store_physical_format::RecordArtifactFile;

use super::PhysicalPublicationEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalEffectRecoveryObligation {
    Cleared,
    Retained,
}

pub(in crate::physical_runtime) struct PhysicalExecutorDispatch {
    dispatched: super::super::DispatchedPhysicalWork,
    outcome: PhysicalExecutorOutcome,
    recovery: PhysicalEffectRecoveryObligation,
}

pub struct CompletedPhysicalPublicationEffect {
    physical: CompletedArtifactTreePublicationEffect,
    artifact: RecordArtifactFile,
    effect: PhysicalPublicationEffect,
}

pub(in crate::physical_runtime) struct IndeterminatePhysicalPublicationEffect {
    physical: IndeterminateArtifactTreePublicationEffect,
    artifact: RecordArtifactFile,
    effect: PhysicalPublicationEffect,
}

pub enum PhysicalExecutorOutcome {
    DeniedBeforeEffect {
        failure: ArtifactTreeFailure,
        retry: super::PhysicalRetryPayload,
    },
    MetadataCompleted {
        physical: CompletedArtifactMetadataRead,
        scheduler: QueueExecutionOutcome,
    },
    ReadCompleted {
        physical: CompletedArtifactRangeRead,
        bytes: Box<[u8]>,
        scheduler: QueueExecutionOutcome,
    },
    WriteCompleted {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
    },
    ResidencyWritebackCompleted {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
    },
    ResidencyTerminal {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
        denial: worth_store_buffer_pool::PhysicalResidencyDenial,
    },
    PublicationCompleted {
        physical: CompletedArtifactRangeWrite,
        scheduler: QueueExecutionOutcome,
    },
    NewArtifactCompleted {
        physical: CompletedArtifactNewWrite,
        scheduler: QueueExecutionOutcome,
    },
    PublicationEffectCompleted {
        physical: CompletedPhysicalPublicationEffect,
        scheduler: QueueExecutionOutcome,
    },
    Indeterminate(IndeterminateArtifactRangeWrite),
    NewArtifactIndeterminate(IndeterminateArtifactNewWrite),
    PublicationEffectIndeterminate(IndeterminatePhysicalPublicationEffect),
}

impl PhysicalEffectRecoveryObligation {
    pub(in crate::physical_runtime) const fn join(self, other: Self) -> Self {
        if matches!(self, Self::Retained) || matches!(other, Self::Retained) {
            Self::Retained
        } else {
            Self::Cleared
        }
    }

    pub(in crate::physical_runtime) const fn is_retained(self) -> bool {
        matches!(self, Self::Retained)
    }
}

impl PhysicalExecutorDispatch {
    pub(in crate::physical_runtime) const fn new(
        dispatched: super::super::DispatchedPhysicalWork,
        outcome: PhysicalExecutorOutcome,
        recovery: PhysicalEffectRecoveryObligation,
    ) -> Self {
        Self {
            dispatched,
            outcome,
            recovery,
        }
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        super::super::DispatchedPhysicalWork,
        PhysicalExecutorOutcome,
        PhysicalEffectRecoveryObligation,
    ) {
        (self.dispatched, self.outcome, self.recovery)
    }
}

impl CompletedPhysicalPublicationEffect {
    pub(in crate::physical_runtime) const fn new(
        physical: CompletedArtifactTreePublicationEffect,
        artifact: RecordArtifactFile,
        effect: PhysicalPublicationEffect,
    ) -> Self {
        Self {
            physical,
            artifact,
            effect,
        }
    }

    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }

    pub const fn artifact(&self) -> RecordArtifactFile {
        self.artifact
    }

    pub const fn effect(&self) -> PhysicalPublicationEffect {
        self.effect
    }

    pub const fn recovery_target(&self) -> crate::physical_runtime::PhysicalWorkRecoveryTarget {
        publication_recovery_target(self.effect, self.artifact)
    }
}

impl IndeterminatePhysicalPublicationEffect {
    pub(in crate::physical_runtime) const fn new(
        physical: IndeterminateArtifactTreePublicationEffect,
        artifact: RecordArtifactFile,
        effect: PhysicalPublicationEffect,
    ) -> Self {
        Self {
            physical,
            artifact,
            effect,
        }
    }

    pub(in crate::physical_runtime) const fn physical(
        &self,
    ) -> &IndeterminateArtifactTreePublicationEffect {
        &self.physical
    }

    pub(in crate::physical_runtime) const fn artifact(&self) -> RecordArtifactFile {
        self.artifact
    }

    pub(in crate::physical_runtime) const fn effect(&self) -> PhysicalPublicationEffect {
        self.effect
    }

    pub(in crate::physical_runtime) const fn recovery_target(
        &self,
    ) -> crate::physical_runtime::PhysicalWorkRecoveryTarget {
        publication_recovery_target(self.effect, self.artifact)
    }
}

const fn publication_recovery_target(
    effect: PhysicalPublicationEffect,
    artifact: RecordArtifactFile,
) -> crate::physical_runtime::PhysicalWorkRecoveryTarget {
    match effect {
        PhysicalPublicationEffect::SynchronizeArtifact => {
            crate::physical_runtime::PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                artifact,
            )
        }
        PhysicalPublicationEffect::SynchronizeArtifactParent => {
            crate::physical_runtime::PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(
                artifact,
            )
        }
        PhysicalPublicationEffect::ReplaceCatalog => {
            crate::physical_runtime::PhysicalWorkRecoveryTarget::CatalogReplacement(artifact)
        }
        PhysicalPublicationEffect::SynchronizeRecordFamily => {
            crate::physical_runtime::PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization
        }
    }
}
