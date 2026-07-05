use forge_store_physical_backend::{StoreDurabilityBoundaryReached, StoreDurabilityState};
use forge_store_wal::CheckpointDurablePublicationScope;

use super::{DurabilityReplayIdentity, DurableCheckpointPublication, DurableWalPublication};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurabilityRecoveryReplaySource {
    WalFrame,
    Checkpoint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointCrashDurabilityPosture {
    FullyDurable,
    BoundaryReachedWithoutNamespaceOrRename,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurabilityRecoverySourcePrecedence {
    selected_source: DurabilityRecoveryReplaySource,
    checkpoint_posture: CheckpointCrashDurabilityPosture,
    selected_identity: DurabilityReplayIdentity,
}

impl DurabilityRecoverySourcePrecedence {
    pub fn after_fully_durable_checkpoint(
        wal: &DurableWalPublication,
        checkpoint: &DurableCheckpointPublication,
    ) -> Self {
        if checkpoint.replay_identity().last_lsn() >= wal.replay_identity().last_lsn() {
            Self {
                selected_source: DurabilityRecoveryReplaySource::Checkpoint,
                checkpoint_posture: CheckpointCrashDurabilityPosture::FullyDurable,
                selected_identity: checkpoint.replay_identity().clone(),
            }
        } else {
            Self {
                selected_source: DurabilityRecoveryReplaySource::WalFrame,
                checkpoint_posture: CheckpointCrashDurabilityPosture::FullyDurable,
                selected_identity: wal.replay_identity().clone(),
            }
        }
    }

    pub fn after_incomplete_checkpoint_namespace(
        wal: &DurableWalPublication,
        checkpoint_boundary: &StoreDurabilityBoundaryReached<CheckpointDurablePublicationScope>,
    ) -> Self {
        debug_assert_eq!(
            checkpoint_boundary.state(),
            StoreDurabilityState::WriteReachedDurabilityBoundary
        );
        Self {
            selected_source: DurabilityRecoveryReplaySource::WalFrame,
            checkpoint_posture:
                CheckpointCrashDurabilityPosture::BoundaryReachedWithoutNamespaceOrRename,
            selected_identity: wal.replay_identity().clone(),
        }
    }

    pub const fn selected_source(&self) -> DurabilityRecoveryReplaySource {
        self.selected_source
    }

    pub const fn checkpoint_posture(&self) -> CheckpointCrashDurabilityPosture {
        self.checkpoint_posture
    }

    pub const fn selected_identity(&self) -> &DurabilityReplayIdentity {
        &self.selected_identity
    }
}
