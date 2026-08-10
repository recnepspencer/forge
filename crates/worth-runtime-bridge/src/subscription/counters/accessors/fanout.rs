use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_fanout_plan_admission_count(&self) -> usize {
        self.values.subscription_fanout_plan_admission_count
    }

    pub fn subscription_fanout_plan_rejection_count(&self) -> usize {
        self.values.subscription_fanout_plan_rejection_count
    }

    pub fn subscription_fanout_layout_build_count(&self) -> usize {
        self.values.subscription_fanout_layout_build_count
    }

    pub fn subscription_fanout_consumer_binding_count(&self) -> usize {
        self.values.subscription_fanout_consumer_binding_count
    }

    pub fn subscription_fanout_delivery_projection_count(&self) -> usize {
        self.values.subscription_fanout_delivery_projection_count
    }

    pub fn subscription_fanout_delivery_projection_rejection_count(&self) -> usize {
        self.values
            .subscription_fanout_delivery_projection_rejection_count
    }

    pub fn subscription_fanout_projection_validation_count(&self) -> usize {
        self.values.subscription_fanout_projection_validation_count
    }

    pub fn subscription_fanout_projection_validation_rejection_count(&self) -> usize {
        self.values
            .subscription_fanout_projection_validation_rejection_count
    }
}
