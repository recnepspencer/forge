use forge_store_physical_backend::{StoreDurabilityCounterSnapshot, StoreDurabilityState};
use forge_store_recovery_physics::{
    DurabilityReplayIdentity, DurableCheckpointPublication, DurableManifestPublication,
    DurableWalPublication,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6FlushDurabilityEvidenceRow {
    replay: DurabilityReplayIdentity,
    required_state: StoreDurabilityState,
    counters: StoreDurabilityCounterSnapshot,
}

impl S6FlushDurabilityEvidenceRow {
    pub fn from_wal_publication(publication: &DurableWalPublication) -> Self {
        Self {
            replay: publication.replay_identity().clone(),
            required_state: publication.required_state(),
            counters: publication.counters(),
        }
    }

    pub fn from_checkpoint_publication(publication: &DurableCheckpointPublication) -> Self {
        Self {
            replay: publication.replay_identity().clone(),
            required_state: publication.required_state(),
            counters: publication.counters(),
        }
    }

    pub fn from_manifest_publication(publication: &DurableManifestPublication) -> Self {
        Self {
            replay: publication.replay_identity().clone(),
            required_state: StoreDurabilityState::OrderingBarrierDurable,
            counters: publication.counters(),
        }
    }

    pub fn replay_identity(&self) -> &DurabilityReplayIdentity {
        &self.replay
    }

    pub const fn required_state(&self) -> StoreDurabilityState {
        self.required_state
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }
}
