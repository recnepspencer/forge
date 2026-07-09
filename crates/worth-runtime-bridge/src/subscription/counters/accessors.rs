use super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn family_registry_freeze_count(&self) -> usize {
        self.values.family_registry_freeze_count
    }
    pub fn family_count(&self) -> usize {
        self.values.family_count
    }
    pub fn family_supported_slice_kind_count(&self) -> usize {
        self.values.family_supported_slice_kind_count
    }
    pub fn family_lookup_count(&self) -> usize {
        self.values.family_lookup_count
    }
    pub fn declaration_count(&self) -> usize {
        self.values.declaration_count
    }
    pub fn declaration_input_slice_intent_count(&self) -> usize {
        self.values.declaration_input_slice_intent_count
    }
    pub fn declaration_normalized_slice_intent_count(&self) -> usize {
        self.values.declaration_normalized_slice_intent_count
    }
    pub fn declaration_deduplicated_slice_intent_count(&self) -> usize {
        self.values.declaration_deduplicated_slice_intent_count
    }
    pub fn declaration_rejection_count(&self) -> usize {
        self.values.declaration_rejection_count
    }
    pub fn basis_request_count(&self) -> usize {
        self.values.basis_request_count
    }
    pub fn basis_binding_count(&self) -> usize {
        self.values.basis_binding_count
    }
    pub fn basis_rejection_count(&self) -> usize {
        self.values.basis_rejection_count
    }
    pub fn signal_strategy_selection_count(&self) -> usize {
        self.values.signal_strategy_selection_count
    }
    pub fn signal_strategy_rejection_count(&self) -> usize {
        self.values.signal_strategy_rejection_count
    }
    pub fn admitted_subscription_count(&self) -> usize {
        self.values.admitted_subscription_count
    }
    pub fn lifecycle_record_count(&self) -> usize {
        self.values.lifecycle_record_count
    }
    pub fn replay_reconstruction_count(&self) -> usize {
        self.values.replay_reconstruction_count
    }
    pub fn replay_mismatch_count(&self) -> usize {
        self.values.replay_mismatch_count
    }
    pub fn diagnostics_bundle_count(&self) -> usize {
        self.values.diagnostics_bundle_count
    }
    pub fn subscription_delivery_cost_profile_selection_count(&self) -> usize {
        self.values
            .subscription_delivery_cost_profile_selection_count
    }
    pub fn subscription_delivery_cost_profile_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_cost_profile_rejection_count
    }
    pub fn subscription_delivery_over_budget_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_over_budget_rejection_count
    }
    pub fn subscription_delivery_density_sparse_count(&self) -> usize {
        self.values.subscription_delivery_density_sparse_count
    }
    pub fn subscription_delivery_density_coalesced_count(&self) -> usize {
        self.values.subscription_delivery_density_coalesced_count
    }
    pub fn subscription_delivery_density_dense_restart_count(&self) -> usize {
        self.values
            .subscription_delivery_density_dense_restart_count
    }
    pub fn subscription_consumer_contract_admission_count(&self) -> usize {
        self.values.subscription_consumer_contract_admission_count
    }
    pub fn subscription_consumer_contract_rejection_count(&self) -> usize {
        self.values.subscription_consumer_contract_rejection_count
    }
    pub fn subscription_activation_count(&self) -> usize {
        self.values.subscription_activation_count
    }
    pub fn subscription_delivery_record_count(&self) -> usize {
        self.values.subscription_delivery_record_count
    }
    pub fn subscription_delivery_member_count(&self) -> usize {
        self.values.subscription_delivery_member_count
    }
    pub fn subscription_delivery_family_selection_count(&self) -> usize {
        self.values.subscription_delivery_family_selection_count
    }
    pub fn subscription_diagnostics_reference_emit_count(&self) -> usize {
        self.values.subscription_diagnostics_reference_emit_count
    }
    pub fn subscription_rich_diagnostics_hot_path_materialization_count(&self) -> usize {
        self.values
            .subscription_rich_diagnostics_hot_path_materialization_count
    }
    pub fn subscription_delivery_arena_reset_count(&self) -> usize {
        self.values.subscription_delivery_arena_reset_count
    }
    pub fn subscription_delivery_buffer_reuse_count(&self) -> usize {
        self.values.subscription_delivery_buffer_reuse_count
    }
    pub fn subscription_allocation_count(&self) -> usize {
        self.values.subscription_allocation_count
    }
    pub fn subscription_clone_count(&self) -> usize {
        self.values.subscription_clone_count
    }
    pub fn subscription_callback_identity_scan_count(&self) -> usize {
        self.values.subscription_callback_identity_scan_count
    }
    pub fn subscription_active_registry_scan_count(&self) -> usize {
        self.values.subscription_active_registry_scan_count
    }
    pub fn subscription_fanout_per_member_consumer_scan_count(&self) -> usize {
        self.values
            .subscription_fanout_per_member_consumer_scan_count
    }
    pub fn subscription_preview_residue_nonzero_count(&self) -> usize {
        self.values.subscription_preview_residue_nonzero_count
    }
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
    pub fn subscription_delivery_window_seed_retention_count(&self) -> usize {
        self.values
            .subscription_delivery_window_seed_retention_count
    }
    pub fn subscription_delivery_replay_seed_retention_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_seed_retention_count
    }
    pub fn subscription_delivery_replay_readiness_inspection_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_readiness_inspection_count
    }
    pub fn subscription_delivery_replay_plan_count(&self) -> usize {
        self.values.subscription_delivery_replay_plan_count
    }
    pub fn subscription_delivery_replay_plan_rejection_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_plan_rejection_count
    }
    pub fn subscription_delivery_replay_retained_window_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_retained_window_count
    }
    pub fn subscription_delivery_replay_retained_member_count(&self) -> usize {
        self.values
            .subscription_delivery_replay_retained_member_count
    }
    pub fn subscription_fanout_projection_validation_count(&self) -> usize {
        self.values.subscription_fanout_projection_validation_count
    }
    pub fn subscription_fanout_projection_validation_rejection_count(&self) -> usize {
        self.values
            .subscription_fanout_projection_validation_rejection_count
    }
    pub fn subscription_acknowledgement_frontier_admission_count(&self) -> usize {
        self.values
            .subscription_acknowledgement_frontier_admission_count
    }
    pub fn subscription_acknowledgement_frontier_rejection_count(&self) -> usize {
        self.values
            .subscription_acknowledgement_frontier_rejection_count
    }
    pub fn subscription_checkpoint_ready_count(&self) -> usize {
        self.values.subscription_checkpoint_ready_count
    }
    pub fn subscription_checkpoint_publication_count(&self) -> usize {
        self.values.subscription_checkpoint_publication_count
    }
    pub fn subscription_checkpoint_publication_rejection_count(&self) -> usize {
        self.values
            .subscription_checkpoint_publication_rejection_count
    }
    pub fn subscription_duplicate_replay_policy_selection_count(&self) -> usize {
        self.values
            .subscription_duplicate_replay_policy_selection_count
    }
    pub fn subscription_resume_admission_count(&self) -> usize {
        self.values.subscription_resume_admission_count
    }
    pub fn subscription_resume_admission_rejection_count(&self) -> usize {
        self.values.subscription_resume_admission_rejection_count
    }
    pub fn subscription_resume_plan_count(&self) -> usize {
        self.values.subscription_resume_plan_count
    }
    pub fn subscription_resume_basis_capture_count(&self) -> usize {
        self.values.subscription_resume_basis_capture_count
    }
    pub fn subscription_resume_temporal_basis_count(&self) -> usize {
        self.values.subscription_resume_temporal_basis_count
    }
    pub fn subscription_resume_inflight_async_basis_count(&self) -> usize {
        self.values.subscription_resume_inflight_async_basis_count
    }
    pub fn subscription_resume_delivery_basis_count(&self) -> usize {
        self.values.subscription_resume_delivery_basis_count
    }
    pub fn subscription_resume_basis_admission_count(&self) -> usize {
        self.values.subscription_resume_basis_admission_count
    }
    pub fn subscription_resume_basis_rejection_count(&self) -> usize {
        self.values.subscription_resume_basis_rejection_count
    }
    pub fn subscription_resume_replay_readiness_count(&self) -> usize {
        self.values.subscription_resume_replay_readiness_count
    }
    pub fn subscription_unsealed_stream_checkpoint_rejection_count(&self) -> usize {
        self.values
            .subscription_unsealed_stream_checkpoint_rejection_count
    }
    pub fn subscription_checkpoint_truncation_rejection_count(&self) -> usize {
        self.values
            .subscription_checkpoint_truncation_rejection_count
    }
    pub fn subscription_resume_cross_branch_rejection_count(&self) -> usize {
        self.values.subscription_resume_cross_branch_rejection_count
    }
    pub fn subscription_resume_delivery_mismatch_rejection_count(&self) -> usize {
        self.values
            .subscription_resume_delivery_mismatch_rejection_count
    }
    pub fn subscription_resume_inflight_async_generation_rejection_count(&self) -> usize {
        self.values
            .subscription_resume_inflight_async_generation_rejection_count
    }
    pub fn subscription_continuation_index_build_count(&self) -> usize {
        self.values.subscription_continuation_index_build_count
    }
    pub fn subscription_continuation_candidate_count(&self) -> usize {
        self.values.subscription_continuation_candidate_count
    }
    pub fn subscription_continuation_candidate_index_lookup_count(&self) -> usize {
        self.values
            .subscription_continuation_candidate_index_lookup_count
    }
    pub fn subscription_continuation_decision_count(&self) -> usize {
        self.values.subscription_continuation_decision_count
    }
    pub fn subscription_continuation_rejection_count(&self) -> usize {
        self.values.subscription_continuation_rejection_count
    }
    pub fn subscription_branch_leak_rejection_count(&self) -> usize {
        self.values.subscription_branch_leak_rejection_count
    }
    pub fn subscription_continuation_child_record_count(&self) -> usize {
        self.values.subscription_continuation_child_record_count
    }
    pub fn subscription_continuation_full_registry_scan_count(&self) -> usize {
        self.values
            .subscription_continuation_full_registry_scan_count
    }
    pub fn subscription_preview_basis_admission_count(&self) -> usize {
        self.values.subscription_preview_basis_admission_count
    }
    pub fn subscription_preview_basis_rejection_count(&self) -> usize {
        self.values.subscription_preview_basis_rejection_count
    }
    pub fn subscription_preview_activation_count(&self) -> usize {
        self.values.subscription_preview_activation_count
    }
    pub fn subscription_preview_discard_count(&self) -> usize {
        self.values.subscription_preview_discard_count
    }
    pub fn subscription_preview_discard_rejection_count(&self) -> usize {
        self.values.subscription_preview_discard_rejection_count
    }
    pub fn subscription_preview_residue_check_count(&self) -> usize {
        self.values.subscription_preview_residue_check_count
    }
    pub fn subscription_preview_residue_scope_index_lookup_count(&self) -> usize {
        self.values
            .subscription_preview_residue_scope_index_lookup_count
    }
    pub fn subscription_preview_non_scope_registry_scan_count(&self) -> usize {
        self.values
            .subscription_preview_non_scope_registry_scan_count
    }
    pub fn subscription_preview_lifecycle_residue_envelope_count(&self) -> usize {
        self.values
            .subscription_preview_lifecycle_residue_envelope_count
    }
    pub fn subscription_preview_promotion_count(&self) -> usize {
        self.values.subscription_preview_promotion_count
    }
    pub fn subscription_preview_promotion_rejection_count(&self) -> usize {
        self.values.subscription_preview_promotion_rejection_count
    }
    pub fn subscription_preview_authoritative_readmission_count(&self) -> usize {
        self.values
            .subscription_preview_authoritative_readmission_count
    }
    pub fn subscription_preview_crossed_completion_rejection_count(&self) -> usize {
        self.values
            .subscription_preview_crossed_completion_rejection_count
    }
    pub fn subscription_preview_temporal_evidence_drift_rejection_count(&self) -> usize {
        self.values
            .subscription_preview_temporal_evidence_drift_rejection_count
    }
    pub fn subscription_temporal_admission_count(&self) -> usize {
        self.values.subscription_temporal_admission_count
    }
    pub fn subscription_temporal_rejection_count(&self) -> usize {
        self.values.subscription_temporal_rejection_count
    }
    pub fn subscription_temporal_activation_ready_count(&self) -> usize {
        self.values.subscription_temporal_activation_ready_count
    }
    pub fn subscription_temporal_time_only_cause_count(&self) -> usize {
        self.values.subscription_temporal_time_only_cause_count
    }
    pub fn subscription_temporal_truth_plus_time_cause_count(&self) -> usize {
        self.values
            .subscription_temporal_truth_plus_time_cause_count
    }
    pub fn subscription_temporal_duplicate_clock_rejection_count(&self) -> usize {
        self.values
            .subscription_temporal_duplicate_clock_rejection_count
    }
    pub fn subscription_temporal_stale_clock_rejection_count(&self) -> usize {
        self.values
            .subscription_temporal_stale_clock_rejection_count
    }
    pub fn subscription_temporal_delivery_plan_count(&self) -> usize {
        self.values.subscription_temporal_delivery_plan_count
    }
    pub fn subscription_mixed_cause_ordering_request_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordering_request_count
    }
    pub fn subscription_mixed_cause_ordering_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordering_count
    }
    pub fn subscription_mixed_cause_ordered_cause_count(&self) -> usize {
        self.values.subscription_mixed_cause_ordered_cause_count
    }
    pub fn subscription_mixed_cause_duplicate_suppression_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_duplicate_suppression_count
    }
    pub fn subscription_mixed_cause_denied_cause_count(&self) -> usize {
        self.values.subscription_mixed_cause_denied_cause_count
    }
    pub fn subscription_mixed_cause_authoritative_preview_rejection_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_authoritative_preview_rejection_count
    }
    pub fn subscription_mixed_cause_delivery_window_plan_count(&self) -> usize {
        self.values
            .subscription_mixed_cause_delivery_window_plan_count
    }
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
    pub fn subscription_historical_truth_basis_admission_count(&self) -> usize {
        self.values
            .subscription_historical_truth_basis_admission_count
    }
    pub fn subscription_historical_previous_value_evidence_count(&self) -> usize {
        self.values
            .subscription_historical_previous_value_evidence_count
    }
    pub fn subscription_historical_temporal_replay_basis_admission_count(&self) -> usize {
        self.values
            .subscription_historical_temporal_replay_basis_admission_count
    }
    pub fn subscription_historical_temporal_replay_rejection_count(&self) -> usize {
        self.values
            .subscription_historical_temporal_replay_rejection_count
    }
    pub fn subscription_historical_temporal_readiness_count(&self) -> usize {
        self.values.subscription_historical_temporal_readiness_count
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
