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
    DurablePhysicalRootManifest, DurableRootSelector, RecordArtifactFile, RootSelectorRole,
    VerifiedCheckpointStream,
};
use worth_store_wal::{VerifiedWalArtifact, WalSegmentArtifactIdentity};

mod revalidation;
pub use revalidation::{
    RecoveryCleanupArtifactRevalidationDenial, RecoveryCleanupArtifactRevalidationProgress,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    physical: CompletedArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    cause: RecoveryCleanupRemovalDenialCause,
    queue: Option<BackendQueueExecutionCompletion>,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryCleanupRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    physical: IndeterminateArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCleanupRemovalDenialCause {
    Admission,
    TerminalCoverage(worth_store_wal::CheckpointCoveredWalCleanupDenial),
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
        root_read: &super::CompletedScheduledRecoveryReopenRead,
        checkpoint: &VerifiedCheckpointStream,
        wal: &VerifiedWalArtifact,
        effect_binding: worth_store_authority::RecoveryCleanupEffectBinding,
        binding: BackendQueueExecutionPlanBinding,
    ) -> RecoveryCleanupRemovalOutcome {
        let inspection = wal.inspection();
        let artifact = inspection.identity();
        let admission_identity = effect_binding.identity();
        if !cleanup_facts_match(
            self,
            selector_read,
            root_read,
            checkpoint,
            wal,
            effect_binding,
        ) {
            return denied_without_queue(
                artifact,
                admission_identity,
                RecoveryCleanupRemovalDenialCause::Admission,
            );
        }
        if let Err(denial) = worth_store_wal::admit_checkpoint_covered_wal_cleanup(checkpoint, wal)
        {
            return denied_without_queue(
                artifact,
                admission_identity,
                RecoveryCleanupRemovalDenialCause::TerminalCoverage(denial),
            );
        }
        let wal = match ArtifactTreeDirectory::families().child("wal") {
            Ok(wal) => wal,
            Err(_) => {
                return denied_without_queue(
                    artifact,
                    admission_identity,
                    RecoveryCleanupRemovalDenialCause::Preparation(
                        ArtifactTreeFailure::recovery_denial(),
                    ),
                )
            }
        };
        let physical = match wal.file(&file_name(artifact)) {
            Ok(physical) => physical,
            Err(_) => {
                return denied_without_queue(
                    artifact,
                    admission_identity,
                    RecoveryCleanupRemovalDenialCause::Preparation(
                        ArtifactTreeFailure::recovery_denial(),
                    ),
                )
            }
        };
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            &self.parts.execution_capability,
            BackendQueueExecutionAdaptation::None,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return denied_without_queue(
                    artifact,
                    admission_identity,
                    RecoveryCleanupRemovalDenialCause::Preparation(
                        ArtifactTreeFailure::recovery_denial(),
                    ),
                )
            }
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
                    admission_identity,
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
        lower_cleanup_removal(artifact, admission_identity, physical, queue, revalidation)
    }
}

fn cleanup_facts_match(
    media: &AdmittedRecoveryFilesystemMedia,
    selector_read: &super::CompletedScheduledRecoveryReopenRead,
    root_read: &super::CompletedScheduledRecoveryReopenRead,
    checkpoint: &VerifiedCheckpointStream,
    wal: &VerifiedWalArtifact,
    effect: worth_store_authority::RecoveryCleanupEffectBinding,
) -> bool {
    let Ok(selector) = DurableRootSelector::decode(selector_read.bytes()) else {
        return false;
    };
    let Ok((root, _)) = DurablePhysicalRootManifest::decode(root_read.bytes(), u16::MAX) else {
        return false;
    };
    let source = checkpoint.source();
    let inspection = wal.inspection();
    selector_read.artifact() == RecordArtifactFile::CurrentRootSelector
        && selector.store_identity() == media.store_identity()
        && selector.role() == RootSelectorRole::Current
        && selector.root_generation() >= source.root().generation()
        && root_read.artifact()
            == RecordArtifactFile::RootManifest {
                generation: selector.root_generation(),
            }
        && root.generation() == selector.root_generation()
        && root.tree_identity() == source.root().tree_identity()
        && inspection.byte_count() != 0
        && inspection.lsn_range().end_exclusive().get() <= source.wal().covered_end_lsn_exclusive()
        && effect.store() == media.store_identity().bytes()
        && effect.checkpoint_store() == source.identity().store_identity().bytes()
        && effect.checkpoint_sequence() == source.identity().sequence().get()
        && effect.artifact_segment() == inspection.identity().segment().get()
        && effect.artifact_generation() == inspection.identity().generation().get()
        && effect.lsn_start() == inspection.lsn_range().start().get()
        && effect.lsn_end_exclusive() == inspection.lsn_range().end_exclusive().get()
        && effect.byte_count() == inspection.byte_count()
        && effect.artifact_digest() == inspection.artifact_digest()
}

fn lower_cleanup_removal(
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    physical: ArtifactTreePublicationEffectOutcome,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
) -> RecoveryCleanupRemovalOutcome {
    match physical {
        ArtifactTreePublicationEffectOutcome::Completed(physical) => {
            RecoveryCleanupRemovalOutcome::Completed(Box::new(
                CompletedScheduledRecoveryCleanupRemoval {
                    artifact,
                    admission,
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
                    admission,
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
                    admission,
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
    admission: [u8; 32],
    cause: RecoveryCleanupRemovalDenialCause,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
    queue: BackendQueueExecutionCompletion,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            artifact,
            admission,
            cause,
            queue: Some(queue),
            revalidation,
        },
    ))
}

fn denied_without_queue(
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    cause: RecoveryCleanupRemovalDenialCause,
) -> RecoveryCleanupRemovalOutcome {
    RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
        DeniedScheduledRecoveryCleanupRemoval {
            artifact,
            admission,
            cause,
            queue: None,
            revalidation: RecoveryCleanupArtifactRevalidationProgress::default(),
        },
    ))
}

impl CompletedScheduledRecoveryCleanupRemoval {
    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
    }
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
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
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
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
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
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
            Self::Admission | Self::TerminalCoverage(_) | Self::Revalidation(_) => {
                ArtifactTreeFailure::recovery_denial()
            }
        }
    }
}
