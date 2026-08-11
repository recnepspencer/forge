use worth_store_physical_backend::{
    CompletedArtifactTreePublicationEffect, RecoveryWalArtifactCoordinate,
};
use worth_store_physical_format::PhysicalCheckpointIdentity;
use worth_store_wal::WalLsnRange;

pub struct RecoveryCleanupRemovalAction;
impl worth_proof::ActionMarker for RecoveryCleanupRemovalAction {}

pub(in crate::physical_runtime::recovery_coordination) struct RecoveryCleanupRemovalBinding {
    session: [u8; 16],
    plan: [u8; 32],
    published_generation: u64,
    checkpoint: PhysicalCheckpointIdentity,
}

pub(in crate::physical_runtime::recovery_coordination) struct RecoveryCleanupRemovalSettlement {
    physical: CompletedArtifactTreePublicationEffect,
    work: crate::physical_runtime::PhysicalWorkIdentity,
    scheduler: crate::physical_runtime::PhysicalWorkSchedulerPosture,
    signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

pub(in crate::physical_runtime::recovery_coordination) struct RecoveryCleanupRemovalTarget {
    artifact: RecoveryWalArtifactCoordinate,
    lsn_range: WalLsnRange,
    byte_count: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecoveryCleanupRemovalOccurrence {
    pub(super) session: [u8; 16],
    pub(super) plan: [u8; 32],
    pub(super) published_generation: u64,
    pub(super) checkpoint: PhysicalCheckpointIdentity,
    pub(super) artifact: RecoveryWalArtifactCoordinate,
    pub(super) lsn_range: WalLsnRange,
    pub(super) byte_count: u64,
    pub(super) physical: CompletedArtifactTreePublicationEffect,
    pub(super) work: crate::physical_runtime::PhysicalWorkIdentity,
    pub(super) scheduler: crate::physical_runtime::PhysicalWorkSchedulerPosture,
    pub(super) signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
}

impl RecoveryCleanupRemovalOccurrence {
    pub(in crate::physical_runtime::recovery_coordination) fn new(
        binding: RecoveryCleanupRemovalBinding,
        target: RecoveryCleanupRemovalTarget,
        settlement: RecoveryCleanupRemovalSettlement,
    ) -> Self {
        Self {
            session: binding.session,
            plan: binding.plan,
            published_generation: binding.published_generation,
            checkpoint: binding.checkpoint,
            artifact: target.artifact,
            lsn_range: target.lsn_range,
            byte_count: target.byte_count,
            physical: settlement.physical,
            work: settlement.work,
            scheduler: settlement.scheduler,
            signal: settlement.signal,
        }
    }

    pub const fn session(&self) -> [u8; 16] {
        self.session
    }
    pub const fn plan(&self) -> [u8; 32] {
        self.plan
    }
    pub const fn published_generation(&self) -> u64 {
        self.published_generation
    }
    pub const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }
    pub const fn artifact(&self) -> RecoveryWalArtifactCoordinate {
        self.artifact
    }
    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }
    pub const fn byte_count(&self) -> u64 {
        self.byte_count
    }
    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }
    pub const fn work(&self) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.work
    }
    pub const fn scheduler(&self) -> crate::physical_runtime::PhysicalWorkSchedulerPosture {
        self.scheduler
    }
    pub const fn signal(&self) -> crate::physical_runtime::PhysicalSignalSettlementOutcome {
        self.signal
    }
}

impl RecoveryCleanupRemovalTarget {
    pub(in crate::physical_runtime::recovery_coordination) const fn new(
        artifact: RecoveryWalArtifactCoordinate,
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

impl RecoveryCleanupRemovalBinding {
    pub(in crate::physical_runtime::recovery_coordination) const fn new(
        session: [u8; 16],
        plan: [u8; 32],
        published_generation: u64,
        checkpoint: PhysicalCheckpointIdentity,
    ) -> Self {
        Self {
            session,
            plan,
            published_generation,
            checkpoint,
        }
    }
}

impl RecoveryCleanupRemovalSettlement {
    pub(in crate::physical_runtime::recovery_coordination) fn new(
        physical: CompletedArtifactTreePublicationEffect,
        work: crate::physical_runtime::PhysicalWorkIdentity,
        scheduler: crate::physical_runtime::PhysicalWorkSchedulerPosture,
        signal: crate::physical_runtime::PhysicalSignalSettlementOutcome,
    ) -> Self {
        Self {
            physical,
            work,
            scheduler,
            signal,
        }
    }
}

impl super::PerformedRecoveryPhysicalEffect<RecoveryCleanupRemovalAction> {
    pub(in crate::physical_runtime::recovery_coordination) fn record_cleanup_removal(
        outcome: RecoveryCleanupRemovalOccurrence,
    ) -> Self {
        Self {
            evidence: worth_proof::Performed::record(
                &super::RecoveryPhysicalEffectAuthority::witness(),
                super::RecoveryPhysicalEffectOccurrence::CleanupRemoval(outcome),
            ),
            _action: std::marker::PhantomData,
        }
    }
}
