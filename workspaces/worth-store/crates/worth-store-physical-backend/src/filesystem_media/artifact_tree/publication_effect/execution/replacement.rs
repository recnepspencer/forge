use super::denied;
use crate::filesystem_media::artifact_tree::publication_effect::{
    ArtifactTreePublicationEffect, ArtifactTreePublicationEffectOutcome,
    ScheduledArtifactTreePublicationEffectOutcome,
};
use crate::filesystem_media::artifact_tree::{
    ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia,
};
use crate::filesystem_media::artifact_tree_effects::begin_identified;
use crate::filesystem_media::file_mutation_sequence::FileMutationSequence;
use crate::{
    filesystem_media::MediaOperationRole, BackendQueueExecutionAdaptation,
    BackendQueueExecutionPlanBinding,
};

impl ArtifactTreeMedia<'_> {
    pub fn replace_scheduled(
        &self,
        source: &ArtifactTreeFile,
        destination: &ArtifactTreeFile,
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactTreePublicationEffectOutcome {
        self.schedule_effect(binding, adaptation, |media| {
            media.replace_observed(source, destination)
        })
    }

    pub fn replace_effect(
        &self,
        source: &ArtifactTreeFile,
        destination: &ArtifactTreeFile,
    ) -> ArtifactTreePublicationEffectOutcome {
        self.replace_observed(source, destination)
    }

    pub(in crate::filesystem_media::artifact_tree) fn replace_observed(
        &self,
        source: &ArtifactTreeFile,
        destination: &ArtifactTreeFile,
    ) -> ArtifactTreePublicationEffectOutcome {
        let effect = ArtifactTreePublicationEffect::Replacement {
            source: source.clone(),
            destination: destination.clone(),
        };
        let _coordination = match self.owner.begin_artifact_namespace_mutation(vec![
            source.coordination_key(),
            destination.coordination_key(),
        ]) {
            Ok(coordination) => coordination,
            Err(_) => return denied(),
        };
        let source_directory = match self.open_directory(&source.directory) {
            Ok(directory) => directory,
            Err(failure) => {
                return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
            }
        };
        let destination_directory = match self.open_directory(&destination.directory) {
            Ok(directory) => directory,
            Err(failure) => {
                return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
            }
        };
        let source_file = match self.open_mutable_file(&source_directory, &source.file_name) {
            Ok(file) => file,
            Err(failure) => {
                return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
            }
        };
        let source_sequence = match mutation_sequence(self, &source_file) {
            Ok(sequence) => sequence,
            Err(failure) => {
                return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
            }
        };
        let destination_sequence =
            match self.open_mutable_file(&destination_directory, &destination.file_name) {
                Ok(file) => match mutation_sequence(self, &file) {
                    Ok(sequence) => Some(sequence),
                    Err(failure) => {
                        return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
                    }
                },
                Err(failure) if failure.kind() == ArtifactTreeFailureKind::Absent => None,
                Err(failure) => {
                    return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
                }
            };
        let Some((operation, attempt)) =
            begin_identified(self.owner, MediaOperationRole::AtomicReplace, 0)
        else {
            return denied();
        };
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error),
            );
        }
        let rename = || {
            source_directory.rename(
                &source.file_name,
                &destination_directory,
                &destination.file_name,
            )
        };
        match FileMutationSequence::with_ordered_pair(
            &source_sequence,
            destination_sequence.as_ref(),
            rename,
        ) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                self.owner.boundary().counters().replacement();
                attempt.indeterminate(0);
                self.indeterminate(effect, operation, None)
            }
            Ok(()) => {
                self.owner.boundary().counters().replacement();
                attempt.completed(0);
                self.completed(effect, operation)
            }
            Err(error) => {
                attempt.indeterminate(0);
                self.indeterminate(effect, operation, Some(&error))
            }
        }
    }
}

pub(super) fn mutation_sequence(
    media: &ArtifactTreeMedia<'_>,
    file: &cap_std::fs::File,
) -> Result<
    crate::filesystem_media::file_mutation_sequence::FileMutationSequence,
    ArtifactTreeFailure,
> {
    let sequence_file = file
        .try_clone()
        .map(cap_std::fs::File::into_std)
        .map_err(|error| {
            ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
        })?;
    media
        .owner
        .mutation_sequence_for(&sequence_file)
        .map_err(|error| {
            ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
        })
}
