use forge_store_physical_format::PhysicalReference;

use super::BTreeReplaySourceDenial;
use crate::{AdmittedRecoverySource, CheckpointId, WalLsnRange};

/// Recovery-owned proof that one checkpoint/WAL source names the exact B-tree root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTreeReplayRootAgreement {
    checkpoint_id: CheckpointId,
    root_reference: PhysicalReference,
    checkpoint_coverage: WalLsnRange,
    replay_tail: WalLsnRange,
}

impl BTreeReplayRootAgreement {
    pub(super) fn admit(
        source: &AdmittedRecoverySource,
        root_reference: PhysicalReference,
    ) -> Result<Self, BTreeReplaySourceDenial> {
        let AdmittedRecoverySource::CheckpointPlusWalTail {
            checkpoint,
            wal_tail,
            ..
        } = source
        else {
            return Err(match source {
                AdmittedRecoverySource::WalOnly { .. } => {
                    BTreeReplaySourceDenial::WalOnlyRootNotMaterialized
                }
                AdmittedRecoverySource::NoValidCheckpoint { .. } => {
                    BTreeReplaySourceDenial::NoAdmittedDurableSource
                }
                AdmittedRecoverySource::RecoveryBlocked { .. } => {
                    BTreeReplaySourceDenial::DurableSourceBlockedByIntegrity
                }
                AdmittedRecoverySource::CheckpointPlusWalTail { .. } => unreachable!(),
            });
        };

        if checkpoint.root_reference() != root_reference {
            return Err(BTreeReplaySourceDenial::CheckpointRootMismatch {
                expected: checkpoint.root_reference(),
                actual: root_reference,
            });
        }
        if wal_tail.checkpoint_id() != Some(checkpoint.checkpoint_id()) {
            return Err(BTreeReplaySourceDenial::CheckpointTailIdentityMismatch);
        }
        let checkpoint_coverage = checkpoint.covered_lsn_range();
        let replay_tail = wal_tail.lsn_range();
        if checkpoint_coverage.end_exclusive() != replay_tail.start() {
            return Err(BTreeReplaySourceDenial::CheckpointTailFrontierMismatch {
                checkpoint_end: checkpoint_coverage.end_exclusive().get(),
                tail_start: replay_tail.start().get(),
            });
        }

        Ok(Self {
            checkpoint_id: checkpoint.checkpoint_id().clone(),
            root_reference,
            checkpoint_coverage,
            replay_tail,
        })
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }

    pub const fn checkpoint_coverage(&self) -> WalLsnRange {
        self.checkpoint_coverage
    }

    pub const fn replay_tail(&self) -> WalLsnRange {
        self.replay_tail
    }
}
