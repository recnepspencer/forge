use super::*;

impl RuntimeBridge {
    /// Admits one Phase 1 delivery cost profile before active delivery.
    pub fn admit_subscription_delivery_cost_profile(
        &self,
        density_posture: BridgeSubscriptionDeliveryDensityPosture,
        max_member_count: usize,
        max_coalesced_member_width: usize,
        max_fanout_width: usize,
    ) -> Result<BridgeSubscriptionDeliveryCostProfile, BridgeSubscriptionDeliveryCostProfileRejection>
    {
        let _ = self;
        BridgeSubscriptionDeliveryCostProfile::admit(
            density_posture,
            max_member_count,
            max_coalesced_member_width,
            max_fanout_width,
        )
    }

    /// Admits a single-consumer Phase 1 contract. Callback and channel
    /// identity are intentionally absent from this API.
    pub fn admit_subscription_consumer_contract(
        &self,
        family: BridgeSubscriptionConsumerContractFamily,
        pacing_capability: BridgeSubscriptionConsumerPacingCapability,
        backpressure_posture: BridgeSubscriptionConsumerBackpressurePosture,
        coalescing_admitted: bool,
        diagnostics_retention: BridgeSubscriptionConsumerDiagnosticsRetention,
    ) -> Result<BridgeSubscriptionConsumerContract, BridgeSubscriptionConsumerContractRejection>
    {
        let _ = self;
        BridgeSubscriptionConsumerContract::admit(
            family,
            pacing_capability,
            backpressure_posture,
            coalescing_admitted,
            diagnostics_retention,
        )
    }

    /// Consumes an activation-ready subscription into an active delivery proof.
    pub fn activate_subscription_delivery(
        &self,
        activation_ready: BridgeSubscriptionActivationReady,
        cost_profile: BridgeSubscriptionDeliveryCostProfile,
        consumer_contract: BridgeSubscriptionConsumerContract,
    ) -> BridgeActiveSubscription {
        let _ = self;
        BridgeActiveSubscription::activate(activation_ready, cost_profile, consumer_contract)
    }
}
