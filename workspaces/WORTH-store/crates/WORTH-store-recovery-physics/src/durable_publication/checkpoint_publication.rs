use worth_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};
use worth_store_wal::CheckpointDurablePublicationScope;

use super::DurabilityReplayIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurablePublicationDenialKind {
    WrongPublicationKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDurablePublicationDenial {
    kind: StoreDurablePublicationDenialKind,
    required: StoreDurabilityPublicationKind,
    actual: StoreDurabilityPublicationKind,
}

impl StoreDurablePublicationDenial {
    const fn wrong_kind(
        required: StoreDurabilityPublicationKind,
        actual: StoreDurabilityPublicationKind,
    ) -> Self {
        Self {
            kind: StoreDurablePublicationDenialKind::WrongPublicationKind,
            required,
            actual,
        }
    }

    pub const fn kind(&self) -> StoreDurablePublicationDenialKind {
        self.kind
    }

    pub const fn required(&self) -> StoreDurabilityPublicationKind {
        self.required
    }

    pub const fn actual(&self) -> StoreDurabilityPublicationKind {
        self.actual
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableCheckpointPublication {
    scope: CheckpointDurablePublicationScope,
    replay: DurabilityReplayIdentity,
    counters: StoreDurabilityCounterSnapshot,
}

impl DurableCheckpointPublication {
    pub fn publish(
        receipt: StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Result<Self, StoreDurablePublicationDenial> {
        if receipt.publication() != StoreDurabilityPublicationKind::Checkpoint {
            return Err(StoreDurablePublicationDenial::wrong_kind(
                StoreDurabilityPublicationKind::Checkpoint,
                receipt.publication(),
            ));
        }
        Ok(Self::from_receipt(receipt))
    }

    fn from_receipt(
        receipt: StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Self {
        let scope = receipt.scope().clone();
        let replay = DurabilityReplayIdentity::new(
            StoreDurabilityPublicationKind::Checkpoint,
            receipt.profile(),
            scope.manifest_digest(),
            scope.covered_lsn_start(),
            scope.covered_lsn_end(),
        );
        Self {
            scope,
            replay,
            counters: receipt.counters(),
        }
    }

    pub fn scope(&self) -> &CheckpointDurablePublicationScope {
        &self.scope
    }

    pub fn replay_identity(&self) -> &DurabilityReplayIdentity {
        &self.replay
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub const fn required_state(&self) -> StoreDurabilityState {
        StoreDurabilityState::OrderingBarrierDurable
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableManifestPublication {
    scope: CheckpointDurablePublicationScope,
    replay: DurabilityReplayIdentity,
    counters: StoreDurabilityCounterSnapshot,
}

impl DurableManifestPublication {
    pub fn publish(
        receipt: StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Result<Self, StoreDurablePublicationDenial> {
        if receipt.publication() != StoreDurabilityPublicationKind::Manifest {
            return Err(StoreDurablePublicationDenial::wrong_kind(
                StoreDurabilityPublicationKind::Manifest,
                receipt.publication(),
            ));
        }
        let scope = receipt.scope().clone();
        let replay = DurabilityReplayIdentity::new(
            StoreDurabilityPublicationKind::Manifest,
            receipt.profile(),
            scope.manifest_digest(),
            scope.covered_lsn_start(),
            scope.covered_lsn_end(),
        );
        Ok(Self {
            scope,
            replay,
            counters: receipt.counters(),
        })
    }

    pub fn replay_identity(&self) -> &DurabilityReplayIdentity {
        &self.replay
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub fn scope(&self) -> &CheckpointDurablePublicationScope {
        &self.scope
    }
}
