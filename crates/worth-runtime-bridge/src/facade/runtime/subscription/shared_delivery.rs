use super::*;

impl RuntimeBridge {
    /// Plans one shared-consumer delivery boundary from an admitted active
    /// subscription, a canonical mixed-cause delivery window, and an admitted
    /// fanout layout.
    pub fn plan_shared_subscription_delivery(
        &self,
        active_subscription: &BridgeActiveSubscription,
        mixed_cause_window: &BridgeMixedCauseDeliveryWindowPlan,
        fanout_layout: &BridgeSubscriptionFanoutLayout,
    ) -> Result<BridgeSharedConsumerDeliveryPlan, BridgeSharedConsumerDeliveryPlanRejection> {
        let _ = self;
        BridgeSharedConsumerDeliveryPlan::plan(
            active_subscription,
            mixed_cause_window,
            fanout_layout,
        )
    }

    /// Freezes one shared-consumer delivery layout from an admitted shared
    /// delivery plan without rediscovering consumer grouping or cause order.
    pub fn build_shared_subscription_delivery_layout(
        &self,
        plan: &BridgeSharedConsumerDeliveryPlan,
    ) -> BridgeSharedConsumerDeliveryLayout {
        let _ = self;
        BridgeSharedConsumerDeliveryLayout::build(plan)
    }

    /// Creates a draft canonical shared-consumer delivery bundle from a
    /// delivery layout.
    pub fn draft_shared_delivery_bundle(
        &self,
        layout: &BridgeSharedConsumerDeliveryLayout,
    ) -> BridgeSharedConsumerDeliveryBundleDraft {
        let _ = self;
        BridgeSharedConsumerDeliveryBundleDraft::draft(layout)
    }

    /// Seals a draft shared-consumer delivery bundle into a canonical bundle
    /// artifact for projection and acknowledgement.
    pub fn seal_shared_delivery_bundle(
        &self,
        draft: BridgeSharedConsumerDeliveryBundleDraft,
    ) -> BridgeSharedConsumerDeliveryBundleSealed {
        let _ = self;
        draft.seal()
    }

    /// Projects one consumer-facing delivery view from a sealed canonical
    /// shared-consumer bundle.
    pub fn project_shared_delivery_consumer(
        &self,
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        consumer_projection_ordinal: usize,
    ) -> Result<
        BridgeSharedConsumerDeliveryProjection,
        BridgeSharedConsumerDeliveryProjectionRejection,
    > {
        let _ = self;
        BridgeSharedConsumerDeliveryProjection::project(bundle, consumer_projection_ordinal)
    }

    /// Admits one acknowledged consumer frontier from a shared-consumer bundle
    /// projection without consulting host callback state.
    pub fn admit_shared_delivery_acknowledgement_frontier(
        &self,
        bundle: &BridgeSharedConsumerDeliveryBundleSealed,
        projection: &BridgeSharedConsumerDeliveryProjection,
        acknowledged_ordered_cause_sequence: usize,
    ) -> Result<
        BridgeSharedDeliveryAcknowledgementFrontier,
        BridgeSharedDeliveryAcknowledgementFrontierRejection,
    > {
        let _ = self;
        BridgeSharedDeliveryAcknowledgementFrontier::admit(
            bundle,
            projection,
            acknowledged_ordered_cause_sequence,
        )
    }
}
