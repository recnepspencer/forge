use worth_store_physical_backend::PhysicalRecoveryMediaGeneration;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalCheckpointIdentity, VerifiedCheckpointStream,
};
use worth_store_wal::{
    LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity, WalSegmentInspection,
};

use crate::physical_runtime::{
    CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryCoordination,
    PhysicalWorkSchedulerPosture,
};

use super::super::{PerformedRecoveryPhysicalEffect, RecoveryCleanupRemovalAction};

mod execution;
pub(super) use execution::execute;

pub struct PhysicalRecoveryCleanupRemovalCommand {
    store: StableStoreIdentity,
    media_generation: PhysicalRecoveryMediaGeneration,
    session: [u8; 16],
    plan: [u8; 32],
    published_generation: u64,
    checkpoint: PhysicalCheckpointIdentity,
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
    artifact: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
    artifact_digest: [u8; 32],
}

pub struct CompletedPhysicalRecoveryCleanupRemoval {
    performed: PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction>,
    revalidation: worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress,
}

pub struct PhysicalRecoveryCleanupRemovalDenial {
    kind: PhysicalRecoveryCleanupRemovalDenialKind,
    physical: Option<worth_store_physical_backend::DeniedScheduledRecoveryCleanupRemoval>,
    work: Option<crate::physical_runtime::PhysicalWorkIdentity>,
    scheduler: Option<PhysicalWorkSchedulerPosture>,
    signal: Option<crate::physical_runtime::PhysicalSignalSettlementOutcome>,
}

pub enum PhysicalRecoveryCleanupRemovalDenialKind {
    InvalidCommand,
    Admission(super::admission::PhysicalRecoveryCleanupAdmissionDenial),
    Execution(crate::physical_runtime::PhysicalWorkPreEffectDenial),
    Media(worth_store_physical_backend::RecoveryCleanupRemovalDenialCause),
}

pub enum PhysicalRecoveryCleanupRemovalIndeterminate {
    Media {
        physical: worth_store_physical_backend::IndeterminateScheduledRecoveryCleanupRemoval,
        scheduler: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Scheduler {
        physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
        revalidation: worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress,
        posture: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Signal {
        physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
        revalidation: worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress,
        posture: PhysicalWorkSchedulerPosture,
        outcome: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
}

pub enum PhysicalRecoveryCleanupRemovalOutcome {
    Completed(CompletedPhysicalRecoveryCleanupRemoval),
    DeniedBeforeEffect(PhysicalRecoveryCleanupRemovalDenial),
    Indeterminate(PhysicalRecoveryCleanupRemovalIndeterminate),
}

impl PhysicalRecoveryCleanupRemovalCommand {
    /// Binds one Store-owned freshness decision to performed reopen and
    /// independently verified checkpoint/WAL facts.
    ///
    /// Shape-only coordinates cannot construct this command. The selected
    /// checkpoint must name the independently reopened root, and the complete
    /// WAL artifact must be wholly covered by that checkpoint.
    pub(in crate::physical_runtime) fn admit(
        media: &worth_store_physical_backend::AdmittedRecoveryFilesystemMedia,
        coordination: &PhysicalRecoveryCoordination,
        cleanup_plan_identity: [u8; 32],
        reopened: &CompletedPhysicalRecoveryFreshReopen,
        checkpoint: &VerifiedCheckpointStream,
        wal: WalSegmentInspection,
    ) -> Option<Self> {
        let occurrence = reopened.fresh_reopen_occurrence();
        let root = reopened.root();
        let source = checkpoint.source();
        let checkpoint_root = source.root();
        let retained_boundary = LogSequenceNumber::new(source.wal().covered_end_lsn_exclusive());
        let compaction = checkpoint.compaction_cutover();
        let store = source.identity().store_identity();
        let admissible = cleanup_plan_identity != [0; 32]
            && store == media.store_identity()
            && occurrence.session() == coordination.session_identity()
            && checkpoint_root.generation() <= root.generation()
            && checkpoint_root.tree_identity() == root.tree_identity()
            && wal.byte_count() != 0
            && wal.lsn_range().end_exclusive() <= retained_boundary;
        admissible.then_some(Self {
            store,
            media_generation: media.media_generation(),
            session: occurrence.session(),
            plan: cleanup_plan_identity,
            published_generation: occurrence.generation(),
            checkpoint: source.identity(),
            compaction_generation: compaction.product_generation(),
            compaction_digest: checkpoint.footer().binding_records_digest(),
            retained_boundary,
            artifact: wal.identity(),
            lsn_range: wal.lsn_range(),
            byte_count: wal.byte_count(),
            artifact_digest: wal.artifact_digest(),
        })
    }
}

impl CompletedPhysicalRecoveryCleanupRemoval {
    pub const fn performed(
        &self,
    ) -> &PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction> {
        &self.performed
    }
    pub fn into_performed(self) -> PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction> {
        self.performed
    }
    pub const fn revalidation(
        &self,
    ) -> worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl PhysicalRecoveryCleanupRemovalDenial {
    pub const fn kind(&self) -> &PhysicalRecoveryCleanupRemovalDenialKind {
        &self.kind
    }
    pub const fn physical(
        &self,
    ) -> Option<&worth_store_physical_backend::DeniedScheduledRecoveryCleanupRemoval> {
        self.physical.as_ref()
    }
    pub const fn work(&self) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.work
    }
    pub const fn scheduler(&self) -> Option<PhysicalWorkSchedulerPosture> {
        self.scheduler
    }
    pub const fn signal(&self) -> Option<crate::physical_runtime::PhysicalSignalSettlementOutcome> {
        self.signal
    }
}

impl PhysicalRecoveryCleanupRemovalIndeterminate {
    pub const fn revalidation(
        &self,
    ) -> worth_store_physical_backend::RecoveryCleanupArtifactRevalidationProgress {
        match self {
            Self::Media { physical, .. } => physical.revalidation(),
            Self::Scheduler { revalidation, .. } | Self::Signal { revalidation, .. } => {
                *revalidation
            }
        }
    }
}
