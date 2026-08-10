use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_continuation_index_build_count(&self) -> usize {
        self.values.subscription_continuation_index_build_count
    }

    pub fn subscription_continuation_candidate_count(&self) -> usize {
        self.values.subscription_continuation_candidate_count
    }

    pub fn subscription_continuation_candidate_index_lookup_count(&self) -> usize {
        self.values
            .subscription_continuation_candidate_index_lookup_count
    }

    pub fn subscription_continuation_decision_count(&self) -> usize {
        self.values.subscription_continuation_decision_count
    }

    pub fn subscription_continuation_rejection_count(&self) -> usize {
        self.values.subscription_continuation_rejection_count
    }

    pub fn subscription_branch_leak_rejection_count(&self) -> usize {
        self.values.subscription_branch_leak_rejection_count
    }

    pub fn subscription_continuation_child_record_count(&self) -> usize {
        self.values.subscription_continuation_child_record_count
    }

    pub fn subscription_continuation_full_registry_scan_count(&self) -> usize {
        self.values
            .subscription_continuation_full_registry_scan_count
    }
}
