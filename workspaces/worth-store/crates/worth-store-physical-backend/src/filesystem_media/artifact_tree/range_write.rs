use std::io::{Seek, SeekFrom};

use sha2::{Digest, Sha256};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia};
use crate::filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding, BackendQueueSpeculativeScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRangeWriteDurability {
    BufferedWriteCompleted,
    FileDataSynchronized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRangeWriteDurabilityRequirement {
    BufferedWrite,
    FileDataSynchronization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedArtifactRangeWrite {
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    completed_bytes: u64,
    operation: MediaOperationIdentity,
    durability: ArtifactRangeWriteDurability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateArtifactRangeWrite {
    failure: ArtifactTreeFailure,
    coordinate: RecordFrameCoordinate,
    completed_bytes: u64,
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactRangeWriteOutcome {
    Completed(CompletedArtifactRangeWrite),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactRangeWrite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledArtifactRangeWrite {
    physical: CompletedArtifactRangeWrite,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactRangeWriteOutcome {
    Completed(Box<CompletedScheduledArtifactRangeWrite>),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactRangeWrite),
}

impl CompletedArtifactRangeWrite {
    pub(super) fn buffered(
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            owner,
            store,
            coordinate,
            payload_digest: Sha256::digest(bytes).into(),
            completed_bytes: bytes.len() as u64,
            operation,
            durability: ArtifactRangeWriteDurability::BufferedWriteCompleted,
        }
    }

    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn durability(&self) -> ArtifactRangeWriteDurability {
        self.durability
    }
}

impl IndeterminateArtifactRangeWrite {
    pub(super) const fn new(
        failure: ArtifactTreeFailure,
        coordinate: RecordFrameCoordinate,
        completed_bytes: u64,
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            failure,
            coordinate,
            completed_bytes,
            operation,
        }
    }

    pub const fn failure(self) -> ArtifactTreeFailure {
        self.failure
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

impl CompletedScheduledArtifactRangeWrite {
    pub const fn physical(&self) -> &CompletedArtifactRangeWrite {
        &self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl ArtifactTreeMedia<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn write_scheduled_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        scope: BackendQueueSpeculativeScope,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> ScheduledArtifactRangeWriteOutcome {
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return ScheduledArtifactRangeWriteOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        match self.write_exact_at_with_durability(artifact, coordinate, bytes, durability) {
            ArtifactRangeWriteOutcome::Completed(physical) => {
                let queue = ticket
                    .begin_completion()
                    .observe_queue_depth(1)
                    .observe_write_back(1, scope)
                    .complete();
                ScheduledArtifactRangeWriteOutcome::Completed(Box::new(
                    CompletedScheduledArtifactRangeWrite { physical, queue },
                ))
            }
            ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure) => {
                ScheduledArtifactRangeWriteOutcome::DeniedBeforeEffect(failure)
            }
            ArtifactRangeWriteOutcome::Indeterminate(failure) => {
                ScheduledArtifactRangeWriteOutcome::Indeterminate(failure)
            }
        }
    }

    pub fn write_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
    ) -> ArtifactRangeWriteOutcome {
        self.write_exact_at_with_durability(
            artifact,
            coordinate,
            bytes,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
    }

    fn write_exact_at_with_durability(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> ArtifactRangeWriteOutcome {
        if bytes.len() != coordinate.length() as usize
            || artifact.file_name != coordinate.artifact().file_name()
        {
            return ArtifactRangeWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let _coordination = match self
            .owner
            .begin_artifact_mutation(vec![artifact.coordination_key()])
        {
            Ok(coordination) => coordination,
            Err(_) => {
                return ArtifactRangeWriteOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        let directory = match self.open_directory(&artifact.directory) {
            Ok(directory) => directory,
            Err(failure) => return ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure),
        };
        let mut file = match self.open_mutable_file(&directory, &artifact.file_name) {
            Ok(file) => file,
            Err(failure) => return ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure),
        };
        let end = match coordinate.offset().checked_add(bytes.len() as u64) {
            Some(end) => end,
            None => {
                return ArtifactRangeWriteOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::AccessLimitExceeded),
                );
            }
        };
        match super::super::artifact_tree_effects::artifact_file_length(self.owner, &file) {
            Ok(length) if end <= length => {}
            Ok(_) => {
                return ArtifactRangeWriteOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::AccessLimitExceeded),
                );
            }
            Err(failure) => return ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure),
        }
        if let Err(error) = file.seek(SeekFrom::Start(coordinate.offset())) {
            return ArtifactRangeWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        let sequence_file = match file.try_clone().map(cap_std::fs::File::into_std) {
            Ok(file) => file,
            Err(error) => {
                return ArtifactRangeWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                    &error,
                ));
            }
        };
        let sequence = match self.owner.mutation_sequence_for(&sequence_file) {
            Ok(sequence) => sequence,
            Err(error) => {
                return ArtifactRangeWriteOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                    &error,
                ));
            }
        };
        let _sequence = sequence.lock();
        let requested = bytes.len() as u64;
        match super::exact_write_effect::execute(self.owner, &mut file, bytes) {
            super::exact_write_effect::ExactWriteEffect::DeniedBeforeEffect(failure) => {
                ArtifactRangeWriteOutcome::DeniedBeforeEffect(failure)
            }
            super::exact_write_effect::ExactWriteEffect::Indeterminate {
                failure,
                completed_bytes,
                operation,
            } => ArtifactRangeWriteOutcome::Indeterminate(IndeterminateArtifactRangeWrite::new(
                failure,
                coordinate,
                completed_bytes,
                operation,
            )),
            super::exact_write_effect::ExactWriteEffect::Completed(operation) => {
                let durability = match durability {
                    ArtifactRangeWriteDurabilityRequirement::BufferedWrite => {
                        ArtifactRangeWriteDurability::BufferedWriteCompleted
                    }
                    ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization => {
                        if let Err(failure) =
                            super::super::artifact_tree_effects::synchronize_file(self.owner, &file)
                        {
                            return ArtifactRangeWriteOutcome::Indeterminate(
                                IndeterminateArtifactRangeWrite::new(
                                    failure, coordinate, requested, operation,
                                ),
                            );
                        }
                        ArtifactRangeWriteDurability::FileDataSynchronized
                    }
                };
                let mut receipt = CompletedArtifactRangeWrite::buffered(
                    self.owner.identity(),
                    self.store,
                    coordinate,
                    bytes,
                    operation,
                );
                receipt.durability = durability;
                ArtifactRangeWriteOutcome::Completed(receipt)
            }
        }
    }
}
