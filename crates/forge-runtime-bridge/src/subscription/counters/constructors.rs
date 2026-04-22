use super::{BridgeSubscriptionCounterValues, BridgeSubscriptionCounters};

impl BridgeSubscriptionCounters {
    pub fn from_frozen_registry(
        family_count: usize,
        family_supported_slice_kind_count: usize,
    ) -> Self {
        Self::new(
            1,
            family_count,
            family_supported_slice_kind_count,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    pub fn from_declaration(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(
            0,
            0,
            0,
            1,
            1,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    pub fn from_rejection(
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        Self::new(
            0,
            0,
            0,
            1,
            0,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    pub fn from_basis_binding() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
    }
    pub fn from_incompatible_basis_kind_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }
    pub fn from_basis_resolution_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }
    pub fn from_signal_strategy_descriptor() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
    }
    pub fn from_admitted_subscription() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0)
    }
    pub fn from_lifecycle_record() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
    }
    pub fn from_diagnostics_bundle() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
    }
    pub fn from_replay_reconstruction() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
    }
    pub fn from_replay_mismatch() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
    }

    pub fn from_delivery_cost_profile(
        posture: super::super::BridgeSubscriptionDeliveryDensityPosture,
    ) -> Self {
        let mut values = BridgeSubscriptionCounterValues {
            subscription_delivery_cost_profile_selection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        };
        match posture {
            super::super::BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery => {
                values.subscription_delivery_density_sparse_count = 1
            }
            super::super::BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow => {
                values.subscription_delivery_density_coalesced_count = 1
            }
            super::super::BridgeSubscriptionDeliveryDensityPosture::DenseRestartRequired => {
                values.subscription_delivery_density_dense_restart_count = 1
            }
            super::super::BridgeSubscriptionDeliveryDensityPosture::RejectedOverBudget => {}
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

    pub fn from_delivery_replay_plan_rejection(over_budget: bool) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_delivery_replay_plan_rejection_count: 1,
            subscription_delivery_over_budget_rejection_count: usize::from(over_budget),
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
        raw_stream_checkpoint: bool,
        checkpoint_truncated: bool,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_resume_admission_rejection_count: 1,
            subscription_raw_stream_checkpoint_rejection_count: usize::from(raw_stream_checkpoint),
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

    pub fn from_continuation_index(candidate_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_index_build_count: 1,
            subscription_continuation_candidate_count: candidate_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_continuation_decision(child_record_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_candidate_index_lookup_count: 1,
            subscription_continuation_decision_count: 1,
            subscription_continuation_child_record_count: child_record_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_continuation_rejection(candidate_index_lookup: bool, branch_leak: bool) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_continuation_candidate_index_lookup_count: usize::from(
                candidate_index_lookup,
            ),
            subscription_continuation_rejection_count: 1,
            subscription_branch_leak_rejection_count: usize::from(branch_leak),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_basis_admission() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_basis_admission_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_basis_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_basis_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_activation() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_activation_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_residue_scope_index(artifact_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_residue_check_count: artifact_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_discard(residue_check_count: usize) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_discard_count: 1,
            subscription_preview_residue_check_count: residue_check_count,
            subscription_preview_residue_scope_index_lookup_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_discard_rejection(
        nonzero_residue: bool,
        residue_check_count: usize,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_discard_rejection_count: 1,
            subscription_preview_residue_check_count: residue_check_count,
            subscription_preview_residue_scope_index_lookup_count: 1,
            subscription_preview_residue_nonzero_count: usize::from(nonzero_residue),
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_promotion() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_promotion_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    pub fn from_subscription_preview_promotion_rejection() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            subscription_preview_promotion_rejection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        })
    }
}
