use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_historical_truth_basis_admission_count(&self) -> usize {
        self.values
            .subscription_historical_truth_basis_admission_count
    }

    pub fn subscription_historical_previous_value_evidence_count(&self) -> usize {
        self.values
            .subscription_historical_previous_value_evidence_count
    }

    pub fn subscription_historical_temporal_replay_basis_admission_count(&self) -> usize {
        self.values
            .subscription_historical_temporal_replay_basis_admission_count
    }

    pub fn subscription_historical_temporal_replay_rejection_count(&self) -> usize {
        self.values
            .subscription_historical_temporal_replay_rejection_count
    }

    pub fn subscription_historical_temporal_readiness_count(&self) -> usize {
        self.values.subscription_historical_temporal_readiness_count
    }
}
