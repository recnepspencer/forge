use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_mixed_cause_ordering_request_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordering_request_count
    }

    pub fn subscription_mixed_cause_ordering_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordering_count
    }

    pub fn subscription_mixed_cause_ordered_cause_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordered_cause_count
    }

    pub fn subscription_mixed_cause_duplicate_suppression_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_duplicate_suppression_count
    }

    pub fn subscription_mixed_cause_denied_cause_count(&self) -> usize {
        self.values.subscription_mixed_cause_denied_cause_count
    }

    pub fn subscription_mixed_cause_authoritative_preview_rejection_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_authoritative_preview_rejection_count
    }

    pub fn subscription_mixed_cause_delivery_window_plan_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_delivery_window_plan_count
    }
}
