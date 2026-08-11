use crate::filesystem_media::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFile,
    ArtifactTreePublicationEffectOutcome, CompletedArtifactTreePublicationEffect,
    IndeterminateArtifactTreePublicationEffect,
};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};
use sha2::{Digest, Sha256};

use super::AdmittedRecoveryFilesystemMedia;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedScheduledRecoveryCleanupRemoval {
    coordinate: RecoveryWalArtifactCoordinate,
    failure: ArtifactTreeFailure,
    queue: Option<BackendQueueExecutionCompletion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryCleanupRemoval {
    coordinate: RecoveryWalArtifactCoordinate,
    physical: IndeterminateArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
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
        if let Err(failure) =
            self.verify_cleanup_artifact(&artifact, expected_bytes, expected_digest)
        {
            return denied_with_queue(
                coordinate,
                failure,
                ticket.begin_completion().observe_queue_depth(1).complete(),
            );
        }
        let physical = self
            .parts
            .artifact_tree()
            .remove_file_durably_observed(&artifact);
        let queue = ticket.begin_completion().observe_queue_depth(1).complete();
        lower_cleanup_removal(coordinate, physical, queue)
    }

    fn verify_cleanup_artifact(
        &self,
        artifact: &ArtifactTreeFile,
        expected_bytes: u64,
        expected_digest: [u8; 32],
    ) -> Result<(), ArtifactTreeFailure> {
        let observed = self
            .parts
            .artifact_tree()
            .read_bounded(artifact, expected_bytes)?;
        if observed.len() as u64 != expected_bytes
            || <[u8; 32]>::from(Sha256::digest(&observed)) != expected_digest
        {
            return Err(ArtifactTreeFailure::recovery_denial());
        }
        Ok(())
    }
}

fn lower_cleanup_removal(
    coordinate: RecoveryWalArtifactCoordinate,
    physical: ArtifactTreePublicationEffectOutcome,
    queue: BackendQueueExecutionCompletion,
) -> RecoveryCleanupRemovalOutcome {
    match physical {
        ArtifactTreePublicationEffectOutcome::Completed(physical) => {
            RecoveryCleanupRemovalOutcome::Completed(Box::new(
                CompletedScheduledRecoveryCleanupRemoval {
                    coordinate,
                    physical,
                    queue,
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
            RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                DeniedScheduledRecoveryCleanupRemoval {
                    coordinate,
                    failure,
                    queue: Some(queue),
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::Indeterminate(physical) => {
            RecoveryCleanupRemovalOutcome::Indeterminate(Box::new(
                IndeterminateScheduledRecoveryCleanupRemoval {
                    coordinate,
                    physical,
                    queue,
                },
            ))
        }
    }
}

fn denied_with_queue(
    coordinate: RecoveryWalArtifactCoordinate,
    failure: ArtifactTreeFailure,
    queue: BackendQueueExecutionCompletion,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            coordinate,
            failure,
            queue: Some(queue),
        },
    ))
}

fn denied_without_queue(
    coordinate: RecoveryWalArtifactCoordinate,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            coordinate,
            failure: ArtifactTreeFailure::recovery_denial(),
            queue: None,
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
}

impl DeniedScheduledRecoveryCleanupRemoval {
    pub const fn coordinate(&self) -> RecoveryWalArtifactCoordinate {
        self.coordinate
    }
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
    pub const fn queue(&self) -> Option<BackendQueueExecutionCompletion> {
        self.queue
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
}
