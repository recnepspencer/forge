use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn admitted_subscription_count(&self) -> usize {
        self.values.admitted_subscription_count
    }

    pub fn lifecycle_record_count(&self) -> usize {
        self.values.lifecycle_record_count
    }

    pub fn replay_reconstruction_count(&self) -> usize {
        self.values.replay_reconstruction_count
    }

    pub fn replay_mismatch_count(&self) -> usize {
        self.values.replay_mismatch_count
    }
}
