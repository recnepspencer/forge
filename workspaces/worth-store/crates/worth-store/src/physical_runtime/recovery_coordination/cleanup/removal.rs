use worth_store_physical_backend::PhysicalRecoveryMediaGeneration;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalCheckpointIdentity,
};
use worth_store_wal::{LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity};

use crate::physical_runtime::PhysicalWorkSchedulerPosture;

use super::super::{PerformedRecoveryPhysicalEffect, RecoveryCleanupRemovalAction};

mod binding;
mod execution;
pub(super) use execution::execute;

pub(in crate::physical_runtime) struct PhysicalRecoveryCleanupRemovalCommand<'e> {
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
    selector_read: worth_store_physical_backend::CompletedScheduledRecoveryReopenRead,
    root_read: worth_store_physical_backend::CompletedScheduledRecoveryReopenRead,
    checkpoint_stream: &'e worth_store_physical_format::VerifiedCheckpointStream,
    verified_wal: worth_store_wal::VerifiedWalArtifact,
}

pub struct CompletedPhysicalRecoveryCleanupRemoval {
    performed: PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction>,
    revalidation: crate::physical_runtime::RecoveryCleanupArtifactRevalidationProgress,
}

pub struct PhysicalRecoveryCleanupRemovalDenial {
    kind: PhysicalRecoveryCleanupRemovalDenialKind,
    physical: Option<crate::physical_runtime::DeniedRecoveryCleanupPhysicalRemoval>,
    work: Option<crate::physical_runtime::PhysicalWorkIdentity>,
    scheduler: Option<PhysicalWorkSchedulerPosture>,
    signal: Option<crate::physical_runtime::PhysicalSignalSettlementOutcome>,
}

pub enum PhysicalRecoveryCleanupRemovalDenialKind {
    InvalidCommand,
    Admission(super::admission::PhysicalRecoveryCleanupAdmissionDenial),
    Execution(crate::physical_runtime::PhysicalWorkPreEffectDenial),
    Media(crate::physical_runtime::RecoveryCleanupRemovalDenialCause),
}

pub enum PhysicalRecoveryCleanupRemovalIndeterminate {
    Media {
        physical: crate::physical_runtime::IndeterminateRecoveryCleanupPhysicalRemoval,
        scheduler: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Scheduler {
        physical: crate::physical_runtime::CompletedRecoveryCleanupPhysicalRemoval,
        revalidation: crate::physical_runtime::RecoveryCleanupArtifactRevalidationProgress,
        posture: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Signal {
        physical: crate::physical_runtime::CompletedRecoveryCleanupPhysicalRemoval,
        revalidation: crate::physical_runtime::RecoveryCleanupArtifactRevalidationProgress,
        posture: PhysicalWorkSchedulerPosture,
        outcome: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
}

pub enum PhysicalRecoveryCleanupRemovalOutcome {
    Completed(CompletedPhysicalRecoveryCleanupRemoval),
    DeniedBeforeEffect(PhysicalRecoveryCleanupRemovalDenial),
    Indeterminate(PhysicalRecoveryCleanupRemovalIndeterminate),
}

impl<'e> PhysicalRecoveryCleanupRemovalCommand<'e> {
    pub(in crate::physical_runtime) fn from_freshness(
        basis: crate::physical_runtime::recovery_freshness::StoreRecoveryCleanupRemovalBasis,
        selector_read: worth_store_physical_backend::CompletedScheduledRecoveryReopenRead,
        checkpoint_stream: &'e worth_store_physical_format::VerifiedCheckpointStream,
        verified_wal: worth_store_wal::VerifiedWalArtifact,
    ) -> Self {
        let root_read = basis.root_read();
        Self {
            store: basis.store(),
            media_generation: basis.media_generation(),
            session: basis.session(),
            plan: basis.plan(),
            published_generation: basis.published_generation(),
            checkpoint: basis.checkpoint(),
            compaction_generation: basis.compaction_generation(),
            compaction_digest: basis.compaction_digest(),
            retained_boundary: basis.retained_boundary(),
            artifact: basis.artifact(),
            lsn_range: basis.lsn_range(),
            byte_count: basis.byte_count(),
            selector_read,
            root_read,
            checkpoint_stream,
            verified_wal,
        }
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
    ) -> crate::physical_runtime::RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl PhysicalRecoveryCleanupRemovalDenial {
    pub const fn kind(&self) -> &PhysicalRecoveryCleanupRemovalDenialKind {
        &self.kind
    }
    pub const fn physical(
        &self,
    ) -> Option<&crate::physical_runtime::DeniedRecoveryCleanupPhysicalRemoval> {
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
    ) -> crate::physical_runtime::RecoveryCleanupArtifactRevalidationProgress {
        match self {
            Self::Media { physical, .. } => physical.revalidation(),
            Self::Scheduler { revalidation, .. } | Self::Signal { revalidation, .. } => {
                *revalidation
            }
        }
    }
}
