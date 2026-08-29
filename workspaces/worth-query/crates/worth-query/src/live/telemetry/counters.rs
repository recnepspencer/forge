#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LivePolicyCounters {
    pub(in crate::live) live_invalidation_event_count: usize,
    pub(in crate::live) live_relevance_match_count: usize,
    pub(in crate::live) live_irrelevant_suppression_count: usize,
    pub(in crate::live) live_threshold_suppression_count: usize,
    pub(in crate::live) live_patch_count: usize,
    pub(in crate::live) live_patch_delivery_count: usize,
    pub(in crate::live) live_suppressed_update_count: usize,
    pub(in crate::live) live_patch_field_delta_count: usize,
    pub(in crate::live) live_collection_membership_change_count: usize,
    pub(in crate::live) live_collection_reorder_count: usize,
    pub(in crate::live) live_materialization_patch_count: usize,
    pub(in crate::live) live_refresh_fallback_count: usize,
    pub(in crate::live) live_refresh_denial_count: usize,
    pub(in crate::live) live_replay_change_count: usize,
    pub(in crate::live) live_change_sequence_gap_count: usize,
    pub(in crate::live) live_coalesced_change_bundle_count: usize,
    pub(in crate::live) live_coalescing_denial_count: usize,
    pub(in crate::live) live_delivery_width: usize,
    pub(in crate::live) live_patch_width_overflow_count: usize,
    pub(in crate::live) live_refresh_cost_class_count: usize,
    pub(in crate::live) live_work_avoided_by_irrelevance_count: usize,
    pub(in crate::live) live_work_avoided_by_stable_ordering_count: usize,
    pub(in crate::live) live_work_avoided_by_scope_proof_count: usize,
    pub(in crate::live) live_executor_rediscovery_count: usize,
    pub(in crate::live) live_progress_advance_count: usize,
    pub(in crate::live) live_non_monotonic_sequence_rejection_count: usize,
    pub(in crate::live) live_invalid_promotion_rejection_count: usize,
    pub(in crate::live) live_unsupported_patch_family_rejection_count: usize,
    pub(in crate::live) locality_region_match_count: usize,
    pub(in crate::live) locality_partition_match_count: usize,
    pub(in crate::live) locality_off_region_suppression_count: usize,
    pub(in crate::live) locality_irrelevant_broad_control_count: usize,
    pub(in crate::live) locality_replay_change_count: usize,
    pub(in crate::live) locality_replay_divergence_count: usize,
    pub(in crate::live) locality_breadth_budget_cross_count: usize,
    pub(in crate::live) locality_widening_admission_count: usize,
    pub(in crate::live) locality_widening_budget_cross_count: usize,
    pub(in crate::live) locality_widening_denial_count: usize,
    pub(in crate::live) locality_bridge_slice_incompatibility_count: usize,
    pub(in crate::live) stream_contract_admission_count: usize,
    pub(in crate::live) stream_contract_denial_count: usize,
    pub(in crate::live) stream_lowered_delivery_count: usize,
    pub(in crate::live) stream_lowered_delivery_member_count: usize,
    pub(in crate::live) stream_lowered_delivery_window_width: usize,
    pub(in crate::live) stream_lowered_delivery_width: usize,
    pub(in crate::live) stream_window_width_budget_cross_count: usize,
    pub(in crate::live) stream_member_width_budget_cross_count: usize,
    pub(in crate::live) locality_work_avoided_by_region_narrowing_count: usize,
    pub(in crate::live) locality_work_avoided_vs_broad_control_count: usize,
    pub(in crate::live) locality_executor_rediscovery_count: usize,
    pub(in crate::live) locality_unsupported_family_rejection_count: usize,
    pub(in crate::live) locality_unsupported_predicate_rejection_count: usize,
}

impl LivePolicyCounters {
    pub fn live_invalidation_event_count(&self) -> usize {
        self.live_invalidation_event_count
    }

    pub fn live_relevance_match_count(&self) -> usize {
        self.live_relevance_match_count
    }

    pub fn live_irrelevant_suppression_count(&self) -> usize {
        self.live_irrelevant_suppression_count
    }

    pub fn live_threshold_suppression_count(&self) -> usize {
        self.live_threshold_suppression_count
    }

    pub fn live_patch_count(&self) -> usize {
        self.live_patch_count
    }

    pub fn live_patch_delivery_count(&self) -> usize {
        self.live_patch_delivery_count
    }

    pub fn live_suppressed_update_count(&self) -> usize {
        self.live_suppressed_update_count
    }

    pub fn live_patch_field_delta_count(&self) -> usize {
        self.live_patch_field_delta_count
    }

    pub fn live_collection_membership_change_count(&self) -> usize {
        self.live_collection_membership_change_count
    }

    pub fn live_collection_reorder_count(&self) -> usize {
        self.live_collection_reorder_count
    }

    pub fn live_materialization_patch_count(&self) -> usize {
        self.live_materialization_patch_count
    }

    pub fn live_refresh_fallback_count(&self) -> usize {
        self.live_refresh_fallback_count
    }

    pub fn live_refresh_denial_count(&self) -> usize {
        self.live_refresh_denial_count
    }

    pub fn live_replay_change_count(&self) -> usize {
        self.live_replay_change_count
    }

    pub fn live_change_sequence_gap_count(&self) -> usize {
        self.live_change_sequence_gap_count
    }

    pub fn live_coalesced_change_bundle_count(&self) -> usize {
        self.live_coalesced_change_bundle_count
    }

    pub fn live_coalescing_denial_count(&self) -> usize {
        self.live_coalescing_denial_count
    }

    pub fn live_delivery_width(&self) -> usize {
        self.live_delivery_width
    }

    pub fn live_patch_width_overflow_count(&self) -> usize {
        self.live_patch_width_overflow_count
    }

    pub fn live_refresh_cost_class_count(&self) -> usize {
        self.live_refresh_cost_class_count
    }

    pub fn live_work_avoided_by_irrelevance_count(&self) -> usize {
        self.live_work_avoided_by_irrelevance_count
    }

    pub fn live_work_avoided_by_stable_ordering_count(&self) -> usize {
        self.live_work_avoided_by_stable_ordering_count
    }

    pub fn live_work_avoided_by_scope_proof_count(&self) -> usize {
        self.live_work_avoided_by_scope_proof_count
    }

    pub fn live_executor_rediscovery_count(&self) -> usize {
        self.live_executor_rediscovery_count
    }

    pub fn live_progress_advance_count(&self) -> usize {
        self.live_progress_advance_count
    }

    pub fn live_non_monotonic_sequence_rejection_count(&self) -> usize {
        self.live_non_monotonic_sequence_rejection_count
    }

    pub fn live_invalid_promotion_rejection_count(&self) -> usize {
        self.live_invalid_promotion_rejection_count
    }

    pub fn live_unsupported_patch_family_rejection_count(&self) -> usize {
        self.live_unsupported_patch_family_rejection_count
    }

    pub fn locality_region_match_count(&self) -> usize {
        self.locality_region_match_count
    }

    pub fn locality_partition_match_count(&self) -> usize {
        self.locality_partition_match_count
    }

    pub fn locality_off_region_suppression_count(&self) -> usize {
        self.locality_off_region_suppression_count
    }

    pub fn locality_irrelevant_broad_control_count(&self) -> usize {
        self.locality_irrelevant_broad_control_count
    }

    pub fn locality_replay_change_count(&self) -> usize {
        self.locality_replay_change_count
    }

    pub fn locality_replay_divergence_count(&self) -> usize {
        self.locality_replay_divergence_count
    }

    pub fn locality_breadth_budget_cross_count(&self) -> usize {
        self.locality_breadth_budget_cross_count
    }

    pub fn locality_widening_admission_count(&self) -> usize {
        self.locality_widening_admission_count
    }

    pub fn locality_widening_budget_cross_count(&self) -> usize {
        self.locality_widening_budget_cross_count
    }

    pub fn locality_widening_denial_count(&self) -> usize {
        self.locality_widening_denial_count
    }

    pub fn locality_bridge_slice_incompatibility_count(&self) -> usize {
        self.locality_bridge_slice_incompatibility_count
    }

    pub fn stream_contract_admission_count(&self) -> usize {
        self.stream_contract_admission_count
    }

    pub fn stream_contract_denial_count(&self) -> usize {
        self.stream_contract_denial_count
    }

    pub fn stream_lowered_delivery_count(&self) -> usize {
        self.stream_lowered_delivery_count
    }

    pub fn stream_lowered_delivery_member_count(&self) -> usize {
        self.stream_lowered_delivery_member_count
    }

    pub fn stream_lowered_delivery_window_width(&self) -> usize {
        self.stream_lowered_delivery_window_width
    }

    pub fn stream_lowered_delivery_width(&self) -> usize {
        self.stream_lowered_delivery_width
    }

    pub fn stream_window_width_budget_cross_count(&self) -> usize {
        self.stream_window_width_budget_cross_count
    }

    pub fn stream_member_width_budget_cross_count(&self) -> usize {
        self.stream_member_width_budget_cross_count
    }

    pub fn locality_work_avoided_by_region_narrowing_count(&self) -> usize {
        self.locality_work_avoided_by_region_narrowing_count
    }

    pub fn locality_work_avoided_vs_broad_control_count(&self) -> usize {
        self.locality_work_avoided_vs_broad_control_count
    }

    pub fn locality_executor_rediscovery_count(&self) -> usize {
        self.locality_executor_rediscovery_count
    }

    pub fn locality_unsupported_family_rejection_count(&self) -> usize {
        self.locality_unsupported_family_rejection_count
    }

    pub fn locality_unsupported_predicate_rejection_count(&self) -> usize {
        self.locality_unsupported_predicate_rejection_count
    }

    pub fn has_activity(&self) -> bool {
        self.live_invalidation_event_count > 0
            || self.live_relevance_match_count > 0
            || self.live_irrelevant_suppression_count > 0
            || self.live_threshold_suppression_count > 0
            || self.live_patch_count > 0
            || self.live_patch_delivery_count > 0
            || self.live_suppressed_update_count > 0
            || self.live_patch_field_delta_count > 0
            || self.live_collection_membership_change_count > 0
            || self.live_collection_reorder_count > 0
            || self.live_materialization_patch_count > 0
            || self.live_refresh_fallback_count > 0
            || self.live_refresh_denial_count > 0
            || self.live_replay_change_count > 0
            || self.live_change_sequence_gap_count > 0
            || self.live_coalesced_change_bundle_count > 0
            || self.live_coalescing_denial_count > 0
            || self.live_delivery_width > 0
            || self.live_patch_width_overflow_count > 0
            || self.live_refresh_cost_class_count > 0
            || self.live_work_avoided_by_irrelevance_count > 0
            || self.live_work_avoided_by_stable_ordering_count > 0
            || self.live_work_avoided_by_scope_proof_count > 0
            || self.live_progress_advance_count > 0
            || self.live_non_monotonic_sequence_rejection_count > 0
            || self.live_invalid_promotion_rejection_count > 0
            || self.live_unsupported_patch_family_rejection_count > 0
            || self.locality_region_match_count > 0
            || self.locality_partition_match_count > 0
            || self.locality_off_region_suppression_count > 0
            || self.locality_irrelevant_broad_control_count > 0
            || self.locality_replay_change_count > 0
            || self.locality_replay_divergence_count > 0
            || self.locality_breadth_budget_cross_count > 0
            || self.locality_widening_admission_count > 0
            || self.locality_widening_budget_cross_count > 0
            || self.locality_widening_denial_count > 0
            || self.locality_bridge_slice_incompatibility_count > 0
            || self.stream_contract_admission_count > 0
            || self.stream_contract_denial_count > 0
            || self.stream_lowered_delivery_count > 0
            || self.stream_lowered_delivery_member_count > 0
            || self.stream_lowered_delivery_window_width > 0
            || self.stream_lowered_delivery_width > 0
            || self.stream_window_width_budget_cross_count > 0
            || self.stream_member_width_budget_cross_count > 0
            || self.locality_work_avoided_by_region_narrowing_count > 0
            || self.locality_work_avoided_vs_broad_control_count > 0
            || self.locality_executor_rediscovery_count > 0
            || self.locality_unsupported_family_rejection_count > 0
            || self.locality_unsupported_predicate_rejection_count > 0
    }

    pub(crate) fn absorb(&mut self, other: &Self) {
        self.live_invalidation_event_count += other.live_invalidation_event_count;
        self.live_relevance_match_count += other.live_relevance_match_count;
        self.live_irrelevant_suppression_count += other.live_irrelevant_suppression_count;
        self.live_threshold_suppression_count += other.live_threshold_suppression_count;
        self.live_patch_count += other.live_patch_count;
        self.live_patch_delivery_count += other.live_patch_delivery_count;
        self.live_suppressed_update_count += other.live_suppressed_update_count;
        self.live_patch_field_delta_count += other.live_patch_field_delta_count;
        self.live_collection_membership_change_count +=
            other.live_collection_membership_change_count;
        self.live_collection_reorder_count += other.live_collection_reorder_count;
        self.live_materialization_patch_count += other.live_materialization_patch_count;
        self.live_refresh_fallback_count += other.live_refresh_fallback_count;
        self.live_refresh_denial_count += other.live_refresh_denial_count;
        self.live_replay_change_count += other.live_replay_change_count;
        self.live_change_sequence_gap_count += other.live_change_sequence_gap_count;
        self.live_coalesced_change_bundle_count += other.live_coalesced_change_bundle_count;
        self.live_coalescing_denial_count += other.live_coalescing_denial_count;
        self.live_delivery_width += other.live_delivery_width;
        self.live_patch_width_overflow_count += other.live_patch_width_overflow_count;
        self.live_refresh_cost_class_count += other.live_refresh_cost_class_count;
        self.live_work_avoided_by_irrelevance_count += other.live_work_avoided_by_irrelevance_count;
        self.live_work_avoided_by_stable_ordering_count +=
            other.live_work_avoided_by_stable_ordering_count;
        self.live_work_avoided_by_scope_proof_count += other.live_work_avoided_by_scope_proof_count;
        self.live_executor_rediscovery_count += other.live_executor_rediscovery_count;
        self.live_progress_advance_count += other.live_progress_advance_count;
        self.live_non_monotonic_sequence_rejection_count +=
            other.live_non_monotonic_sequence_rejection_count;
        self.live_invalid_promotion_rejection_count += other.live_invalid_promotion_rejection_count;
        self.live_unsupported_patch_family_rejection_count +=
            other.live_unsupported_patch_family_rejection_count;
        self.locality_region_match_count += other.locality_region_match_count;
        self.locality_partition_match_count += other.locality_partition_match_count;
        self.locality_off_region_suppression_count += other.locality_off_region_suppression_count;
        self.locality_irrelevant_broad_control_count +=
            other.locality_irrelevant_broad_control_count;
        self.locality_replay_change_count += other.locality_replay_change_count;
        self.locality_replay_divergence_count += other.locality_replay_divergence_count;
        self.locality_breadth_budget_cross_count += other.locality_breadth_budget_cross_count;
        self.locality_widening_admission_count += other.locality_widening_admission_count;
        self.locality_widening_budget_cross_count += other.locality_widening_budget_cross_count;
        self.locality_widening_denial_count += other.locality_widening_denial_count;
        self.locality_bridge_slice_incompatibility_count +=
            other.locality_bridge_slice_incompatibility_count;
        self.stream_contract_admission_count += other.stream_contract_admission_count;
        self.stream_contract_denial_count += other.stream_contract_denial_count;
        self.stream_lowered_delivery_count += other.stream_lowered_delivery_count;
        self.stream_lowered_delivery_member_count += other.stream_lowered_delivery_member_count;
        self.stream_lowered_delivery_window_width += other.stream_lowered_delivery_window_width;
        self.stream_lowered_delivery_width += other.stream_lowered_delivery_width;
        self.stream_window_width_budget_cross_count += other.stream_window_width_budget_cross_count;
        self.stream_member_width_budget_cross_count += other.stream_member_width_budget_cross_count;
        self.locality_work_avoided_by_region_narrowing_count +=
            other.locality_work_avoided_by_region_narrowing_count;
        self.locality_work_avoided_vs_broad_control_count +=
            other.locality_work_avoided_vs_broad_control_count;
        self.locality_executor_rediscovery_count += other.locality_executor_rediscovery_count;
        self.locality_unsupported_family_rejection_count +=
            other.locality_unsupported_family_rejection_count;
        self.locality_unsupported_predicate_rejection_count +=
            other.locality_unsupported_predicate_rejection_count;
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedLiveCounters {
    pub(in crate::live) snapshot: LivePolicyCounters,
}

#[cfg(test)]
impl RegionScopedLiveCounters {
    pub fn snapshot(&self) -> &LivePolicyCounters {
        &self.snapshot
    }
}
