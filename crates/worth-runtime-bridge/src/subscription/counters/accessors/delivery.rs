use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_delivery_cost_profile_selection_count(&self) -> usize {
        self.values
            .subscription_delivery_cost_profile_selection_count
    }

    pub fn subscription_delivery_cost_profile_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_cost_profile_rejection_count
    }

    pub fn subscription_delivery_over_budget_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_over_budget_rejection_count
    }

    pub fn subscription_delivery_density_sparse_count(&self) -> usize {
        self.values.subscription_delivery_density_sparse_count
    }

    pub fn subscription_delivery_density_coalesced_count(&self) -> usize {
        self.values.subscription_delivery_density_coalesced_count
    }

    pub fn subscription_delivery_density_dense_restart_count(&self) -> usize {
        self.values
            .subscription_delivery_density_dense_restart_count
    }

    pub fn subscription_consumer_contract_admission_count(&self) -> usize {
        self.values.subscription_consumer_contract_admission_count
    }

    pub fn subscription_consumer_contract_rejection_count(&self) -> usize {
        self.values.subscription_consumer_contract_rejection_count
    }

    pub fn subscription_activation_count(&self) -> usize {
        self.values.subscription_activation_count
    }

    pub fn subscription_delivery_record_count(&self) -> usize {
        self.values.subscription_delivery_record_count
    }

    pub fn subscription_delivery_member_count(&self) -> usize {
        self.values.subscription_delivery_member_count
    }

    pub fn subscription_delivery_family_selection_count(&self) -> usize {
        self.values.subscription_delivery_family_selection_count
    }

    pub fn subscription_delivery_arena_reset_count(&self) -> usize {
        self.values.subscription_delivery_arena_reset_count
    }

    pub fn subscription_delivery_buffer_reuse_count(&self) -> usize {
        self.values.subscription_delivery_buffer_reuse_count
    }

    pub fn subscription_allocation_count(&self) -> usize {
        self.values.subscription_allocation_count
    }

    pub fn subscription_clone_count(&self) -> usize {
        self.values.subscription_clone_count
    }

    pub fn subscription_callback_identity_scan_count(&self) -> usize {
        self.values.subscription_callback_identity_scan_count
    }

    pub fn subscription_active_registry_scan_count(&self) -> usize {
        self.values.subscription_active_registry_scan_count
    }

    pub fn subscription_fanout_per_member_consumer_scan_count(&self) -> usize {
        self.values
            .subscription_fanout_per_member_consumer_scan_count
    }

    pub fn subscription_delivery_window_seed_retention_count(&self) -> usize {
        self.values
            .subscription_delivery_window_seed_retention_count
    }

    pub fn subscription_delivery_replay_seed_retention_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_seed_retention_count
    }

    pub fn subscription_delivery_replay_readiness_inspection_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_readiness_inspection_count
    }

    pub fn subscription_delivery_replay_plan_count(&self) -> usize {
        self.values.subscription_delivery_replay_plan_count
    }

    pub fn subscription_delivery_replay_plan_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_plan_rejection_count
    }

    pub fn subscription_delivery_replay_retained_window_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_retained_window_count
    }

    pub fn subscription_delivery_replay_retained_member_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_retained_member_count
    }
}
