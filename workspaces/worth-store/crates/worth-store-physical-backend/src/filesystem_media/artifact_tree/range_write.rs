use std::io::{Seek, SeekFrom};

use worth_store_physical_format::RecordFrameCoordinate;

use super::{
    ArtifactRangeWriteDurability, ArtifactRangeWriteDurabilityRequirement,
    ArtifactRangeWriteOutcome, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile,
    ArtifactTreeMedia, CompletedArtifactRangeWrite, CompletedScheduledArtifactRangeWrite,
    IndeterminateArtifactRangeWrite, ScheduledArtifactRangeWriteOutcome,
};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionPlanBinding, BackendQueueSpeculativeScope,
};

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
        self.write_scheduled_exact_at_with_posture(
            artifact,
            coordinate,
            bytes,
            binding,
            adaptation,
            Some(scope),
            durability,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn write_scheduled_foreground_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> ScheduledArtifactRangeWriteOutcome {
        self.write_scheduled_exact_at_with_posture(
            artifact, coordinate, bytes, binding, adaptation, None, durability, false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn append_scheduled_foreground_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> ScheduledArtifactRangeWriteOutcome {
        self.write_scheduled_exact_at_with_posture(
            artifact, coordinate, bytes, binding, adaptation, None, durability, true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn write_scheduled_exact_at_with_posture(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        writeback_scope: Option<BackendQueueSpeculativeScope>,
        durability: ArtifactRangeWriteDurabilityRequirement,
        extend_at_eof: bool,
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
        match self.write_exact_at_with_durability(
            artifact,
            coordinate,
            bytes,
            durability,
            extend_at_eof,
        ) {
            ArtifactRangeWriteOutcome::Completed(physical) => {
                let queue = ticket.begin_completion().observe_queue_depth(1);
                let queue = match writeback_scope {
                    Some(scope) => queue.observe_write_back(1, scope),
                    None => queue,
                }
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
            false,
        )
    }

    pub fn append_exact_at_eof(
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
            true,
        )
    }

    fn write_exact_at_with_durability(
        &self,
        artifact: &ArtifactTreeFile,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        durability: ArtifactRangeWriteDurabilityRequirement,
        extend_at_eof: bool,
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
            Ok(length) if !extend_at_eof && end <= length => {}
            Ok(length) if extend_at_eof && coordinate.offset() == length => {}
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
                self.owner.identity(),
                self.store,
                coordinate,
                bytes,
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
                            super::super::artifact_tree_effects::synchronize_file_for_operation(
                                self.owner, &file, operation,
                            )
                        {
                            return ArtifactRangeWriteOutcome::Indeterminate(
                                IndeterminateArtifactRangeWrite::new(
                                    failure,
                                    self.owner.identity(),
                                    self.store,
                                    coordinate,
                                    bytes,
                                    requested,
                                    operation,
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
                receipt.set_durability(durability);
                ArtifactRangeWriteOutcome::Completed(receipt)
            }
        }
    }
}
