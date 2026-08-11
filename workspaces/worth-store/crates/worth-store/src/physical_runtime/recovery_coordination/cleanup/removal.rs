use worth_store_physical_format::PhysicalCheckpointIdentity;
use worth_store_wal::{LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity};

use crate::physical_runtime::PhysicalWorkSchedulerPosture;

use super::super::{PerformedRecoveryPhysicalEffect, RecoveryCleanupRemovalAction};

mod execution;
pub(super) use execution::execute;

pub struct PhysicalRecoveryCleanupRemovalCommand {
    plan: [u8; 32],
    published_generation: u64,
    checkpoint: PhysicalCheckpointIdentity,
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
    artifact: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
}

pub struct PhysicalRecoveryCleanupPublicationBasis {
    plan: [u8; 32],
    published_generation: u64,
    checkpoint: PhysicalCheckpointIdentity,
}

pub struct PhysicalRecoveryCleanupCompactionBasis {
    generation: u64,
    digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
}

pub struct PhysicalRecoveryCleanupWalBasis {
    artifact: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
}

pub struct CompletedPhysicalRecoveryCleanupRemoval {
    performed: PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction>,
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
    Media,
}

pub enum PhysicalRecoveryCleanupRemovalIndeterminate {
    Media {
        physical: worth_store_physical_backend::IndeterminateScheduledRecoveryCleanupRemoval,
        scheduler: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Scheduler {
        physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
        posture: PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    },
    Signal {
        physical: worth_store_physical_backend::CompletedArtifactTreePublicationEffect,
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
    pub fn new(
        publication: PhysicalRecoveryCleanupPublicationBasis,
        compaction: PhysicalRecoveryCleanupCompactionBasis,
        wal: PhysicalRecoveryCleanupWalBasis,
    ) -> Option<Self> {
        (publication.published_generation != 0
            && compaction.generation != 0
            && wal.byte_count != 0
            && wal.lsn_range.end_exclusive() <= compaction.retained_boundary)
            .then_some(Self {
                plan: publication.plan,
                published_generation: publication.published_generation,
                checkpoint: publication.checkpoint,
                compaction_generation: compaction.generation,
                compaction_digest: compaction.digest,
                retained_boundary: compaction.retained_boundary,
                artifact: wal.artifact,
                lsn_range: wal.lsn_range,
                byte_count: wal.byte_count,
            })
    }
}

impl PhysicalRecoveryCleanupPublicationBasis {
    pub const fn new(
        plan: [u8; 32],
        published_generation: u64,
        checkpoint: PhysicalCheckpointIdentity,
    ) -> Self {
        Self {
            plan,
            published_generation,
            checkpoint,
        }
    }
}

impl PhysicalRecoveryCleanupCompactionBasis {
    pub const fn new(
        generation: u64,
        digest: [u8; 32],
        retained_boundary: LogSequenceNumber,
    ) -> Self {
        Self {
            generation,
            digest,
            retained_boundary,
        }
    }
}

impl PhysicalRecoveryCleanupWalBasis {
    pub const fn new(
        artifact: WalSegmentArtifactIdentity,
        lsn_range: WalLsnRange,
        byte_count: u64,
    ) -> Self {
        Self {
            artifact,
            lsn_range,
            byte_count,
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
