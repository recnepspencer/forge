use crate::identity::hash_parts;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ActiveSubscriptionCounters {
    pub(super) active_lane_admission_count: u64,
    pub(super) active_lane_registry_lookup_count: u64,
    pub(super) active_lane_lookup_class_count: u64,
    pub(super) active_lane_linear_scan_debt_count: u64,
    pub(super) active_lane_handle_issue_count: u64,
    pub(super) active_lane_creation_count: u64,
    pub(super) active_lane_join_count: u64,
    pub(super) active_lane_join_denial_count: u64,
    pub(super) shared_lane_count: u64,
    pub(super) active_lane_linear_scan_denial_count: u64,
    pub(super) active_lane_allocation_denial_count: u64,
    pub(super) consumer_attachment_count: u64,
    pub(super) consumer_attachment_denial_count: u64,
    pub(super) fanout_width: u64,
    pub(super) fanout_delivery_count: u64,
    pub(super) affected_consumer_attachment_width: u64,
    pub(super) acknowledgement_frontier_advance_count: u64,
    pub(super) acknowledgement_receipt_mismatch_denial_count: u64,
    pub(super) acknowledgement_sequence_regression_denial_count: u64,
    pub(super) delivery_gap_notice_count: u64,
    pub(super) backpressure_denial_count: u64,
    pub(super) delivery_window_open_count: u64,
    pub(super) delivery_window_overflow_count: u64,
    pub(super) maintenance_delta_lowering_count: u64,
    pub(super) maintenance_delta_width: u64,
    pub(super) active_delivery_work_packet_count: u64,
    pub(super) active_delivery_work_packet_width: u64,
    pub(super) active_delivery_density_sparse_count: u64,
    pub(super) active_delivery_density_burst_coalesced_count: u64,
    pub(super) active_delivery_density_dense_debt_count: u64,
    pub(super) active_delivery_density_dense_denial_count: u64,
    pub(super) raw_cdc_delivery_denial_count: u64,
    pub(super) raw_bridge_invalidation_denial_count: u64,
    pub(super) delivery_batch_count: u64,
    pub(super) delivery_window_width: u64,
    pub(super) patch_group_count: u64,
    pub(super) patch_group_width: u64,
    pub(super) detail_field_patch_width: u64,
    pub(super) focused_inspector_patch_width: u64,
    pub(super) collection_membership_patch_width: u64,
    pub(super) collection_order_patch_width: u64,
    pub(super) grouped_membership_patch_width: u64,
    pub(super) bounded_materialization_scope_patch_width: u64,
    pub(super) continuation_remap_count: u64,
    pub(super) continuation_remap_width: u64,
    pub(super) continuation_remap_denial_count: u64,
    pub(super) continuation_advisory_count: u64,
    pub(super) continuation_identity_break_count: u64,
    pub(super) preview_active_lane_count: u64,
    pub(super) preview_authoritative_sharing_denial_count: u64,
    pub(super) preview_discard_residue_check_count: u64,
    pub(super) preview_residue_width: u64,
    pub(super) preview_authoritative_residue_count: u64,
    pub(super) preview_promotion_handoff_count: u64,
    pub(super) consumer_attachment_close_count: u64,
    pub(super) active_lane_close_count: u64,
    pub(super) subscription_lifecycle_closeout_count: u64,
    pub(super) subscription_lifecycle_closeout_denial_count: u64,
    pub(super) durable_checkpoint_overclaim_denial_count: u64,
    pub(super) store_backed_restart_overclaim_denial_count: u64,
    pub(super) subscription_performance_receipt_count: u64,
    pub(super) subscription_budget_consumption_width: u64,
    pub(super) subscription_budget_remaining_width: u64,
    pub(super) heap_allocation_debt_count: u64,
    pub(super) heap_allocation_denial_count: u64,
}

impl ActiveSubscriptionCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("active_lane_admission:{}", self.active_lane_admission_count),
            format!(
                "active_lane_registry_lookup:{}",
                self.active_lane_registry_lookup_count
            ),
            format!(
                "active_lane_lookup_class:{}",
                self.active_lane_lookup_class_count
            ),
            format!(
                "active_lane_linear_scan_debt:{}",
                self.active_lane_linear_scan_debt_count
            ),
            format!(
                "active_lane_handle_issue:{}",
                self.active_lane_handle_issue_count
            ),
            format!("active_lane_creation:{}", self.active_lane_creation_count),
            format!("active_lane_join:{}", self.active_lane_join_count),
            format!(
                "active_lane_join_denial:{}",
                self.active_lane_join_denial_count
            ),
            format!("shared_lane:{}", self.shared_lane_count),
            format!(
                "active_lane_linear_scan_denial:{}",
                self.active_lane_linear_scan_denial_count
            ),
            format!(
                "active_lane_allocation_denial:{}",
                self.active_lane_allocation_denial_count
            ),
            format!("consumer_attachment:{}", self.consumer_attachment_count),
            format!(
                "consumer_attachment_denial:{}",
                self.consumer_attachment_denial_count
            ),
            format!("fanout_width:{}", self.fanout_width),
            format!("fanout_delivery:{}", self.fanout_delivery_count),
            format!(
                "affected_consumer_attachment_width:{}",
                self.affected_consumer_attachment_width
            ),
            format!(
                "acknowledgement_frontier_advance:{}",
                self.acknowledgement_frontier_advance_count
            ),
            format!(
                "acknowledgement_receipt_mismatch_denial:{}",
                self.acknowledgement_receipt_mismatch_denial_count
            ),
            format!(
                "acknowledgement_sequence_regression_denial:{}",
                self.acknowledgement_sequence_regression_denial_count
            ),
            format!("delivery_gap_notice:{}", self.delivery_gap_notice_count),
            format!("backpressure_denial:{}", self.backpressure_denial_count),
            format!("delivery_window_open:{}", self.delivery_window_open_count),
            format!(
                "delivery_window_overflow:{}",
                self.delivery_window_overflow_count
            ),
            format!(
                "maintenance_delta_lowering:{}",
                self.maintenance_delta_lowering_count
            ),
            format!("maintenance_delta_width:{}", self.maintenance_delta_width),
            format!(
                "active_delivery_work_packet:{}",
                self.active_delivery_work_packet_count
            ),
            format!(
                "active_delivery_work_packet_width:{}",
                self.active_delivery_work_packet_width
            ),
            format!(
                "active_delivery_density_sparse:{}",
                self.active_delivery_density_sparse_count
            ),
            format!(
                "active_delivery_density_burst_coalesced:{}",
                self.active_delivery_density_burst_coalesced_count
            ),
            format!(
                "active_delivery_density_dense_debt:{}",
                self.active_delivery_density_dense_debt_count
            ),
            format!(
                "active_delivery_density_dense_denial:{}",
                self.active_delivery_density_dense_denial_count
            ),
            format!(
                "raw_cdc_delivery_denial:{}",
                self.raw_cdc_delivery_denial_count
            ),
            format!(
                "raw_bridge_invalidation_denial:{}",
                self.raw_bridge_invalidation_denial_count
            ),
            format!("delivery_batch:{}", self.delivery_batch_count),
            format!("delivery_window_width:{}", self.delivery_window_width),
            format!("patch_group:{}", self.patch_group_count),
            format!("patch_group_width:{}", self.patch_group_width),
            format!("detail_field_patch_width:{}", self.detail_field_patch_width),
            format!(
                "focused_inspector_patch_width:{}",
                self.focused_inspector_patch_width
            ),
            format!(
                "collection_membership_patch_width:{}",
                self.collection_membership_patch_width
            ),
            format!(
                "collection_order_patch_width:{}",
                self.collection_order_patch_width
            ),
            format!(
                "grouped_membership_patch_width:{}",
                self.grouped_membership_patch_width
            ),
            format!(
                "bounded_materialization_scope_patch_width:{}",
                self.bounded_materialization_scope_patch_width
            ),
            format!("continuation_remap:{}", self.continuation_remap_count),
            format!("continuation_remap_width:{}", self.continuation_remap_width),
            format!(
                "continuation_remap_denial:{}",
                self.continuation_remap_denial_count
            ),
            format!("continuation_advisory:{}", self.continuation_advisory_count),
            format!(
                "continuation_identity_break:{}",
                self.continuation_identity_break_count
            ),
            format!("preview_active_lane:{}", self.preview_active_lane_count),
            format!(
                "preview_authoritative_sharing_denial:{}",
                self.preview_authoritative_sharing_denial_count
            ),
            format!(
                "preview_discard_residue_check:{}",
                self.preview_discard_residue_check_count
            ),
            format!("preview_residue_width:{}", self.preview_residue_width),
            format!(
                "preview_authoritative_residue:{}",
                self.preview_authoritative_residue_count
            ),
            format!(
                "preview_promotion_handoff:{}",
                self.preview_promotion_handoff_count
            ),
            format!(
                "consumer_attachment_close:{}",
                self.consumer_attachment_close_count
            ),
            format!("active_lane_close:{}", self.active_lane_close_count),
            format!(
                "subscription_lifecycle_closeout:{}",
                self.subscription_lifecycle_closeout_count
            ),
            format!(
                "subscription_lifecycle_closeout_denial:{}",
                self.subscription_lifecycle_closeout_denial_count
            ),
            format!(
                "durable_checkpoint_overclaim_denial:{}",
                self.durable_checkpoint_overclaim_denial_count
            ),
            format!(
                "store_backed_restart_overclaim_denial:{}",
                self.store_backed_restart_overclaim_denial_count
            ),
            format!(
                "subscription_performance_receipt:{}",
                self.subscription_performance_receipt_count
            ),
            format!(
                "subscription_budget_consumption_width:{}",
                self.subscription_budget_consumption_width
            ),
            format!(
                "subscription_budget_remaining_width:{}",
                self.subscription_budget_remaining_width
            ),
            format!("heap_allocation_debt:{}", self.heap_allocation_debt_count),
            format!(
                "heap_allocation_denial:{}",
                self.heap_allocation_denial_count
            ),
        ])
    }

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
