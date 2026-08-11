use super::replacement::mutation_sequence;
use crate::filesystem_media::artifact_tree::publication_effect::{
    ArtifactTreePublicationEffect, ArtifactTreePublicationEffectOutcome, ArtifactTreeReplacement,
    ScheduledArtifactTreePublicationEffectOutcome,
};
use crate::filesystem_media::artifact_tree::{ArtifactTreeFailure, ArtifactTreeFailureKind};
use crate::filesystem_media::file_mutation_sequence::FileMutationSequence;
use crate::{
    filesystem_media::{MediaOperationCoordinates, MediaOperationRole, MediaPathRole},
    ArtifactTreeMedia, BackendQueueExecutionAdaptation, BackendQueueExecutionPlanBinding,
};

impl ArtifactTreeMedia<'_> {
    pub fn replace_root_protocol_scheduled(
        &self,
        previous_selector: ArtifactTreeReplacement,
        current_selector: ArtifactTreeReplacement,
        bootstrap_catalog: ArtifactTreeReplacement,
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactTreePublicationEffectOutcome {
        self.schedule_effect(binding, adaptation, move |media| {
            media.replace_root_protocol(previous_selector, current_selector, bootstrap_catalog)
        })
    }

    fn replace_root_protocol(
        &self,
        previous_selector: ArtifactTreeReplacement,
        current_selector: ArtifactTreeReplacement,
        bootstrap_catalog: ArtifactTreeReplacement,
    ) -> ArtifactTreePublicationEffectOutcome {
        let effect = ArtifactTreePublicationEffect::RootProtocolReplacement {
            previous_selector: previous_selector.clone(),
            current_selector: current_selector.clone(),
            bootstrap_catalog: bootstrap_catalog.clone(),
        };
        let replacements = [previous_selector, current_selector, bootstrap_catalog];
        let keys = replacements
            .iter()
            .flat_map(|replacement| {
                [
                    replacement.source.coordination_key(),
                    replacement.destination.coordination_key(),
                ]
            })
            .collect();
        let _coordination = match self.owner.begin_artifact_namespace_mutation(keys) {
            Ok(coordination) => coordination,
            Err(_) => return denied(),
        };
        let prepared = match replacements
            .iter()
            .map(|replacement| prepare(self, replacement))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(prepared) => prepared,
            Err(failure) => {
                return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure)
            }
        };
        let Some(operation) = self.owner.issue_operation_identity() else {
            return denied();
        };
        for (index, replacement) in prepared.iter().enumerate() {
            let attempt = self.owner.boundary().begin_operation(
                MediaOperationRole::AtomicReplace,
                0,
                MediaOperationCoordinates::for_path(operation, MediaPathRole::ArtifactOwned, None),
            );
            if let Some(error) = attempt.fail_before_error() {
                attempt.denied();
                return if index == 0 {
                    ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(
                        ArtifactTreeFailure::io(
                            ArtifactTreeFailureKind::DeniedBeforeEffect,
                            &error,
                        ),
                    )
                } else {
                    self.indeterminate(effect, operation, Some(&error))
                };
            }
            if let Err(error) = replacement.execute() {
                attempt.indeterminate(0);
                return self.indeterminate(effect, operation, Some(&error));
            }
            self.owner.boundary().counters().replacement();
            if attempt.effect_observation_is_indeterminate() {
                attempt.indeterminate(0);
                return self.indeterminate(effect, operation, None);
            }
            attempt.completed(0);
        }
        self.completed(effect, operation)
    }
}

struct PreparedReplacement {
    source_directory: cap_std::fs::Dir,
    destination_directory: cap_std::fs::Dir,
    source_name: String,
    destination_name: String,
    source_sequence: FileMutationSequence,
    destination_sequence: Option<FileMutationSequence>,
}

impl PreparedReplacement {
    fn execute(&self) -> std::io::Result<()> {
        FileMutationSequence::with_ordered_pair(
            &self.source_sequence,
            self.destination_sequence.as_ref(),
            || {
                self.source_directory.rename(
                    &self.source_name,
                    &self.destination_directory,
                    &self.destination_name,
                )
            },
        )
    }
}

fn prepare(
    media: &ArtifactTreeMedia<'_>,
    replacement: &ArtifactTreeReplacement,
) -> Result<PreparedReplacement, ArtifactTreeFailure> {
    let source_directory = media.open_directory(&replacement.source.directory)?;
    let destination_directory = media.open_directory(&replacement.destination.directory)?;
    let source_file = media.open_mutable_file(&source_directory, &replacement.source.file_name)?;
    let source_sequence = mutation_sequence(media, &source_file)?;
    let destination_sequence =
        match media.open_mutable_file(&destination_directory, &replacement.destination.file_name) {
            Ok(file) => Some(mutation_sequence(media, &file)?),
            Err(failure) if failure.kind() == ArtifactTreeFailureKind::Absent => None,
            Err(failure) => return Err(failure),
        };
    Ok(PreparedReplacement {
        source_directory,
        destination_directory,
        source_name: replacement.source.file_name.clone(),
        destination_name: replacement.destination.file_name.clone(),
        source_sequence,
        destination_sequence,
    })
}

fn denied() -> ArtifactTreePublicationEffectOutcome {
    ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
        ArtifactTreeFailureKind::DeniedBeforeEffect,
    ))
}
