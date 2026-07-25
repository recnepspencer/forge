use sha2::{Digest, Sha256};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::{
    range_io::ArtifactTreeCreateFileOutcome, ArtifactRangeWriteOutcome, ArtifactTreeFailure,
    ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia, CompletedArtifactRangeWrite,
    IndeterminateArtifactRangeWrite,
};
use crate::{
    filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity},
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedArtifactNewWrite {
    create_operation: MediaOperationIdentity,
    write: CompletedArtifactRangeWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateArtifactNewWrite {
    failure: ArtifactTreeFailure,
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    completed_bytes: u64,
    create_operation: MediaOperationIdentity,
    write_operation: Option<MediaOperationIdentity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledArtifactNewWrite {
    physical: CompletedArtifactNewWrite,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactNewWriteOutcome {
    Completed(CompletedArtifactNewWrite),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactNewWrite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactNewWriteOutcome {
    Completed(Box<CompletedScheduledArtifactNewWrite>),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactNewWrite),
}

impl ArtifactTreeMedia<'_> {
    pub fn write_new_exact(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
    ) -> ArtifactNewWriteOutcome {
        if coordinate.offset() != 0
            || coordinate.length() as usize != bytes.len()
            || coordinate.artifact().file_name() != artifact.file_name
        {
            return ArtifactNewWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let mut file = match self.create_new_file_observed(artifact) {
            ArtifactTreeCreateFileOutcome::Created(file) => file,
            ArtifactTreeCreateFileOutcome::DeniedBeforeEffect(failure) => {
                return ArtifactNewWriteOutcome::DeniedBeforeEffect(failure);
            }
            ArtifactTreeCreateFileOutcome::Indeterminate { failure, operation } => {
                return ArtifactNewWriteOutcome::Indeterminate(
                    IndeterminateArtifactNewWrite::after_create(
                        failure,
                        self.owner.identity(),
                        self.store,
                        coordinate,
                        bytes,
                        operation,
                    ),
                );
            }
        };
        let create_operation = file.create_operation();
        match file.write_exact_chunk(coordinate, bytes) {
            ArtifactRangeWriteOutcome::Completed(write) => {
                ArtifactNewWriteOutcome::Completed(CompletedArtifactNewWrite {
                    create_operation,
                    write,
                })
            }
            ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure) => {
                ArtifactNewWriteOutcome::Indeterminate(IndeterminateArtifactNewWrite::after_create(
                    failure,
                    self.owner.identity(),
                    self.store,
                    coordinate,
                    bytes,
                    create_operation,
                ))
            }
            ArtifactRangeWriteOutcome::Indeterminate(write) => {
                ArtifactNewWriteOutcome::Indeterminate(IndeterminateArtifactNewWrite::during_write(
                    create_operation,
                    write,
                ))
            }
        }
    }

    pub fn write_scheduled_new_exact(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactNewWriteOutcome {
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return ScheduledArtifactNewWriteOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        match self.write_new_exact(artifact, coordinate, bytes) {
            ArtifactNewWriteOutcome::Completed(physical) => {
                ScheduledArtifactNewWriteOutcome::Completed(Box::new(
                    CompletedScheduledArtifactNewWrite {
                        physical,
                        queue: ticket.begin_completion().observe_queue_depth(1).complete(),
                    },
                ))
            }
            ArtifactNewWriteOutcome::DeniedBeforeEffect(failure) => {
                ScheduledArtifactNewWriteOutcome::DeniedBeforeEffect(failure)
            }
            ArtifactNewWriteOutcome::Indeterminate(failure) => {
                ScheduledArtifactNewWriteOutcome::Indeterminate(failure)
            }
        }
    }
}

impl CompletedArtifactNewWrite {
    pub const fn create_operation(&self) -> MediaOperationIdentity {
        self.create_operation
    }

    pub const fn write(&self) -> &CompletedArtifactRangeWrite {
        &self.write
    }

    pub fn into_write(self) -> CompletedArtifactRangeWrite {
        self.write
    }
}

impl CompletedScheduledArtifactNewWrite {
    pub const fn physical(&self) -> &CompletedArtifactNewWrite {
        &self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl IndeterminateArtifactNewWrite {
    fn after_create(
        failure: ArtifactTreeFailure,
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        create_operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            failure,
            owner,
            store,
            coordinate,
            payload_digest: Sha256::digest(bytes).into(),
            completed_bytes: 0,
            create_operation,
            write_operation: None,
        }
    }

    fn during_write(
        create_operation: MediaOperationIdentity,
        write: IndeterminateArtifactRangeWrite,
    ) -> Self {
        Self {
            failure: write.failure(),
            owner: write.owner(),
            store: write.store(),
            coordinate: write.coordinate(),
            payload_digest: write.payload_digest(),
            completed_bytes: write.completed_bytes(),
            create_operation,
            write_operation: Some(write.operation()),
        }
    }

    pub const fn failure(self) -> ArtifactTreeFailure {
        self.failure
    }

    pub const fn owner(self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn completed_bytes(self) -> u64 {
        self.completed_bytes
    }

    pub const fn create_operation(self) -> MediaOperationIdentity {
        self.create_operation
    }

    pub const fn write_operation(self) -> Option<MediaOperationIdentity> {
        self.write_operation
    }
}
