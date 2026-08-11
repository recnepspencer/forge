use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn subscription_shared_delivery_plan_count(&self) -> usize {
        self.values.subscription_shared_delivery_plan_count
    }

    pub fn subscription_shared_delivery_plan_rejection_count(&self) -> usize {
        self.values
            .subscription_shared_delivery_plan_rejection_count
    }

    pub fn subscription_shared_delivery_layout_count(&self) -> usize {
        self.values.subscription_shared_delivery_layout_count
    }

    pub fn subscription_shared_delivery_bundle_draft_count(&self) -> usize {
        self.values.subscription_shared_delivery_bundle_draft_count
    }

    pub fn subscription_shared_delivery_bundle_sealed_count(&self) -> usize {
        self.values.subscription_shared_delivery_bundle_sealed_count
    }

    pub fn subscription_shared_delivery_projection_count(&self) -> usize {
        self.values.subscription_shared_delivery_projection_count
    }

    pub fn subscription_shared_delivery_projection_rejection_count(&self) -> usize {
        self.values
            .subscription_shared_delivery_projection_rejection_count
    }

    pub fn subscription_shared_delivery_acknowledgement_count(&self) -> usize {
        self.values
            .subscription_shared_delivery_acknowledgement_count
    }

    pub fn subscription_shared_delivery_acknowledgement_rejection_count(&self) -> usize {
        self.values
            .subscription_shared_delivery_acknowledgement_rejection_count
    }
}
