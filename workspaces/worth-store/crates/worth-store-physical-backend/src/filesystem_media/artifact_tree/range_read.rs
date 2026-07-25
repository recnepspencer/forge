use std::io::{Seek, SeekFrom};

use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia};
use crate::filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedArtifactRangeRead {
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    completed_bytes: u64,
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRangeReadOutcome {
    Completed(CompletedArtifactRangeRead),
    DeniedBeforeEffect(ArtifactTreeFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedScheduledArtifactRangeRead {
    physical: CompletedArtifactRangeRead,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactRangeReadOutcome {
    Completed(Box<CompletedScheduledArtifactRangeRead>),
    DeniedBeforeEffect(ArtifactTreeFailure),
}

impl CompletedArtifactRangeRead {
    pub const fn owner(self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn completed_bytes(self) -> u64 {
        self.completed_bytes
    }

    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }
}

impl CompletedScheduledArtifactRangeRead {
    pub const fn physical(&self) -> CompletedArtifactRangeRead {
        self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl ArtifactTreeMedia<'_> {
    pub fn read_scheduled_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        target: &mut [u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactRangeReadOutcome {
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return ScheduledArtifactRangeReadOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        match self.read_exact_range(artifact, coordinate, target) {
            ArtifactRangeReadOutcome::Completed(physical) => {
                ScheduledArtifactRangeReadOutcome::Completed(Box::new(
                    CompletedScheduledArtifactRangeRead {
                        physical,
                        queue: ticket.begin_completion().observe_queue_depth(1).complete(),
                    },
                ))
            }
            ArtifactRangeReadOutcome::DeniedBeforeEffect(failure) => {
                ScheduledArtifactRangeReadOutcome::DeniedBeforeEffect(failure)
            }
        }
    }

    fn read_exact_range(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        target: &mut [u8],
    ) -> ArtifactRangeReadOutcome {
        if target.len() != coordinate.length() as usize
            || artifact.file_name != coordinate.artifact().file_name()
        {
            return denied(ArtifactTreeFailureKind::AccessLimitExceeded);
        }
        let directory = match self.open_directory(&artifact.directory) {
            Ok(directory) => directory,
            Err(failure) => return ArtifactRangeReadOutcome::DeniedBeforeEffect(failure),
        };
        let mut file = match self.open_readable_file(&directory, &artifact.file_name) {
            Ok(file) => file,
            Err(failure) => return ArtifactRangeReadOutcome::DeniedBeforeEffect(failure),
        };
        let end = match coordinate.offset().checked_add(target.len() as u64) {
            Some(end) => end,
            None => return denied(ArtifactTreeFailureKind::AccessLimitExceeded),
        };
        match super::super::artifact_tree_effects::artifact_file_length(self.owner, &file) {
            Ok(length) if end <= length => {}
            Ok(_) => return denied(ArtifactTreeFailureKind::Damaged),
            Err(failure) => return ArtifactRangeReadOutcome::DeniedBeforeEffect(failure),
        }
        if let Err(error) = file.seek(SeekFrom::Start(coordinate.offset())) {
            return ArtifactRangeReadOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        match super::exact_read_effect::execute(self.owner, &mut file, target) {
            super::exact_read_effect::ExactReadEffect::Completed {
                operation,
                completed_bytes,
            } => ArtifactRangeReadOutcome::Completed(CompletedArtifactRangeRead {
                owner: self.owner.identity(),
                store: self.store,
                coordinate,
                completed_bytes,
                operation,
            }),
            super::exact_read_effect::ExactReadEffect::DeniedBeforeEffect(failure) => {
                ArtifactRangeReadOutcome::DeniedBeforeEffect(failure)
            }
        }
    }
}

fn denied(kind: ArtifactTreeFailureKind) -> ArtifactRangeReadOutcome {
    ArtifactRangeReadOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(kind))
}
