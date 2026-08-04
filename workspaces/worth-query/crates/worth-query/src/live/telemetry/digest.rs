use super::LivePolicyCounters;

impl LivePolicyCounters {
    pub fn digest_parts(&self, label: &str) -> Vec<String> {
        let mut parts = Vec::with_capacity(52);
        parts.extend(self.live_invalidation_relevance_digest_parts(label));
        parts.extend(self.live_patch_delivery_digest_parts(label));
        parts.extend(self.live_refresh_digest_parts(label));
        parts.extend(self.live_replay_digest_parts(label));
        parts.extend(self.live_coalescing_digest_parts(label));
        parts.extend(self.live_delivery_width_digest_parts(label));
        parts.extend(self.live_work_avoidance_digest_parts(label));
        parts.extend(self.live_progression_rejection_digest_parts(label));
        parts.extend(self.locality_match_digest_parts(label));
        parts.extend(self.stream_delivery_digest_parts(label));
        parts.extend(self.locality_efficiency_digest_parts(label));
        parts
    }

    fn live_invalidation_relevance_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_invalidation_event_count:{}",
                self.live_invalidation_event_count
            ),
            format!(
                "{label}_relevance_match_count:{}",
                self.live_relevance_match_count
            ),
            format!(
                "{label}_irrelevant_suppression_count:{}",
                self.live_irrelevant_suppression_count
            ),
            format!(
                "{label}_threshold_suppression_count:{}",
                self.live_threshold_suppression_count
            ),
        ]
    }

    fn live_patch_delivery_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!("{label}_patch_count:{}", self.live_patch_count),
            format!(
                "{label}_patch_delivery_count:{}",
                self.live_patch_delivery_count
            ),
            format!(
                "{label}_suppressed_update_count:{}",
                self.live_suppressed_update_count
            ),
            format!(
                "{label}_patch_field_delta_count:{}",
                self.live_patch_field_delta_count
            ),
            format!(
                "{label}_collection_membership_change_count:{}",
                self.live_collection_membership_change_count
            ),
            format!(
                "{label}_collection_reorder_count:{}",
                self.live_collection_reorder_count
            ),
            format!(
                "{label}_materialization_patch_count:{}",
                self.live_materialization_patch_count
            ),
        ]
    }

    fn live_refresh_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_refresh_fallback_count:{}",
                self.live_refresh_fallback_count
            ),
            format!(
                "{label}_refresh_denial_count:{}",
                self.live_refresh_denial_count
            ),
        ]
    }

    fn live_replay_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_replay_change_count:{}",
                self.live_replay_change_count
            ),
            format!(
                "{label}_change_sequence_gap_count:{}",
                self.live_change_sequence_gap_count
            ),
        ]
    }

    fn live_coalescing_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_coalesced_change_bundle_count:{}",
                self.live_coalesced_change_bundle_count
            ),
            format!(
                "{label}_coalescing_denial_count:{}",
                self.live_coalescing_denial_count
            ),
        ]
    }

    fn live_delivery_width_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!("{label}_delivery_width:{}", self.live_delivery_width),
            format!(
                "{label}_patch_width_overflow_count:{}",
                self.live_patch_width_overflow_count
            ),
            format!(
                "{label}_refresh_cost_class_count:{}",
                self.live_refresh_cost_class_count
            ),
        ]
    }

    fn live_work_avoidance_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_work_avoided_by_irrelevance_count:{}",
                self.live_work_avoided_by_irrelevance_count
            ),
            format!(
                "{label}_work_avoided_by_stable_ordering_count:{}",
                self.live_work_avoided_by_stable_ordering_count
            ),
            format!(
                "{label}_work_avoided_by_scope_proof_count:{}",
                self.live_work_avoided_by_scope_proof_count
            ),
            format!(
                "{label}_executor_rediscovery_count:{}",
                self.live_executor_rediscovery_count
            ),
        ]
    }

    fn live_progression_rejection_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_progress_advance_count:{}",
                self.live_progress_advance_count
            ),
            format!(
                "{label}_non_monotonic_sequence_rejection_count:{}",
                self.live_non_monotonic_sequence_rejection_count
            ),
            format!(
                "{label}_invalid_promotion_rejection_count:{}",
                self.live_invalid_promotion_rejection_count
            ),
            format!(
                "{label}_unsupported_patch_family_rejection_count:{}",
                self.live_unsupported_patch_family_rejection_count
            ),
        ]
    }

    fn locality_match_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_locality_region_match_count:{}",
                self.locality_region_match_count
            ),
            format!(
                "{label}_locality_partition_match_count:{}",
                self.locality_partition_match_count
            ),
            format!(
                "{label}_locality_off_region_suppression_count:{}",
                self.locality_off_region_suppression_count
            ),
            format!(
                "{label}_locality_irrelevant_broad_control_count:{}",
                self.locality_irrelevant_broad_control_count
            ),
            format!(
                "{label}_locality_replay_change_count:{}",
                self.locality_replay_change_count
            ),
            format!(
                "{label}_locality_replay_divergence_count:{}",
                self.locality_replay_divergence_count
            ),
            format!(
                "{label}_locality_breadth_budget_cross_count:{}",
                self.locality_breadth_budget_cross_count
            ),
            format!(
                "{label}_locality_widening_admission_count:{}",
                self.locality_widening_admission_count
            ),
            format!(
                "{label}_locality_widening_budget_cross_count:{}",
                self.locality_widening_budget_cross_count
            ),
            format!(
                "{label}_locality_widening_denial_count:{}",
                self.locality_widening_denial_count
            ),
            format!(
                "{label}_locality_bridge_slice_incompatibility_count:{}",
                self.locality_bridge_slice_incompatibility_count
            ),
        ]
    }

    fn stream_delivery_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_stream_contract_admission_count:{}",
                self.stream_contract_admission_count
            ),
            format!(
                "{label}_stream_contract_denial_count:{}",
                self.stream_contract_denial_count
            ),
            format!(
                "{label}_stream_lowered_delivery_count:{}",
                self.stream_lowered_delivery_count
            ),
            format!(
                "{label}_stream_lowered_delivery_member_count:{}",
                self.stream_lowered_delivery_member_count
            ),
            format!(
                "{label}_stream_lowered_delivery_window_width:{}",
                self.stream_lowered_delivery_window_width
            ),
            format!(
                "{label}_stream_lowered_delivery_width:{}",
                self.stream_lowered_delivery_width
            ),
            format!(
                "{label}_stream_window_width_budget_cross_count:{}",
                self.stream_window_width_budget_cross_count
            ),
            format!(
                "{label}_stream_member_width_budget_cross_count:{}",
                self.stream_member_width_budget_cross_count
            ),
        ]
    }

    fn locality_efficiency_digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}_locality_work_avoided_by_region_narrowing_count:{}",
                self.locality_work_avoided_by_region_narrowing_count
            ),
            format!(
                "{label}_locality_work_avoided_vs_broad_control_count:{}",
                self.locality_work_avoided_vs_broad_control_count
            ),
            format!(
                "{label}_locality_executor_rediscovery_count:{}",
                self.locality_executor_rediscovery_count
            ),
            format!(
                "{label}_locality_unsupported_family_rejection_count:{}",
                self.locality_unsupported_family_rejection_count
            ),
            format!(
                "{label}_locality_unsupported_predicate_rejection_count:{}",
                self.locality_unsupported_predicate_rejection_count
            ),
        ]
    }
}
