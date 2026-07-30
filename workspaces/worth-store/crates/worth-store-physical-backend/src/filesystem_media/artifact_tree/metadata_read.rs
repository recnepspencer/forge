use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordArtifactFile};

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia};
use crate::{
    filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity},
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedArtifactMetadataRead {
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    artifact: RecordArtifactFile,
    file_length: u64,
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedScheduledArtifactMetadataRead {
    physical: CompletedArtifactMetadataRead,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(
    clippy::large_enum_variant,
    reason = "the move-owned queue completion stays inline to avoid a heap allocation per metadata read"
)]
pub enum ScheduledArtifactMetadataReadOutcome {
    Completed(CompletedScheduledArtifactMetadataRead),
    DeniedBeforeEffect(ArtifactTreeFailure),
}

impl CompletedArtifactMetadataRead {
    pub const fn owner(self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn artifact(self) -> RecordArtifactFile {
        self.artifact
    }

    pub const fn file_length(self) -> u64 {
        self.file_length
    }

    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }
}

impl CompletedScheduledArtifactMetadataRead {
    pub const fn physical(self) -> CompletedArtifactMetadataRead {
        self.physical
    }

    pub const fn queue(self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl ArtifactTreeMedia<'_> {
    pub fn read_scheduled_file_length(
        &self,
        file: &ArtifactTreeFile,
        artifact: RecordArtifactFile,
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactMetadataReadOutcome {
        if file.file_name != artifact.file_name() {
            return denied(ArtifactTreeFailureKind::AccessLimitExceeded);
        }
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => return denied(ArtifactTreeFailureKind::DeniedBeforeEffect),
        };
        let directory = match self.open_directory(&file.directory) {
            Ok(directory) => directory,
            Err(failure) => {
                return ScheduledArtifactMetadataReadOutcome::DeniedBeforeEffect(failure)
            }
        };
        let file = match self.open_readable_file(&directory, &file.file_name) {
            Ok(file) => file,
            Err(failure) => {
                return ScheduledArtifactMetadataReadOutcome::DeniedBeforeEffect(failure)
            }
        };
        match super::super::artifact_tree_effects::identified_artifact_file_length(
            self.owner, &file,
        ) {
            Ok((operation, file_length)) => ScheduledArtifactMetadataReadOutcome::Completed(
                CompletedScheduledArtifactMetadataRead {
                    physical: CompletedArtifactMetadataRead {
                        owner: self.owner.identity(),
                        store: self.store,
                        artifact,
                        file_length,
                        operation,
                    },
                    queue: ticket.begin_completion().observe_queue_depth(1).complete(),
                },
            ),
            Err(failure) => ScheduledArtifactMetadataReadOutcome::DeniedBeforeEffect(failure),
        }
    }
}

fn denied(kind: ArtifactTreeFailureKind) -> ScheduledArtifactMetadataReadOutcome {
    ScheduledArtifactMetadataReadOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(kind))
}
