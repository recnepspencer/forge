use super::active_digest::ActiveSubscriptionLaneDigest;
use super::future_selection::QuerySubscriptionFutureSelection;

#[derive(Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneHandle {
    lane_digest: ActiveSubscriptionLaneDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_digest: String,
    checkpoint_identity_digest: String,
    lane_index: u64,
    registry_generation: u64,
}

impl ActiveSubscriptionLaneHandle {
    pub(super) fn new(
        lane_digest: ActiveSubscriptionLaneDigest,
        future_selection: QuerySubscriptionFutureSelection,
        basis_binding_digest: String,
        checkpoint_identity_digest: String,
        lane_index: u64,
        registry_generation: u64,
    ) -> Self {
        Self {
            lane_digest,
            future_selection,
            basis_binding_digest,
            checkpoint_identity_digest,
            lane_index,
            registry_generation,
        }
    }

    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_identity_digest
    }

    pub fn lane_index(&self) -> u64 {
        self.lane_index
    }

    pub fn registry_generation(&self) -> u64 {
        self.registry_generation
    }
}
