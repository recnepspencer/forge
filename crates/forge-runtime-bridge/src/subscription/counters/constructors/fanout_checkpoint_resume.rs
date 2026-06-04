use super::super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_fanout_plan_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_plan_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_plan_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_plan_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_layout(consumer_binding_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_layout_build_count: 1,
            subscription_fanout_consumer_binding_count: consumer_binding_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_delivery_projection(projection_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_delivery_projection_count: projection_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_delivery_projection_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_delivery_projection_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_projection_validation() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_projection_validation_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_fanout_projection_validation_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_fanout_projection_validation_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_acknowledgement_frontier_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_acknowledgement_frontier_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_acknowledgement_frontier_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_acknowledgement_frontier_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_checkpoint_ready() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_checkpoint_ready_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_checkpoint_publication() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_checkpoint_publication_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_checkpoint_publication_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_checkpoint_publication_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_duplicate_replay_policy_selection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_duplicate_replay_policy_selection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_resume_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_resume_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_resume_admission_rejection(
        unsealed_stream_checkpoint: bool,
        checkpoint_truncated: bool,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_resume_admission_rejection_count: 1,
            subscription_unsealed_stream_checkpoint_rejection_count: usize::from(
                unsealed_stream_checkpoint,
            ),
            subscription_checkpoint_truncation_rejection_count: usize::from(checkpoint_truncated),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_resume_plan() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_resume_plan_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }
}
