use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_temporal_subscription_admission() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_admission_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_subscription_rejection() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_rejection_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_activation_ready() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_activation_ready_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_time_only_cause() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_time_only_cause_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_truth_plus_time_cause() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_truth_plus_time_cause_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_duplicate_clock_rejection() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_duplicate_clock_rejection_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_stale_clock_rejection() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_stale_clock_rejection_count = 1;
        Self::from_values(values)
    }

    pub fn from_temporal_delivery_plan() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_temporal_delivery_plan_count = 1;
        Self::from_values(values)
    }

    pub fn from_historical_truth_basis_admission() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_historical_truth_basis_admission_count = 1;
        Self::from_values(values)
    }

    pub fn from_historical_previous_value_evidence() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_historical_previous_value_evidence_count = 1;
        Self::from_values(values)
    }

    pub fn from_historical_temporal_replay_basis_admission() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_historical_temporal_replay_basis_admission_count = 1;
        Self::from_values(values)
    }

    pub fn from_historical_temporal_replay_rejection() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_historical_temporal_replay_rejection_count = 1;
        Self::from_values(values)
    }

    pub fn from_historical_temporal_readiness() -> Self {
        let mut values = BridgeSubscriptionCounterValues::default();
        values.subscription_historical_temporal_readiness_count = 1;
        Self::from_values(values)
    }
}
