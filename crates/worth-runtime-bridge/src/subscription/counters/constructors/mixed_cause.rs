use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_mixed_cause_ordering_request() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_ordering_request_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_ordering() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_ordering_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_ordered() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_ordered_cause_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_duplicate_suppression() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_duplicate_suppression_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_denied() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_denied_cause_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_authoritative_preview_rejection() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_authoritative_preview_rejection_count = 1;
        Self::from_values(values)
    }

    pub fn from_mixed_cause_delivery_window_plan() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_mixed_cause_delivery_window_plan_count = 1;
        Self::from_values(values)
    }
}
