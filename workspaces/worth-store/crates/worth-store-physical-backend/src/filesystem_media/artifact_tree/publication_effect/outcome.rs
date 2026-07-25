use super::super::{ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFile};
use crate::{
    filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity},
    BackendQueueExecutionCompletion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactTreePublicationEffect {
    FileSynchronization(ArtifactTreeFile),
    DirectorySynchronization(ArtifactTreeDirectory),
    Replacement {
        source: ArtifactTreeFile,
        destination: ArtifactTreeFile,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedArtifactTreePublicationEffect {
    pub(super) owner: MediaOwnerIdentity,
    pub(super) store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    pub(super) operation: MediaOperationIdentity,
    pub(super) effect: ArtifactTreePublicationEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateArtifactTreePublicationEffect {
    pub(super) failure: ArtifactTreeFailure,
    pub(super) owner: MediaOwnerIdentity,
    pub(super) store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    pub(super) operation: MediaOperationIdentity,
    pub(super) effect: ArtifactTreePublicationEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactTreePublicationEffectOutcome {
    Completed(CompletedArtifactTreePublicationEffect),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactTreePublicationEffect),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledArtifactTreePublicationEffect {
    pub(super) physical: CompletedArtifactTreePublicationEffect,
    pub(super) queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactTreePublicationEffectOutcome {
    Completed(Box<CompletedScheduledArtifactTreePublicationEffect>),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactTreePublicationEffect),
}

impl CompletedArtifactTreePublicationEffect {
    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn effect(&self) -> &ArtifactTreePublicationEffect {
        &self.effect
    }
}

impl IndeterminateArtifactTreePublicationEffect {
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }

    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn effect(&self) -> &ArtifactTreePublicationEffect {
        &self.effect
    }
}

impl CompletedScheduledArtifactTreePublicationEffect {
    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}
