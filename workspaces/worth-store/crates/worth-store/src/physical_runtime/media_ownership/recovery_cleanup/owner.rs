use std::num::NonZeroU64;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use cap_fs_ext::DirExt;
use worth_store_physical_backend::{
    ArtifactTreeFailure, ArtifactTreeFailureKind, BackendQueueExecutionCompletion,
    MediaOperationIdentity,
};
use worth_store_physical_format::store_namespace::{
    StableStoreIdentity, StoreNamespaceIdentityRecord, STORE_NAMESPACE_IDENTITY_RECORD_LENGTH,
};
use worth_store_physical_format::VerifiedCheckpointStream;
use worth_store_wal::{VerifiedWalArtifact, WalSegmentArtifactIdentity};

use super::{
    revalidation, CompletedRecoveryCleanupPhysicalRemoval, DeniedRecoveryCleanupPhysicalRemoval,
    IndeterminateRecoveryCleanupPhysicalRemoval, RecoveryCleanupArtifactRevalidationProgress,
    RecoveryCleanupRemovalDenialCause, RecoveryCleanupRemovalOutcome,
};

#[derive(Debug)]
pub(in crate::physical_runtime) struct RecoveryCleanupMediaOwner {
    root: cap_std::fs::Dir,
    next_operation: AtomicU64,
    #[cfg(feature = "certification-test-authority")]
    schedule: worth_store_physical_backend::MediaFaultSchedule,
}

impl RecoveryCleanupMediaOwner {
    pub(in crate::physical_runtime) fn open(root: &Path) -> Option<Self> {
        cap_std::fs::Dir::open_ambient_dir(root, cap_std::ambient_authority())
            .ok()
            .map(|root| Self {
                root,
                next_operation: AtomicU64::new(1),
                #[cfg(feature = "certification-test-authority")]
                schedule: worth_store_physical_backend::MediaFaultSchedule::default(),
            })
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn open_for_certification(
        root: &Path,
        schedule: worth_store_physical_backend::MediaFaultSchedule,
    ) -> Option<Self> {
        let mut owner = Self::open(root)?;
        owner.schedule = schedule;
        Some(owner)
    }

    pub(in crate::physical_runtime) fn matches_store(&self, expected: StableStoreIdentity) -> bool {
        self.read_store_identity()
            .is_some_and(|found| found == expected)
    }

    pub(in crate::physical_runtime) fn remove_wal(
        &self,
        expected_store: StableStoreIdentity,
        checkpoint: &VerifiedCheckpointStream,
        wal: &VerifiedWalArtifact,
        admission: [u8; 32],
        queue: BackendQueueExecutionCompletion,
    ) -> RecoveryCleanupRemovalOutcome {
        let inspection = wal.inspection();
        let artifact = inspection.identity();
        if !self.matches_store(expected_store) {
            return self.denied(
                artifact,
                admission,
                RecoveryCleanupRemovalDenialCause::Admission,
            );
        }
        if let Err(denial) = worth_store_wal::admit_checkpoint_covered_wal_cleanup(checkpoint, wal)
        {
            return self.denied(
                artifact,
                admission,
                RecoveryCleanupRemovalDenialCause::TerminalCoverage(denial),
            );
        }
        let directory = match self.wal_directory() {
            Ok(directory) => directory,
            Err(failure) => {
                return self.denied(
                    artifact,
                    admission,
                    RecoveryCleanupRemovalDenialCause::Preparation(failure),
                )
            }
        };
        let file_name = wal_file_name(artifact);
        let revalidation = match revalidation::verify(
            &directory,
            &file_name,
            inspection.byte_count(),
            inspection.artifact_digest(),
        ) {
            Ok(progress) => progress,
            Err(failure) => {
                return RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                    DeniedRecoveryCleanupPhysicalRemoval::new(
                        artifact,
                        admission,
                        RecoveryCleanupRemovalDenialCause::Revalidation(failure.denial()),
                        Some(queue),
                        failure.progress(),
                    ),
                ))
            }
        };
        self.remove_revalidated(
            directory,
            file_name,
            artifact,
            admission,
            queue,
            revalidation,
        )
    }

    fn remove_revalidated(
        &self,
        directory: cap_std::fs::Dir,
        file_name: String,
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> RecoveryCleanupRemovalOutcome {
        let operation = self.next_operation();
        #[cfg(feature = "certification-test-authority")]
        if let Some(outcome) = self.before_delete(artifact, admission, queue, revalidation) {
            return outcome;
        }
        if let Err(error) = directory.remove_file(&file_name) {
            return RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                DeniedRecoveryCleanupPhysicalRemoval::new(
                    artifact,
                    admission,
                    RecoveryCleanupRemovalDenialCause::Removal(io_failure(
                        ArtifactTreeFailureKind::DeniedBeforeEffect,
                        &error,
                    )),
                    Some(queue),
                    revalidation,
                ),
            ));
        }
        #[cfg(feature = "certification-test-authority")]
        if let Some(outcome) =
            self.after_delete(artifact, admission, operation, queue, revalidation)
        {
            return outcome;
        }
        if let Err(error) = synchronize_directory(&directory) {
            return self.indeterminate(
                artifact,
                admission,
                operation,
                io_failure(ArtifactTreeFailureKind::IndeterminateEffect, &error),
                queue,
                revalidation,
            );
        }
        RecoveryCleanupRemovalOutcome::Completed(Box::new(
            CompletedRecoveryCleanupPhysicalRemoval::new(
                artifact,
                admission,
                operation,
                queue,
                revalidation,
            ),
        ))
    }

    fn read_store_identity(&self) -> Option<StableStoreIdentity> {
        use std::io::Read;
        let namespace = self.root.open_dir_nofollow("namespace").ok()?;
        let mut identity = namespace.open("identity").ok()?;
        let mut bytes = Vec::with_capacity(STORE_NAMESPACE_IDENTITY_RECORD_LENGTH);
        identity
            .by_ref()
            .take(STORE_NAMESPACE_IDENTITY_RECORD_LENGTH as u64 + 1)
            .read_to_end(&mut bytes)
            .ok()?;
        StoreNamespaceIdentityRecord::decode(&bytes)
            .ok()
            .map(|record| record.published_identity())
    }

    fn wal_directory(&self) -> Result<cap_std::fs::Dir, ArtifactTreeFailure> {
        let families = self
            .root
            .open_dir_nofollow("families")
            .map_err(|error| io_failure(ArtifactTreeFailureKind::DeniedBeforeEffect, &error))?;
        families
            .open_dir_nofollow("wal")
            .map_err(|error| io_failure(ArtifactTreeFailureKind::DeniedBeforeEffect, &error))
    }

    fn next_operation(&self) -> MediaOperationIdentity {
        let value = self.next_operation.fetch_add(1, Ordering::Relaxed);
        MediaOperationIdentity::from_recovery_effect(
            NonZeroU64::new(value).expect("cleanup operation counter starts nonzero"),
        )
    }

    fn denied(
        &self,
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        cause: RecoveryCleanupRemovalDenialCause,
    ) -> RecoveryCleanupRemovalOutcome {
        RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
            DeniedRecoveryCleanupPhysicalRemoval::new(
                artifact,
                admission,
                cause,
                None,
                RecoveryCleanupArtifactRevalidationProgress::default(),
            ),
        ))
    }

    fn indeterminate(
        &self,
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        operation: MediaOperationIdentity,
        failure: ArtifactTreeFailure,
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> RecoveryCleanupRemovalOutcome {
        RecoveryCleanupRemovalOutcome::Indeterminate(Box::new(
            IndeterminateRecoveryCleanupPhysicalRemoval::new(
                artifact,
                admission,
                operation,
                failure,
                queue,
                revalidation,
            ),
        ))
    }

    #[cfg(feature = "certification-test-authority")]
    fn before_delete(
        &self,
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> Option<RecoveryCleanupRemovalOutcome> {
        use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};
        let directive = self
            .schedule
            .unbound_directive_for_certification(MediaOperationRole::Delete, 1)?;
        let error_kind = match directive {
            MediaFaultDirective::FailBefore { kind, .. } => Some(kind),
            MediaFaultDirective::PauseBeforeThenFailBefore { gate, kind, .. } => {
                gate.pause_for_certification();
                Some(kind)
            }
            MediaFaultDirective::PauseBefore(gate) => {
                gate.pause_for_certification();
                None
            }
            _ => None,
        };
        error_kind.map(|kind| {
            RecoveryCleanupRemovalOutcome::DeniedBeforeEffect(Box::new(
                DeniedRecoveryCleanupPhysicalRemoval::new(
                    artifact,
                    admission,
                    RecoveryCleanupRemovalDenialCause::Removal(ArtifactTreeFailure::recovery_io(
                        ArtifactTreeFailureKind::DeniedBeforeEffect,
                        kind,
                    )),
                    Some(queue),
                    revalidation,
                ),
            ))
        })
    }

    #[cfg(feature = "certification-test-authority")]
    fn after_delete(
        &self,
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        operation: MediaOperationIdentity,
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> Option<RecoveryCleanupRemovalOutcome> {
        use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};
        match self
            .schedule
            .unbound_directive_for_certification(MediaOperationRole::Delete, 1)?
        {
            MediaFaultDirective::PauseAfter(gate) => {
                gate.pause_for_certification();
                None
            }
            MediaFaultDirective::PanicAfter => panic!("certification panic after cleanup unlink"),
            MediaFaultDirective::IndeterminateAfterEffect => Some(self.indeterminate(
                artifact,
                admission,
                operation,
                ArtifactTreeFailure::recovery_io(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                    std::io::ErrorKind::Other,
                ),
                queue,
                revalidation,
            )),
            _ => None,
        }
    }
}

fn wal_file_name(artifact: WalSegmentArtifactIdentity) -> String {
    format!(
        "segment-{}-generation-{}.wal",
        artifact.segment().get(),
        artifact.generation().get()
    )
}

fn io_failure(kind: ArtifactTreeFailureKind, error: &std::io::Error) -> ArtifactTreeFailure {
    ArtifactTreeFailure::recovery_io(kind, error.kind())
}

#[cfg(windows)]
fn synchronize_directory(directory: &cap_std::fs::Dir) -> std::io::Result<()> {
    use cap_std::fs::OpenOptionsExt;
    let mut options = cap_std::fs::OpenOptions::new();
    options.read(true).write(true).custom_flags(0x0200_0000);
    directory.open_with(".", &options)?.into_std().sync_all()
}

#[cfg(not(windows))]
fn synchronize_directory(directory: &cap_std::fs::Dir) -> std::io::Result<()> {
    directory.try_clone()?.into_std_file().sync_all()
}
