use forge_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};
use forge_store_wal::WalFrameDurablePublicationScope;

use super::DurabilityReplayIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableWalPublication {
    scope: WalFrameDurablePublicationScope,
    replay: DurabilityReplayIdentity,
    counters: StoreDurabilityCounterSnapshot,
}

impl DurableWalPublication {
    pub fn publish(
        receipt: StoreDurabilityOrderingBarrierDurable<WalFrameDurablePublicationScope>,
    ) -> Self {
        let scope = receipt.scope().clone();
        let replay = DurabilityReplayIdentity::new(
            StoreDurabilityPublicationKind::WalFrame,
            receipt.profile(),
            scope.frame_digest(),
            scope.lsn_start(),
            scope.lsn_end(),
        );
        Self {
            scope,
            replay,
            counters: receipt.counters(),
        }
    }

    pub fn scope(&self) -> &WalFrameDurablePublicationScope {
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
