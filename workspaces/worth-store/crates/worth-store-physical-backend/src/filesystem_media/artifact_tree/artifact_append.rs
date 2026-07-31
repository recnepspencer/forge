use std::io::{Seek, SeekFrom};

use super::{
    ArtifactAppendOutcome, ArtifactAppendRange, ArtifactTreeFailure, ArtifactTreeFailureKind,
    ArtifactTreeFile, ArtifactTreeMedia, CompletedArtifactAppend, CompletedScheduledArtifactAppend,
    IndeterminateArtifactAppend, ScheduledArtifactAppendOutcome,
};
use crate::{BackendQueueExecutionAdaptation, BackendQueueExecutionPlanBinding};

impl ArtifactTreeMedia<'_> {
    /// Appends exact bytes at the current EOF under one admitted scheduler plan.
    ///
    /// This operation performs no durability barrier. Completion proves only
    /// that the complete requested bytes were observed written.
    pub fn append_scheduled_artifact_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        range: ArtifactAppendRange,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactAppendOutcome {
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return ScheduledArtifactAppendOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        match self.append_artifact_exact_at(artifact, range, bytes) {
            ArtifactAppendOutcome::Completed(physical) => {
                ScheduledArtifactAppendOutcome::Completed(Box::new(
                    CompletedScheduledArtifactAppend {
                        physical,
                        queue: ticket.begin_completion().observe_queue_depth(1).complete(),
                    },
                ))
            }
            ArtifactAppendOutcome::DeniedBeforeEffect(failure) => {
                ScheduledArtifactAppendOutcome::DeniedBeforeEffect(failure)
            }
            ArtifactAppendOutcome::Indeterminate(failure) => {
                ScheduledArtifactAppendOutcome::Indeterminate(failure)
            }
        }
    }

    pub fn append_artifact_exact_at(
        &self,
        artifact: &ArtifactTreeFile,
        range: ArtifactAppendRange,
        bytes: &[u8],
    ) -> ArtifactAppendOutcome {
        if range.byte_count() != bytes.len() as u64 {
            return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let _coordination = match self
            .owner
            .begin_artifact_mutation(vec![artifact.coordination_key()])
        {
            Ok(coordination) => coordination,
            Err(_) => {
                return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                ));
            }
        };
        let directory = match self.open_directory(&artifact.directory) {
            Ok(directory) => directory,
            Err(failure) => return ArtifactAppendOutcome::DeniedBeforeEffect(failure),
        };
        let mut file = match self.open_mutable_file(&directory, &artifact.file_name) {
            Ok(file) => file,
            Err(failure) => return ArtifactAppendOutcome::DeniedBeforeEffect(failure),
        };
        let length =
            match super::super::artifact_tree_effects::artifact_file_length(self.owner, &file) {
                Ok(length) => length,
                Err(failure) => return ArtifactAppendOutcome::DeniedBeforeEffect(failure),
            };
        if length != range.offset() {
            return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        if let Err(error) = file.seek(SeekFrom::Start(range.offset())) {
            return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        let sequence_file = match file.try_clone().map(cap_std::fs::File::into_std) {
            Ok(file) => file,
            Err(error) => {
                return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                    &error,
                ));
            }
        };
        let sequence = match self.owner.mutation_sequence_for(&sequence_file) {
            Ok(sequence) => sequence,
            Err(error) => {
                return ArtifactAppendOutcome::DeniedBeforeEffect(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::DeniedBeforeEffect,
                    &error,
                ));
            }
        };
        let _sequence = sequence.lock();
        match super::exact_write_effect::execute(self.owner, &mut file, bytes) {
            super::exact_write_effect::ExactWriteEffect::DeniedBeforeEffect(failure) => {
                ArtifactAppendOutcome::DeniedBeforeEffect(failure)
            }
            super::exact_write_effect::ExactWriteEffect::Indeterminate {
                failure,
                completed_bytes,
                operation,
            } => ArtifactAppendOutcome::Indeterminate(IndeterminateArtifactAppend::new(
                failure,
                self.owner.identity(),
                self.store,
                artifact.clone(),
                range,
                bytes,
                completed_bytes,
                operation,
            )),
            super::exact_write_effect::ExactWriteEffect::Completed(operation) => {
                ArtifactAppendOutcome::Completed(CompletedArtifactAppend::new(
                    self.owner.identity(),
                    self.store,
                    artifact.clone(),
                    range,
                    bytes,
                    operation,
                ))
            }
        }
    }
}
