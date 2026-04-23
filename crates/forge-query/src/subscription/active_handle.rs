use super::active_digest::ActiveSubscriptionLaneDigest;

#[derive(Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneHandle {
    lane_digest: ActiveSubscriptionLaneDigest,
    lane_index: u64,
    registry_generation: u64,
}

impl ActiveSubscriptionLaneHandle {
    pub(super) fn new(
        lane_digest: ActiveSubscriptionLaneDigest,
        lane_index: u64,
        registry_generation: u64,
    ) -> Self {
        Self {
            lane_digest,
            lane_index,
            registry_generation,
        }
    }

    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn lane_index(&self) -> u64 {
        self.lane_index
    }

    pub fn registry_generation(&self) -> u64 {
        self.registry_generation
    }
}
