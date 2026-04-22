use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BridgeSubscriptionCounterValues {
    family_registry_freeze_count: usize,
    family_count: usize,
    family_supported_slice_kind_count: usize,
    family_lookup_count: usize,
    declaration_count: usize,
    declaration_input_slice_intent_count: usize,
    declaration_normalized_slice_intent_count: usize,
    declaration_deduplicated_slice_intent_count: usize,
    declaration_rejection_count: usize,
    basis_request_count: usize,
    basis_binding_count: usize,
    basis_rejection_count: usize,
    signal_strategy_selection_count: usize,
    signal_strategy_rejection_count: usize,
    admitted_subscription_count: usize,
    lifecycle_record_count: usize,
    replay_reconstruction_count: usize,
    replay_mismatch_count: usize,
    diagnostics_bundle_count: usize,
    subscription_delivery_cost_profile_selection_count: usize,
    subscription_delivery_cost_profile_rejection_count: usize,
    subscription_delivery_over_budget_rejection_count: usize,
    subscription_delivery_density_sparse_count: usize,
    subscription_delivery_density_coalesced_count: usize,
    subscription_delivery_density_dense_restart_count: usize,
    subscription_consumer_contract_admission_count: usize,
    subscription_consumer_contract_rejection_count: usize,
    subscription_activation_count: usize,
    subscription_delivery_record_count: usize,
    subscription_delivery_member_count: usize,
    subscription_delivery_family_selection_count: usize,
    subscription_diagnostics_reference_emit_count: usize,
    subscription_rich_diagnostics_hot_path_materialization_count: usize,
    subscription_delivery_arena_reset_count: usize,
    subscription_delivery_buffer_reuse_count: usize,
    subscription_allocation_count: usize,
    subscription_clone_count: usize,
    subscription_callback_identity_scan_count: usize,
    subscription_active_registry_scan_count: usize,
    subscription_fanout_per_member_consumer_scan_count: usize,
    subscription_preview_residue_nonzero_count: usize,
    subscription_fanout_plan_admission_count: usize,
    subscription_fanout_plan_rejection_count: usize,
    subscription_fanout_layout_build_count: usize,
    subscription_fanout_consumer_binding_count: usize,
    subscription_fanout_delivery_projection_count: usize,
    subscription_fanout_delivery_projection_rejection_count: usize,
    subscription_delivery_window_seed_retention_count: usize,
    subscription_delivery_replay_seed_retention_count: usize,
    subscription_delivery_replay_readiness_inspection_count: usize,
    subscription_delivery_replay_plan_count: usize,
    subscription_delivery_replay_plan_rejection_count: usize,
    subscription_delivery_replay_retained_window_count: usize,
    subscription_delivery_replay_retained_member_count: usize,
    subscription_fanout_projection_validation_count: usize,
    subscription_fanout_projection_validation_rejection_count: usize,
    subscription_acknowledgement_frontier_admission_count: usize,
    subscription_acknowledgement_frontier_rejection_count: usize,
    subscription_checkpoint_ready_count: usize,
    subscription_checkpoint_publication_count: usize,
    subscription_checkpoint_publication_rejection_count: usize,
    subscription_duplicate_replay_policy_selection_count: usize,
    subscription_resume_admission_count: usize,
    subscription_resume_admission_rejection_count: usize,
    subscription_resume_plan_count: usize,
    subscription_raw_stream_checkpoint_rejection_count: usize,
    subscription_checkpoint_truncation_rejection_count: usize,
    subscription_continuation_index_build_count: usize,
    subscription_continuation_candidate_count: usize,
    subscription_continuation_candidate_index_lookup_count: usize,
    subscription_continuation_decision_count: usize,
    subscription_continuation_rejection_count: usize,
    subscription_branch_leak_rejection_count: usize,
    subscription_continuation_child_record_count: usize,
    subscription_continuation_full_registry_scan_count: usize,
    subscription_preview_basis_admission_count: usize,
    subscription_preview_basis_rejection_count: usize,
    subscription_preview_activation_count: usize,
    subscription_preview_discard_count: usize,
    subscription_preview_discard_rejection_count: usize,
    subscription_preview_residue_check_count: usize,
    subscription_preview_residue_scope_index_lookup_count: usize,
    subscription_preview_non_scope_registry_scan_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCounters {
    values: BridgeSubscriptionCounterValues,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCounters {
    pub fn zero() -> Self {
        Self::from_values(BridgeSubscriptionCounterValues::default())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family_registry_freeze_count: usize,
        family_count: usize,
        family_supported_slice_kind_count: usize,
        family_lookup_count: usize,
        declaration_count: usize,
        declaration_input_slice_intent_count: usize,
        declaration_normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
        declaration_rejection_count: usize,
        basis_request_count: usize,
        basis_binding_count: usize,
        basis_rejection_count: usize,
        signal_strategy_selection_count: usize,
        signal_strategy_rejection_count: usize,
        admitted_subscription_count: usize,
        lifecycle_record_count: usize,
        replay_reconstruction_count: usize,
        replay_mismatch_count: usize,
        diagnostics_bundle_count: usize,
    ) -> Self {
        Self::from_values(BridgeSubscriptionCounterValues {
            family_registry_freeze_count,
            family_count,
            family_supported_slice_kind_count,
            family_lookup_count,
            declaration_count,
            declaration_input_slice_intent_count,
            declaration_normalized_slice_intent_count,
            declaration_deduplicated_slice_intent_count,
            declaration_rejection_count,
            basis_request_count,
            basis_binding_count,
            basis_rejection_count,
            signal_strategy_selection_count,
            signal_strategy_rejection_count,
            admitted_subscription_count,
            lifecycle_record_count,
            replay_reconstruction_count,
            replay_mismatch_count,
            diagnostics_bundle_count,
            ..BridgeSubscriptionCounterValues::default()
        })
    }

    fn from_values(values: BridgeSubscriptionCounterValues) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-subscription-counters|family-registry-freeze-count:{}|",
                "family-count:{}|family-supported-slice-kind-count:{}|family-lookup-count:{}|",
                "declaration-count:{}|declaration-input-slice-intent-count:{}|",
                "declaration-normalized-slice-intent-count:{}|",
                "declaration-deduplicated-slice-intent-count:{}|",
                "declaration-rejection-count:{}|basis-request-count:{}|basis-binding-count:{}|",
                "basis-rejection-count:{}|signal-strategy-selection-count:{}|",
                "signal-strategy-rejection-count:{}|admitted-subscription-count:{}|",
                "lifecycle-record-count:{}|replay-reconstruction-count:{}|",
                "replay-mismatch-count:{}|diagnostics-bundle-count:{}|",
                "subscription-delivery-cost-profile-selection-count:{}|",
                "subscription-delivery-cost-profile-rejection-count:{}|",
                "subscription-delivery-over-budget-rejection-count:{}|",
                "subscription-delivery-density-sparse-count:{}|",
                "subscription-delivery-density-coalesced-count:{}|",
                "subscription-delivery-density-dense-restart-count:{}|",
                "subscription-consumer-contract-admission-count:{}|",
                "subscription-consumer-contract-rejection-count:{}|",
                "subscription-activation-count:{}|subscription-delivery-record-count:{}|",
                "subscription-delivery-member-count:{}|",
                "subscription-delivery-family-selection-count:{}|",
                "subscription-diagnostics-reference-emit-count:{}|",
                "subscription-rich-diagnostics-hot-path-materialization-count:{}|",
                "subscription-delivery-arena-reset-count:{}|",
                "subscription-delivery-buffer-reuse-count:{}|",
                "subscription-allocation-count:{}|subscription-clone-count:{}|",
                "subscription-callback-identity-scan-count:{}|",
                "subscription-active-registry-scan-count:{}|",
                "subscription-fanout-per-member-consumer-scan-count:{}|",
                "subscription-preview-residue-nonzero-count:{}|",
                "subscription-fanout-plan-admission-count:{}|",
                "subscription-fanout-plan-rejection-count:{}|",
                "subscription-fanout-layout-build-count:{}|",
                "subscription-fanout-consumer-binding-count:{}|",
                "subscription-fanout-delivery-projection-count:{}|",
                "subscription-fanout-delivery-projection-rejection-count:{}|",
                "subscription-delivery-window-seed-retention-count:{}|",
                "subscription-delivery-replay-seed-retention-count:{}|",
                "subscription-delivery-replay-readiness-inspection-count:{}|",
                "subscription-delivery-replay-plan-count:{}|",
                "subscription-delivery-replay-plan-rejection-count:{}|",
                "subscription-delivery-replay-retained-window-count:{}|",
                "subscription-delivery-replay-retained-member-count:{}|",
                "subscription-fanout-projection-validation-count:{}|",
                "subscription-fanout-projection-validation-rejection-count:{}|",
                "subscription-acknowledgement-frontier-admission-count:{}|",
                "subscription-acknowledgement-frontier-rejection-count:{}|",
                "subscription-checkpoint-ready-count:{}|",
                "subscription-checkpoint-publication-count:{}|",
                "subscription-checkpoint-publication-rejection-count:{}|",
                "subscription-duplicate-replay-policy-selection-count:{}|",
                "subscription-resume-admission-count:{}|",
                "subscription-resume-admission-rejection-count:{}|",
                "subscription-resume-plan-count:{}|",
                "subscription-raw-stream-checkpoint-rejection-count:{}|",
                "subscription-checkpoint-truncation-rejection-count:{}|",
                "subscription-continuation-index-build-count:{}|",
                "subscription-continuation-candidate-count:{}|",
                "subscription-continuation-candidate-index-lookup-count:{}|",
                "subscription-continuation-decision-count:{}|",
                "subscription-continuation-rejection-count:{}|",
                "subscription-branch-leak-rejection-count:{}|",
                "subscription-continuation-child-record-count:{}|",
                "subscription-continuation-full-registry-scan-count:{}|",
                "subscription-preview-basis-admission-count:{}|",
                "subscription-preview-basis-rejection-count:{}|",
                "subscription-preview-activation-count:{}|",
                "subscription-preview-discard-count:{}|",
                "subscription-preview-discard-rejection-count:{}|",
                "subscription-preview-residue-check-count:{}|",
                "subscription-preview-residue-scope-index-lookup-count:{}|",
                "subscription-preview-non-scope-registry-scan-count:{}"
            ),
            values.family_registry_freeze_count,
            values.family_count,
            values.family_supported_slice_kind_count,
            values.family_lookup_count,
            values.declaration_count,
            values.declaration_input_slice_intent_count,
            values.declaration_normalized_slice_intent_count,
            values.declaration_deduplicated_slice_intent_count,
            values.declaration_rejection_count,
            values.basis_request_count,
            values.basis_binding_count,
            values.basis_rejection_count,
            values.signal_strategy_selection_count,
            values.signal_strategy_rejection_count,
            values.admitted_subscription_count,
            values.lifecycle_record_count,
            values.replay_reconstruction_count,
            values.replay_mismatch_count,
            values.diagnostics_bundle_count,
            values.subscription_delivery_cost_profile_selection_count,
            values.subscription_delivery_cost_profile_rejection_count,
            values.subscription_delivery_over_budget_rejection_count,
            values.subscription_delivery_density_sparse_count,
            values.subscription_delivery_density_coalesced_count,
            values.subscription_delivery_density_dense_restart_count,
            values.subscription_consumer_contract_admission_count,
            values.subscription_consumer_contract_rejection_count,
            values.subscription_activation_count,
            values.subscription_delivery_record_count,
            values.subscription_delivery_member_count,
            values.subscription_delivery_family_selection_count,
            values.subscription_diagnostics_reference_emit_count,
            values.subscription_rich_diagnostics_hot_path_materialization_count,
            values.subscription_delivery_arena_reset_count,
            values.subscription_delivery_buffer_reuse_count,
            values.subscription_allocation_count,
            values.subscription_clone_count,
            values.subscription_callback_identity_scan_count,
            values.subscription_active_registry_scan_count,
            values.subscription_fanout_per_member_consumer_scan_count,
            values.subscription_preview_residue_nonzero_count,
            values.subscription_fanout_plan_admission_count,
            values.subscription_fanout_plan_rejection_count,
            values.subscription_fanout_layout_build_count,
            values.subscription_fanout_consumer_binding_count,
            values.subscription_fanout_delivery_projection_count,
            values.subscription_fanout_delivery_projection_rejection_count,
            values.subscription_delivery_window_seed_retention_count,
            values.subscription_delivery_replay_seed_retention_count,
            values.subscription_delivery_replay_readiness_inspection_count,
            values.subscription_delivery_replay_plan_count,
            values.subscription_delivery_replay_plan_rejection_count,
            values.subscription_delivery_replay_retained_window_count,
            values.subscription_delivery_replay_retained_member_count,
            values.subscription_fanout_projection_validation_count,
            values.subscription_fanout_projection_validation_rejection_count,
            values.subscription_acknowledgement_frontier_admission_count,
            values.subscription_acknowledgement_frontier_rejection_count,
            values.subscription_checkpoint_ready_count,
            values.subscription_checkpoint_publication_count,
            values.subscription_checkpoint_publication_rejection_count,
            values.subscription_duplicate_replay_policy_selection_count,
            values.subscription_resume_admission_count,
            values.subscription_resume_admission_rejection_count,
            values.subscription_resume_plan_count,
            values.subscription_raw_stream_checkpoint_rejection_count,
            values.subscription_checkpoint_truncation_rejection_count,
            values.subscription_continuation_index_build_count,
            values.subscription_continuation_candidate_count,
            values.subscription_continuation_candidate_index_lookup_count,
            values.subscription_continuation_decision_count,
            values.subscription_continuation_rejection_count,
            values.subscription_branch_leak_rejection_count,
            values.subscription_continuation_child_record_count,
            values.subscription_continuation_full_registry_scan_count,
            values.subscription_preview_basis_admission_count,
            values.subscription_preview_basis_rejection_count,
            values.subscription_preview_activation_count,
            values.subscription_preview_discard_count,
            values.subscription_preview_discard_rejection_count,
            values.subscription_preview_residue_check_count,
            values.subscription_preview_residue_scope_index_lookup_count,
            values.subscription_preview_non_scope_registry_scan_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            values,
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-counters:sha256:{digest:x}")),
        }
    }

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
    pub fn subscription_raw_stream_checkpoint_rejection_count(&self) -> usize {
        self.values
            .subscription_raw_stream_checkpoint_rejection_count
    }
    pub fn subscription_checkpoint_truncation_rejection_count(&self) -> usize {
        self.values
            .subscription_checkpoint_truncation_rejection_count
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

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

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
        posture: super::BridgeSubscriptionDeliveryDensityPosture,
    ) -> Self {
        let mut values = BridgeSubscriptionCounterValues {
            subscription_delivery_cost_profile_selection_count: 1,
            ..BridgeSubscriptionCounterValues::default()
        };
        match posture {
            super::BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery => {
                values.subscription_delivery_density_sparse_count = 1
            }
            super::BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow => {
                values.subscription_delivery_density_coalesced_count = 1
            }
            super::BridgeSubscriptionDeliveryDensityPosture::DenseRestartRequired => {
                values.subscription_delivery_density_dense_restart_count = 1
            }
            super::BridgeSubscriptionDeliveryDensityPosture::RejectedOverBudget => {}
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
}

#[cfg(test)]
mod tests {
    use super::BridgeSubscriptionCounters;

    #[test]
    fn incompatible_basis_rejection_counters_match_actual_work() {
        let counters = BridgeSubscriptionCounters::from_incompatible_basis_kind_rejection();

        assert_eq!(counters.declaration_rejection_count(), 0);
        assert_eq!(counters.basis_request_count(), 1);
        assert_eq!(counters.basis_binding_count(), 0);
        assert_eq!(counters.basis_rejection_count(), 1);
        assert_eq!(counters.signal_strategy_selection_count(), 0);
        assert_eq!(counters.signal_strategy_rejection_count(), 0);
    }
}
