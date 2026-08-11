use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_acknowledgement_frontier_admission_count(&self) -> usize {
        self.values
            .subscription_acknowledgement_frontier_admission_count
    }

    pub fn subscription_acknowledgement_frontier_rejection_count(&self) -> usize {
        self.values
            .subscription_acknowledgement_frontier_rejection_count
    }

    pub fn subscription_checkpoint_ready_count(&self) -> usize {
        self.values.subscription_checkpoint_ready_count
    }

    pub fn subscription_checkpoint_publication_count(&self) -> usize {
        self.values.subscription_checkpoint_publication_count
    }

    pub fn subscription_checkpoint_publication_rejection_count(&self) -> usize {
        self.values
            .subscription_checkpoint_publication_rejection_count
    }

    pub fn subscription_duplicate_replay_policy_selection_count(&self) -> usize {
        self.values
            .subscription_duplicate_replay_policy_selection_count
    }

    pub fn subscription_unsealed_stream_checkpoint_rejection_count(&self) -> usize {
        self.values
            .subscription_unsealed_stream_checkpoint_rejection_count
    }

    pub fn subscription_checkpoint_truncation_rejection_count(&self) -> usize {
        self.values
            .subscription_checkpoint_truncation_rejection_count
    }
}
