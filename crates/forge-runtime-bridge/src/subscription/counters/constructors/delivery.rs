use crate::subscription::BridgeSubscriptionDeliveryDensityPosture;

use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_delivery_cost_profile(posture: BridgeSubscriptionDeliveryDensityPosture) -> Self {
        let mut values = BridgeSubscriptionCounterValues {
            subscription_delivery_cost_profile_selection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        };
        match posture {
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery => {
                values.subscription_delivery_density_sparse_count = 1
            }
            BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow => {
                values.subscription_delivery_density_coalesced_count = 1
            }
            BridgeSubscriptionDeliveryDensityPosture::DenseRestartRequired => {
                values.subscription_delivery_density_dense_restart_count = 1
            }
            BridgeSubscriptionDeliveryDensityPosture::RejectedOverBudget => {}
        }
        Self::from_values(values)
    }

    pub fn from_delivery_cost_profile_rejection(over_budget: bool) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_cost_profile_rejection_count: 1,
            subscription_delivery_over_budget_rejection_count: usize::from(over_budget),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_consumer_contract_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_consumer_contract_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_consumer_contract_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_consumer_contract_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_active_subscription() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_activation_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_buffer_plan() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_arena_reset_count: 1,
            subscription_delivery_buffer_reuse_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_window(member_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_record_count: 1,
            subscription_delivery_member_count: member_count,
            subscription_delivery_family_selection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_diagnostics_reference() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_diagnostics_reference_emit_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_window_seed_retention() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_window_seed_retention_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_replay_seed_retention() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_replay_seed_retention_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_replay_readiness_inspection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_replay_readiness_inspection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_replay_plan(window_count: usize, member_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_replay_plan_count: 1,
            subscription_delivery_replay_retained_window_count: window_count,
            subscription_delivery_replay_retained_member_count: member_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_delivery_replay_plan_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_replay_plan_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }
}
