use super::AdmittedRecoveryFilesystemMedia;
use crate::filesystem_media::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreePublicationEffectOutcome,
    CompletedArtifactTreePublicationEffect, IndeterminateArtifactTreePublicationEffect,
};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};
use worth_store_physical_format::{
    DurableRootSelector, RecordArtifactFile, RootSelectorRole, VerifiedCheckpointStream,
};
use worth_store_wal::{VerifiedWalArtifact, WalSegmentArtifactIdentity};

mod revalidation;
pub use revalidation::{
    RecoveryCleanupArtifactRevalidationDenial, RecoveryCleanupArtifactRevalidationProgress,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
    physical: CompletedArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
    cause: RecoveryCleanupRemovalDenialCause,
    queue: Option<BackendQueueExecutionCompletion>,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
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

fn file_name(artifact: WalSegmentArtifactIdentity) -> String {
    format!(
        "segment-{}-generation-{}.wal",
        artifact.segment().get(),
        artifact.generation().get()
    )
}

impl AdmittedRecoveryFilesystemMedia {
    /// Removes only an exact selector/checkpoint/WAL basis that has already
    /// survived bounded decoding. Raw coordinates cannot enter this boundary:
    ///
    /// ```compile_fail
    /// use worth_store_physical_backend::{
    ///     AdmittedRecoveryFilesystemMedia, BackendQueueExecutionPlanBinding,
    ///     RecoveryWalArtifactCoordinate,
    /// };
    /// fn bypass(
    ///     media: &AdmittedRecoveryFilesystemMedia,
    ///     binding: BackendQueueExecutionPlanBinding,
    /// ) {
    ///     let coordinate = RecoveryWalArtifactCoordinate::new(7, 3).unwrap();
    ///     let _ = media.remove_recovery_wal_artifact_scheduled(
    ///         coordinate, 4096, [9; 32], binding,
    ///     );
    /// }
    /// ```
    pub fn remove_recovery_wal_artifact_scheduled(
        &self,
        selector_read: &super::CompletedScheduledRecoveryReopenRead,
        checkpoint: &VerifiedCheckpointStream,
        wal: &VerifiedWalArtifact,
        binding: BackendQueueExecutionPlanBinding,
    ) -> RecoveryCleanupRemovalOutcome {
        let inspection = wal.inspection();
        let artifact = inspection.identity();
        if !cleanup_facts_match(self, selector_read, checkpoint, wal) {
            return denied_without_queue(artifact);
        }
        let wal = match ArtifactTreeDirectory::families().child("wal") {
            Ok(wal) => wal,
            Err(_) => return denied_without_queue(artifact),
        };
        let physical = match wal.file(&file_name(artifact)) {
            Ok(physical) => physical,
            Err(_) => return denied_without_queue(artifact),
        };
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            &self.parts.execution_capability,
            BackendQueueExecutionAdaptation::None,
        ) {
            Ok(ticket) => ticket,
            Err(_) => return denied_without_queue(artifact),
        };
        let revalidation = match revalidation::verify(
            self,
            &physical,
            inspection.byte_count(),
            inspection.artifact_digest(),
        ) {
            Ok(progress) => progress,
            Err(failure) => {
                return denied_with_queue(
                    artifact,
                    RecoveryCleanupRemovalDenialCause::Revalidation(failure.denial()),
                    failure.progress(),
                    ticket.begin_completion().observe_queue_depth(1).complete(),
                );
            }
        };
        let physical = self
            .parts
            .artifact_tree()
            .remove_file_durably_observed(&physical);
        let queue = ticket.begin_completion().observe_queue_depth(1).complete();
        lower_cleanup_removal(artifact, physical, queue, revalidation)
    }
}

fn cleanup_facts_match(
    media: &AdmittedRecoveryFilesystemMedia,
    selector_read: &super::CompletedScheduledRecoveryReopenRead,
    checkpoint: &VerifiedCheckpointStream,
    wal: &VerifiedWalArtifact,
) -> bool {
    let Ok(selector) = DurableRootSelector::decode(selector_read.bytes()) else {
        return false;
    };
    let source = checkpoint.source();
    let inspection = wal.inspection();
    selector_read.artifact() == RecordArtifactFile::CurrentRootSelector
        && selector.store_identity() == media.store_identity()
        && selector.role() == RootSelectorRole::Current
        && selector.root_generation() >= source.root().generation()
        && inspection.byte_count() != 0
        && inspection.lsn_range().end_exclusive().get() <= source.wal().covered_end_lsn_exclusive()
}

fn lower_cleanup_removal(
    artifact: WalSegmentArtifactIdentity,
    physical: ArtifactTreePublicationEffectOutcome,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
) -> RecoveryCleanupRemovalOutcome {
    match physical {
        ArtifactTreePublicationEffectOutcome::Completed(physical) => {
            RecoveryCleanupRemovalOutcome::Completed(Box::new(
                CompletedScheduledRecoveryCleanupRemoval {
                    artifact,
                    physical,
                    queue,
                    revalidation,
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
            RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                DeniedScheduledRecoveryCleanupRemoval {
                    artifact,
                    cause: RecoveryCleanupRemovalDenialCause::Removal(failure),
                    queue: Some(queue),
                    revalidation,
                },
            ))
        }
        ArtifactTreePublicationEffectOutcome::Indeterminate(physical) => {
            RecoveryCleanupRemovalOutcome::Indeterminate(Box::new(
                IndeterminateScheduledRecoveryCleanupRemoval {
                    artifact,
                    physical,
                    queue,
                    revalidation,
                },
            ))
        }
    }
}

fn denied_with_queue(
    artifact: WalSegmentArtifactIdentity,
    cause: RecoveryCleanupRemovalDenialCause,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
    queue: BackendQueueExecutionCompletion,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            artifact,
            cause,
            queue: Some(queue),
            revalidation,
        },
    ))
}

fn denied_without_queue(artifact: WalSegmentArtifactIdentity) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            artifact,
            cause: RecoveryCleanupRemovalDenialCause::Preparation(
                ArtifactTreeFailure::recovery_denial(),
            ),
            queue: None,
            revalidation: RecoveryCleanupArtifactRevalidationProgress::default(),
        },
    ))
}

impl CompletedScheduledRecoveryCleanupRemoval {
    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
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
    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
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
    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
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
