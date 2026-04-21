use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct DeltaLayoutCounters {
    branch_create_count: AtomicU64,
    branch_base_reuse_count: AtomicU64,
    branch_base_copy_count: AtomicU64,
    branch_hidden_full_base_materialization_count: AtomicU64,
    branch_delta_read_count: AtomicU64,
    branch_delta_layers_traversed_count: AtomicU64,
    branch_delta_read_record_count: AtomicU64,
    branch_delta_replay_commit_count: AtomicU64,
    branch_delta_authority_replay_fallback_count: AtomicU64,
    branch_delta_rewrite_count: AtomicU64,
    branch_delta_rewrite_layers_replaced_count: AtomicU64,
    branch_delta_rewrite_record_count: AtomicU64,
    branch_delta_hidden_full_stack_rewrite_count: AtomicU64,
    branch_delta_merge_path_search_count: AtomicU64,
    branch_delta_rebuild_count: AtomicU64,
    branch_delta_rebuild_record_count: AtomicU64,
    branch_delta_integrity_failure_count: AtomicU64,
    concurrent_artifact_boundary_rejection_count: AtomicU64,
    aspect_layout_plan_count: AtomicU64,
    aspect_layout_admitted_count: AtomicU64,
    aspect_layout_fallback_count: AtomicU64,
    aspect_layout_rejected_count: AtomicU64,
    aspect_layout_slice_read_count: AtomicU64,
    aspect_layout_block_decode_count: AtomicU64,
    aspect_layout_control_replay_breadth: AtomicU64,
    aspect_layout_whole_state_fallback_count: AtomicU64,
    structural_block_lookup_count: AtomicU64,
    structural_block_reuse_admission_count: AtomicU64,
    structural_block_reuse_hit_count: AtomicU64,
    structural_block_reuse_miss_count: AtomicU64,
    chunk_model_freeze_count: AtomicU64,
    physical_chunk_export_count: AtomicU64,
    physical_chunk_width_count: AtomicU64,
    physical_chunk_determinism_violation_count: AtomicU64,
    milestone_6_proof_only_prepare_count: AtomicU64,
    milestone_6_on_demand_materialize_count: AtomicU64,
    milestone_6_policy_eager_resolution_count: AtomicU64,
    milestone_6_policy_eager_publish_count: AtomicU64,
    milestone_6_policy_eager_reuse_existing_count: AtomicU64,
    milestone_7_layout_reference_admission_count: AtomicU64,
    milestone_9_physical_chunk_reference_admission_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_branch_create(&self) {
        self.delta_layout
            .branch_create_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_branch_base_reuse(&self) {
        self.delta_layout
            .branch_base_reuse_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_delta_read(
        &self,
        layers_traversed: usize,
        read_record_count: usize,
        replay_commit_count: usize,
        used_authority_replay_fallback: bool,
    ) {
        self.delta_layout
            .branch_delta_read_count
            .fetch_add(1, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_layers_traversed_count
            .fetch_add(layers_traversed as u64, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_read_record_count
            .fetch_add(read_record_count as u64, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_replay_commit_count
            .fetch_add(replay_commit_count as u64, Ordering::Relaxed);
        if used_authority_replay_fallback {
            self.delta_layout
                .branch_delta_authority_replay_fallback_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_branch_delta_merge_path_search(&self) {
        self.delta_layout
            .branch_delta_merge_path_search_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_branch_delta_integrity_failure(&self) {
        self.delta_layout
            .branch_delta_integrity_failure_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_delta_rewrite(
        &self,
        replaced_layer_count: usize,
        rewrite_record_count: usize,
        used_hidden_full_stack_rewrite: bool,
    ) {
        self.delta_layout
            .branch_delta_rewrite_count
            .fetch_add(1, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_rewrite_layers_replaced_count
            .fetch_add(replaced_layer_count as u64, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_rewrite_record_count
            .fetch_add(rewrite_record_count as u64, Ordering::Relaxed);
        if used_hidden_full_stack_rewrite {
            self.delta_layout
                .branch_delta_hidden_full_stack_rewrite_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_branch_delta_rebuild(&self, rebuilt_record_count: usize) {
        self.delta_layout
            .branch_delta_rebuild_count
            .fetch_add(1, Ordering::Relaxed);
        self.delta_layout
            .branch_delta_rebuild_record_count
            .fetch_add(rebuilt_record_count as u64, Ordering::Relaxed);
    }

    pub fn record_aspect_layout_plan(
        &self,
        admitted: bool,
        fallback: bool,
        rejected: bool,
        slice_read_count: usize,
        block_decode_count: usize,
        control_replay_breadth: usize,
    ) {
        self.delta_layout
            .aspect_layout_plan_count
            .fetch_add(1, Ordering::Relaxed);
        if admitted {
            self.delta_layout
                .aspect_layout_admitted_count
                .fetch_add(1, Ordering::Relaxed);
        }
        if fallback {
            self.delta_layout
                .aspect_layout_fallback_count
                .fetch_add(1, Ordering::Relaxed);
        }
        if rejected {
            self.delta_layout
                .aspect_layout_rejected_count
                .fetch_add(1, Ordering::Relaxed);
        }
        self.delta_layout
            .aspect_layout_slice_read_count
            .fetch_add(slice_read_count as u64, Ordering::Relaxed);
        self.delta_layout
            .aspect_layout_block_decode_count
            .fetch_add(block_decode_count as u64, Ordering::Relaxed);
        self.delta_layout
            .aspect_layout_control_replay_breadth
            .fetch_add(control_replay_breadth as u64, Ordering::Relaxed);
    }

    pub fn record_structural_block_lookup(&self, hit: bool) {
        self.delta_layout
            .structural_block_lookup_count
            .fetch_add(1, Ordering::Relaxed);
        if hit {
            self.delta_layout
                .structural_block_reuse_hit_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.delta_layout
                .structural_block_reuse_miss_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_structural_block_reuse_admission(&self) {
        self.delta_layout
            .structural_block_reuse_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_chunk_model_freeze(&self) {
        self.delta_layout
            .chunk_model_freeze_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_physical_chunk_export(&self, chunk_width: u64) {
        self.delta_layout
            .physical_chunk_export_count
            .fetch_add(1, Ordering::Relaxed);
        self.delta_layout
            .physical_chunk_width_count
            .fetch_add(chunk_width, Ordering::Relaxed);
    }
    pub fn record_physical_chunk_determinism_violation(&self) {
        self.delta_layout
            .physical_chunk_determinism_violation_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_6_proof_only_prepare(&self) {
        self.delta_layout
            .milestone_6_proof_only_prepare_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_6_on_demand_materialize(&self) {
        self.delta_layout
            .milestone_6_on_demand_materialize_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_6_policy_eager_resolution(&self) {
        self.delta_layout
            .milestone_6_policy_eager_resolution_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_6_policy_eager_publish(&self) {
        self.delta_layout
            .milestone_6_policy_eager_publish_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_6_policy_eager_reuse_existing(&self) {
        self.delta_layout
            .milestone_6_policy_eager_reuse_existing_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_7_layout_reference_admission(&self) {
        self.delta_layout
            .milestone_7_layout_reference_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_milestone_9_physical_chunk_reference_admission(&self) {
        self.delta_layout
            .milestone_9_physical_chunk_reference_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }
}

pub(super) fn write_snapshot(counters: &DeltaLayoutCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(branch_create_count);
    load!(branch_base_reuse_count);
    load!(branch_base_copy_count);
    load!(branch_hidden_full_base_materialization_count);
    load!(branch_delta_read_count);
    load!(branch_delta_layers_traversed_count);
    load!(branch_delta_read_record_count);
    load!(branch_delta_replay_commit_count);
    load!(branch_delta_authority_replay_fallback_count);
    load!(branch_delta_rewrite_count);
    load!(branch_delta_rewrite_layers_replaced_count);
    load!(branch_delta_rewrite_record_count);
    load!(branch_delta_hidden_full_stack_rewrite_count);
    load!(branch_delta_merge_path_search_count);
    load!(branch_delta_rebuild_count);
    load!(branch_delta_rebuild_record_count);
    load!(branch_delta_integrity_failure_count);
    load!(concurrent_artifact_boundary_rejection_count);
    load!(aspect_layout_plan_count);
    load!(aspect_layout_admitted_count);
    load!(aspect_layout_fallback_count);
    load!(aspect_layout_rejected_count);
    load!(aspect_layout_slice_read_count);
    load!(aspect_layout_block_decode_count);
    load!(aspect_layout_control_replay_breadth);
    load!(aspect_layout_whole_state_fallback_count);
    load!(structural_block_lookup_count);
    load!(structural_block_reuse_admission_count);
    load!(structural_block_reuse_hit_count);
    load!(structural_block_reuse_miss_count);
    load!(chunk_model_freeze_count);
    load!(physical_chunk_export_count);
    load!(physical_chunk_width_count);
    load!(physical_chunk_determinism_violation_count);
    load!(milestone_6_proof_only_prepare_count);
    load!(milestone_6_on_demand_materialize_count);
    load!(milestone_6_policy_eager_resolution_count);
    load!(milestone_6_policy_eager_publish_count);
    load!(milestone_6_policy_eager_reuse_existing_count);
    load!(milestone_7_layout_reference_admission_count);
    load!(milestone_9_physical_chunk_reference_admission_count);
}
