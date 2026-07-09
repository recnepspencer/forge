use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_shared_delivery_plan() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_plan_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_plan_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_plan_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_layout() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_layout_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_bundle_draft() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_bundle_draft_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_bundle_sealed() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_bundle_sealed_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_projection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_projection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_projection_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_projection_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_acknowledgement() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_acknowledgement_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_shared_delivery_acknowledgement_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_shared_delivery_acknowledgement_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }
}
