use super::{
    ArtifactTreePublicationEffect, ArtifactTreePublicationEffectOutcome,
    CompletedArtifactTreePublicationEffect, CompletedScheduledArtifactTreePublicationEffect,
    IndeterminateArtifactTreePublicationEffect, ScheduledArtifactTreePublicationEffectOutcome,
};
use crate::filesystem_media::artifact_tree::{
    ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeMedia,
};
use crate::{
    filesystem_media::MediaOperationIdentity, BackendQueueExecutionAdaptation,
    BackendQueueExecutionPlanBinding,
};

mod directory_synchronization;
mod durable_removal;
mod file_synchronization;
mod replacement;
mod root_protocol_replacement;

impl ArtifactTreeMedia<'_> {
    fn schedule_effect(
        &self,
        binding: BackendQueueExecutionPlanBinding,
        adaptation: BackendQueueExecutionAdaptation,
        effect: impl FnOnce(&Self) -> ArtifactTreePublicationEffectOutcome,
    ) -> ScheduledArtifactTreePublicationEffectOutcome {
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            self.execution_capability,
            adaptation,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(
                    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect),
                );
            }
        };
        match effect(self) {
            ArtifactTreePublicationEffectOutcome::Completed(physical) => {
                ScheduledArtifactTreePublicationEffectOutcome::Completed(Box::new(
                    CompletedScheduledArtifactTreePublicationEffect {
                        physical,
                        queue: ticket.begin_completion().observe_queue_depth(1).complete(),
                    },
                ))
            }
            ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
                ScheduledArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure)
            }
            ArtifactTreePublicationEffectOutcome::Indeterminate(failure) => {
                ScheduledArtifactTreePublicationEffectOutcome::Indeterminate(failure)
            }
        }
    }

    fn completed(
        &self,
        effect: ArtifactTreePublicationEffect,
        operation: MediaOperationIdentity,
    ) -> ArtifactTreePublicationEffectOutcome {
        ArtifactTreePublicationEffectOutcome::Completed(CompletedArtifactTreePublicationEffect {
            owner: self.owner.identity(),
            store: self.store,
            operation,
            effect,
        })
    }

    fn indeterminate(
        &self,
        effect: ArtifactTreePublicationEffect,
        operation: MediaOperationIdentity,
        error: Option<&std::io::Error>,
    ) -> ArtifactTreePublicationEffectOutcome {
        let failure = error.map_or_else(
            || ArtifactTreeFailure::structural(ArtifactTreeFailureKind::IndeterminateEffect),
            |error| ArtifactTreeFailure::io(ArtifactTreeFailureKind::IndeterminateEffect, error),
        );
        ArtifactTreePublicationEffectOutcome::Indeterminate(
            IndeterminateArtifactTreePublicationEffect {
                failure,
                owner: self.owner.identity(),
                store: self.store,
                operation,
                effect,
            },
        )
    }
}

fn denied() -> ArtifactTreePublicationEffectOutcome {
    ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(ArtifactTreeFailure::structural(
        ArtifactTreeFailureKind::DeniedBeforeEffect,
    ))
}
