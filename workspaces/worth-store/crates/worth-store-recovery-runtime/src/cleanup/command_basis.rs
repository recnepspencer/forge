use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupCompactionBasis, PhysicalRecoveryCleanupPublicationBasis,
    PhysicalRecoveryCleanupRemovalCommand, PhysicalRecoveryCleanupWalBasis,
};
use worth_store_recovery_physics::LogSequenceNumber;

use crate::progression::ReopenedPhysicalRecovery;

use super::{RecoveryCleanupEligibility, RecoveryCleanupPlan};

pub(super) struct RecoveryCleanupCommandBasis {
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
}

impl RecoveryCleanupCommandBasis {
    pub(super) fn from_reopened(reopened: &ReopenedPhysicalRecovery) -> Option<Self> {
        let checkpoint = reopened.state.selection.checkpoint()?;
        let stream = checkpoint.checkpoint();
        Some(Self {
            compaction_generation: stream.compaction_cutover().product_generation(),
            compaction_digest: stream.footer().binding_records_digest(),
            retained_boundary: LogSequenceNumber::new(checkpoint.wal_tail_begin_lsn()),
        })
    }

    pub(super) fn command(
        &self,
        plan: &RecoveryCleanupPlan,
        candidate: RecoveryCleanupEligibility,
    ) -> Option<PhysicalRecoveryCleanupRemovalCommand> {
        PhysicalRecoveryCleanupRemovalCommand::new(
            PhysicalRecoveryCleanupPublicationBasis::new(
                plan.identity(),
                plan.published_generation(),
                plan.checkpoint(),
            ),
            PhysicalRecoveryCleanupCompactionBasis::new(
                self.compaction_generation,
                self.compaction_digest,
                self.retained_boundary,
            ),
            PhysicalRecoveryCleanupWalBasis::new(
                candidate.artifact(),
                candidate.range(),
                candidate.byte_count(),
            ),
        )
    }
}
