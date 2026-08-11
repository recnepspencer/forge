use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_temporal_admission_count(&self) -> usize {
        self.values.subscription_temporal_admission_count
    }

    pub fn subscription_temporal_rejection_count(&self) -> usize {
        self.values.subscription_temporal_rejection_count
    }

    pub fn subscription_temporal_activation_ready_count(&self) -> usize {
        self.values.subscription_temporal_activation_ready_count
    }

    pub fn subscription_temporal_time_only_cause_count(&self) -> usize {
        self.values.subscription_temporal_time_only_cause_count
    }

    pub fn subscription_temporal_truth_plus_time_cause_count(&self) -> usize {
        self.values
            .subscription_temporal_truth_plus_time_cause_count
    }

    pub fn subscription_temporal_duplicate_clock_rejection_count(&self) -> usize {
        self.values
            .subscription_temporal_duplicate_clock_rejection_count
    }

    pub fn subscription_temporal_stale_clock_rejection_count(&self) -> usize {
        self.values
            .subscription_temporal_stale_clock_rejection_count
    }

    pub fn subscription_temporal_delivery_plan_count(&self) -> usize {
        self.values.subscription_temporal_delivery_plan_count
    }
}
