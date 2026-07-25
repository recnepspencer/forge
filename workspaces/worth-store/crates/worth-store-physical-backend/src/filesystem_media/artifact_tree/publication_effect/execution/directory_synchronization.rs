use super::denied;
use crate::filesystem_media::artifact_tree::publication_effect::{
    ArtifactTreePublicationEffect, ArtifactTreePublicationEffectOutcome,
    ScheduledArtifactTreePublicationEffectOutcome,
};
use crate::filesystem_media::artifact_tree::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeMedia,
};
use crate::filesystem_media::artifact_tree_effects::begin_identified;
use crate::{
    filesystem_media::MediaOperationRole, BackendQueueExecutionAdaptation,
    BackendQueueExecutionPlanBinding,
};

impl ArtifactTreeMedia<'_> {
    pub fn synchronize_scheduled_directory(
        &self,
        directory: &ArtifactTreeDirectory,
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> ScheduledArtifactTreePublicationEffectOutcome {
        self.schedule_effect(binding, adaptation, |media| {
            media.synchronize_directory_observed(directory)
        })
    }

    pub fn synchronize_directory_effect(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> ArtifactTreePublicationEffectOutcome {
        self.synchronize_directory_observed(directory)
    }

    pub(in crate::filesystem_media::artifact_tree) fn synchronize_directory_observed(
        &self,
        directory: &ArtifactTreeDirectory,
    ) -> ArtifactTreePublicationEffectOutcome {
        let effect = ArtifactTreePublicationEffect::DirectorySynchronization(directory.clone());
        let _coordination = match self
            .owner
            .begin_artifact_namespace_mutation(vec![directory.coordination_key()])
        {
            Ok(coordination) => coordination,
            Err(_) => return denied(),
        };
        let opened = if directory.components.is_empty() {
            None
        } else {
            match self.open_directory(directory) {
                Ok(directory) => Some(directory),
                Err(failure) => {
                    return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure);
                }
            }
        };
        let opened = opened.as_ref().unwrap_or_else(|| self.root(directory.root));
        let Some((operation, attempt)) = begin_identified(
            self.owner,
            MediaOperationRole::SynchronizeDirectoryPublication,
            0,
        ) else {
            return denied();
        };
        if let Some(error) = attempt
            .fail_before_error()
            .or_else(|| attempt.barrier_error())
        {
            attempt.denied();
            return ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error),
            );
        }
        match crate::filesystem_media::directory_synchronization::synchronize_directory_handle(
            opened,
        ) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(0);
                self.indeterminate(effect, operation, None)
            }
            Ok(()) => {
                self.owner.boundary().counters().directory_sync();
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
