use super::AdmittedRecoveryFilesystemMedia;
use crate::filesystem_media::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreePublicationEffectOutcome,
    CompletedArtifactTreePublicationEffect, IndeterminateArtifactTreePublicationEffect,
};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};

mod revalidation;
pub use revalidation::{
    RecoveryCleanupArtifactRevalidationDenial, RecoveryCleanupArtifactRevalidationProgress,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWalArtifactCoordinate {
    segment: u64,
    generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledRecoveryCleanupRemoval {
    coordinate: RecoveryWalArtifactCoordinate,
    physical: CompletedArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedScheduledRecoveryCleanupRemoval {
    coordinate: RecoveryWalArtifactCoordinate,
    cause: RecoveryCleanupRemovalDenialCause,
    queue: Option<BackendQueueExecutionCompletion>,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryCleanupRemoval {
    coordinate: RecoveryWalArtifactCoordinate,
    physical: IndeterminateArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCleanupRemovalDenialCause {
    Preparation(ArtifactTreeFailure),
    Revalidation(RecoveryCleanupArtifactRevalidationDenial),
    Removal(ArtifactTreeFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryCleanupRemovalOutcome {
    Completed(Box<CompletedScheduledRecoveryCleanupRemoval>),
    DeniedBeforeEffect(Box<DeniedScheduledRecoveryCleanupRemoval>),
    Indeterminate(Box<IndeterminateScheduledRecoveryCleanupRemoval>),
}

impl RecoveryWalArtifactCoordinate {
    pub const fn new(segment: u64, generation: u64) -> Option<Self> {
        if segment == 0 || generation == 0 {
            None
        } else {
            Some(Self {
                segment,
                generation,
            })
        }
    }

    pub const fn segment(self) -> u64 {
        self.segment
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    fn file_name(self) -> String {
        format!(
            "segment-{}-generation-{}.wal",
            self.segment, self.generation
        )
    }
}

impl AdmittedRecoveryFilesystemMedia {
    pub fn remove_recovery_wal_artifact_scheduled(
        &self,
        coordinate: RecoveryWalArtifactCoordinate,
        expected_bytes: u64,
        expected_digest: [u8; 32],
        binding: BackendQueueExecutionPlanBinding,
    ) -> RecoveryCleanupRemovalOutcome {
        let wal = match ArtifactTreeDirectory::families().child("wal") {
            Ok(wal) => wal,
            Err(_) => return denied_without_queue(coordinate),
        };
        let artifact = match wal.file(&coordinate.file_name()) {
            Ok(artifact) => artifact,
            Err(_) => return denied_without_queue(coordinate),
        };
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            &self.parts.execution_capability,
            BackendQueueExecutionAdaptation::None,
        ) {
            Ok(ticket) => ticket,
            Err(_) => return denied_without_queue(coordinate),
        };
        let revalidation =
            match revalidation::verify(self, &artifact, expected_bytes, expected_digest) {
                Ok(progress) => progress,
                Err(failure) => {
                    return denied_with_queue(
                        coordinate,
                        RecoveryCleanupRemovalDenialCause::Revalidation(failure.denial()),
                        failure.progress(),
                        ticket.begin_completion().observe_queue_depth(1).complete(),
                    );
                }
            };
        let physical = self
            .parts
            .artifact_tree()
            .remove_file_durably_observed(&artifact);
        let queue = ticket.begin_completion().observe_queue_depth(1).complete();
        lower_cleanup_removal(coordinate, physical, queue, revalidation)
    }
}

fn lower_cleanup_removal(
    coordinate: RecoveryWalArtifactCoordinate,
    physical: ArtifactTreePublicationEffectOutcome,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
) -> RecoveryCleanupRemovalOutcome {
    match physical {
        ArtifactTreePublicationEffectOutcome::Completed(physical) => {
            RecoveryCleanupRemovalOutcome::Completed(Box::new(
                CompletedScheduledRecoveryCleanupRemoval {
                    coordinate,
                    physical,
                    queue,
                    revalidation,
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
            RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                DeniedScheduledRecoveryCleanupRemoval {
                    coordinate,
                    cause: RecoveryCleanupRemovalDenialCause::Removal(failure),
                    queue: Some(queue),
                    revalidation,
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::Indeterminate(physical) => {
            RecoveryCleanupRemovalOutcome::Indeterminate(Box::new(
                IndeterminateScheduledRecoveryCleanupRemoval {
                    coordinate,
                    physical,
                    queue,
                    revalidation,
                },
            ))
        }
    }
}

fn denied_with_queue(
    coordinate: RecoveryWalArtifactCoordinate,
    cause: RecoveryCleanupRemovalDenialCause,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
    queue: BackendQueueExecutionCompletion,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            coordinate,
            cause,
            queue: Some(queue),
            revalidation,
        },
    ))
}

fn denied_without_queue(
    coordinate: RecoveryWalArtifactCoordinate,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            coordinate,
            cause: RecoveryCleanupRemovalDenialCause::Preparation(
                ArtifactTreeFailure::recovery_denial(),
            ),
            queue: None,
            revalidation: RecoveryCleanupArtifactRevalidationProgress::default(),
        },
    ))
}

impl CompletedScheduledRecoveryCleanupRemoval {
    pub const fn coordinate(&self) -> RecoveryWalArtifactCoordinate {
        self.coordinate
    }
    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl DeniedScheduledRecoveryCleanupRemoval {
    pub const fn coordinate(&self) -> RecoveryWalArtifactCoordinate {
        self.coordinate
    }
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.cause.failure()
    }
    pub const fn cause(&self) -> RecoveryCleanupRemovalDenialCause {
        self.cause
    }
    pub const fn queue(&self) -> Option<BackendQueueExecutionCompletion> {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl IndeterminateScheduledRecoveryCleanupRemoval {
    pub const fn coordinate(&self) -> RecoveryWalArtifactCoordinate {
        self.coordinate
    }
    pub const fn physical(&self) -> &IndeterminateArtifactTreePublicationEffect {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl RecoveryCleanupRemovalDenialCause {
    pub const fn failure(self) -> ArtifactTreeFailure {
        match self {
            Self::Preparation(failure) | Self::Removal(failure) => failure,
            Self::Revalidation(_) => ArtifactTreeFailure::recovery_denial(),
        }
    }
}
