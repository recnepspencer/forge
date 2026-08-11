use super::ActiveSubscriptionCounters;

impl ActiveSubscriptionCounters {
    pub fn active_lane_admission_count(&self) -> u64 {
        self.active_lane_admission_count
    }

    pub fn active_lane_registry_lookup_count(&self) -> u64 {
        self.active_lane_registry_lookup_count
    }

    pub fn active_lane_lookup_class_count(&self) -> u64 {
        self.active_lane_lookup_class_count
    }

    pub fn active_lane_linear_scan_debt_count(&self) -> u64 {
        self.active_lane_linear_scan_debt_count
    }

    pub fn active_lane_handle_issue_count(&self) -> u64 {
        self.active_lane_handle_issue_count
    }

    pub fn active_lane_creation_count(&self) -> u64 {
        self.active_lane_creation_count
    }

    pub fn active_lane_join_count(&self) -> u64 {
        self.active_lane_join_count
    }

    pub fn active_lane_join_denial_count(&self) -> u64 {
        self.active_lane_join_denial_count
    }

    pub fn shared_lane_count(&self) -> u64 {
        self.shared_lane_count
    }

    pub fn active_lane_linear_scan_denial_count(&self) -> u64 {
        self.active_lane_linear_scan_denial_count
    }

    pub fn active_lane_allocation_denial_count(&self) -> u64 {
        self.active_lane_allocation_denial_count
    }

    pub fn consumer_attachment_count(&self) -> u64 {
        self.consumer_attachment_count
    }

    pub fn consumer_attachment_denial_count(&self) -> u64 {
        self.consumer_attachment_denial_count
    }

    pub fn fanout_width(&self) -> u64 {
        self.fanout_width
    }

    pub fn fanout_delivery_count(&self) -> u64 {
        self.fanout_delivery_count
    }

    pub fn affected_consumer_attachment_width(&self) -> u64 {
        self.affected_consumer_attachment_width
    }

    pub fn acknowledgement_frontier_advance_count(&self) -> u64 {
        self.acknowledgement_frontier_advance_count
    }

    pub fn acknowledgement_receipt_mismatch_denial_count(&self) -> u64 {
        self.acknowledgement_receipt_mismatch_denial_count
    }

    pub fn acknowledgement_sequence_regression_denial_count(&self) -> u64 {
        self.acknowledgement_sequence_regression_denial_count
    }

    pub fn delivery_gap_notice_count(&self) -> u64 {
        self.delivery_gap_notice_count
    }

    pub fn backpressure_denial_count(&self) -> u64 {
        self.backpressure_denial_count
    }

    pub fn delivery_window_open_count(&self) -> u64 {
        self.delivery_window_open_count
    }

    pub fn delivery_window_overflow_count(&self) -> u64 {
        self.delivery_window_overflow_count
    }

    pub fn maintenance_delta_lowering_count(&self) -> u64 {
        self.maintenance_delta_lowering_count
    }

    pub fn maintenance_delta_width(&self) -> u64 {
        self.maintenance_delta_width
    }

    pub fn active_delivery_work_packet_count(&self) -> u64 {
        self.active_delivery_work_packet_count
    }

    pub fn active_delivery_work_packet_width(&self) -> u64 {
        self.active_delivery_work_packet_width
    }

    pub fn active_delivery_density_sparse_count(&self) -> u64 {
        self.active_delivery_density_sparse_count
    }

    pub fn active_delivery_density_burst_coalesced_count(&self) -> u64 {
        self.active_delivery_density_burst_coalesced_count
    }

    pub fn active_delivery_density_dense_debt_count(&self) -> u64 {
        self.active_delivery_density_dense_debt_count
    }

    pub fn active_delivery_density_dense_denial_count(&self) -> u64 {
        self.active_delivery_density_dense_denial_count
    }

    pub fn raw_cdc_delivery_denial_count(&self) -> u64 {
        self.raw_cdc_delivery_denial_count
    }

    pub fn raw_bridge_invalidation_denial_count(&self) -> u64 {
        self.raw_bridge_invalidation_denial_count
    }

    pub fn delivery_batch_count(&self) -> u64 {
        self.delivery_batch_count
    }

    pub fn delivery_window_width(&self) -> u64 {
        self.delivery_window_width
    }

    pub fn patch_group_count(&self) -> u64 {
        self.patch_group_count
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn detail_field_patch_width(&self) -> u64 {
        self.detail_field_patch_width
    }

    pub fn focused_inspector_patch_width(&self) -> u64 {
        self.focused_inspector_patch_width
    }

    pub fn collection_membership_patch_width(&self) -> u64 {
        self.collection_membership_patch_width
    }

    pub fn collection_order_patch_width(&self) -> u64 {
        self.collection_order_patch_width
    }

    pub fn grouped_membership_patch_width(&self) -> u64 {
        self.grouped_membership_patch_width
    }

    pub fn bounded_materialization_scope_patch_width(&self) -> u64 {
        self.bounded_materialization_scope_patch_width
    }

    pub fn continuation_remap_count(&self) -> u64 {
        self.continuation_remap_count
    }

    pub fn continuation_remap_width(&self) -> u64 {
        self.continuation_remap_width
    }

    pub fn continuation_remap_denial_count(&self) -> u64 {
        self.continuation_remap_denial_count
    }

    pub fn continuation_advisory_count(&self) -> u64 {
        self.continuation_advisory_count
    }

    pub fn continuation_identity_break_count(&self) -> u64 {
        self.continuation_identity_break_count
    }

    pub fn preview_active_lane_count(&self) -> u64 {
        self.preview_active_lane_count
    }

    pub fn preview_authoritative_sharing_denial_count(&self) -> u64 {
        self.preview_authoritative_sharing_denial_count
    }

    pub fn preview_discard_residue_check_count(&self) -> u64 {
        self.preview_discard_residue_check_count
    }

    pub fn preview_residue_width(&self) -> u64 {
        self.preview_residue_width
    }

    pub fn preview_authoritative_residue_count(&self) -> u64 {
        self.preview_authoritative_residue_count
    }

    pub fn preview_promotion_handoff_count(&self) -> u64 {
        self.preview_promotion_handoff_count
    }

    pub fn consumer_attachment_close_count(&self) -> u64 {
        self.consumer_attachment_close_count
    }

    pub fn active_lane_close_count(&self) -> u64 {
        self.active_lane_close_count
    }

    pub fn subscription_lifecycle_closeout_count(&self) -> u64 {
        self.subscription_lifecycle_closeout_count
    }

    pub fn subscription_lifecycle_closeout_denial_count(&self) -> u64 {
        self.subscription_lifecycle_closeout_denial_count
    }

    pub fn durable_checkpoint_overclaim_denial_count(&self) -> u64 {
        self.durable_checkpoint_overclaim_denial_count
    }

    pub fn store_backed_restart_overclaim_denial_count(&self) -> u64 {
        self.store_backed_restart_overclaim_denial_count
    }

    pub fn subscription_performance_receipt_count(&self) -> u64 {
        self.subscription_performance_receipt_count
    }

    pub fn subscription_budget_consumption_width(&self) -> u64 {
        self.subscription_budget_consumption_width
    }

    pub fn subscription_budget_remaining_width(&self) -> u64 {
        self.subscription_budget_remaining_width
    }

    pub fn heap_allocation_debt_count(&self) -> u64 {
        self.heap_allocation_debt_count
    }

    pub fn heap_allocation_denial_count(&self) -> u64 {
        self.heap_allocation_denial_count
    }
}
