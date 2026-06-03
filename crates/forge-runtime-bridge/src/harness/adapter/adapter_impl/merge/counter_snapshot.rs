#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MergeHarnessCounterSnapshot {
    merge_declaration_count: usize,
    merge_contract_count: usize,
    merge_parent_count: usize,
    merge_supported_class_count: usize,
    merge_unsupported_class_count: usize,
    merge_parent_order_rejection_count: usize,
    merge_causal_frontier_count: usize,
    merge_policy_outcome_count: usize,
    merge_history_packet_count: usize,
    merge_routing_result_count: usize,
    merge_lineage_resolution_width: usize,
    merge_candidate_cohort_width: usize,
    merge_structural_consult_width: usize,
    merge_causal_frontier_lookup_count: usize,
    merge_history_segment_scan_count: usize,
    merge_continuity_count: usize,
    merge_continuity_denial_count: usize,
    merge_remap_publication_count: usize,
    merge_deletion_class_count: usize,
    merge_topology_rewire_class_count: usize,
    merge_structural_contradiction_count: usize,
    merge_explanation_request_count: usize,
    merge_replay_request_count: usize,
    merge_replay_mismatch_count: usize,
    merge_widened_scan_count: usize,
    digest_computation_count: usize,
    digest_input_bytes: usize,
}

impl MergeHarnessCounterSnapshot {
    pub(super) fn from_counters(counters: &crate::facade::BridgeMergeCounters) -> Self {
        Self {
            merge_declaration_count: counters.merge_history_declaration_count(),
            merge_contract_count: counters.merge_history_contract_count(),
            merge_parent_count: counters.merge_parent_count(),
            merge_supported_class_count: counters.merge_supported_class_count(),
            merge_unsupported_class_count: counters.merge_unsupported_class_count(),
            merge_parent_order_rejection_count: counters.merge_parent_order_rejection_count(),
            merge_causal_frontier_count: counters.merge_causal_frontier_count(),
            merge_policy_outcome_count: counters.merge_policy_outcome_count(),
            merge_history_packet_count: counters.merge_packet_count(),
            merge_routing_result_count: counters.merge_routing_result_count(),
            merge_lineage_resolution_width: counters.merge_lineage_resolution_width(),
            merge_candidate_cohort_width: counters.merge_candidate_cohort_width(),
            merge_structural_consult_width: counters.merge_structural_consult_width(),
            merge_causal_frontier_lookup_count: counters.merge_causal_frontier_lookup_count(),
            merge_history_segment_scan_count: counters.merge_history_segment_scan_count(),
            merge_continuity_count: counters.merge_continuity_count(),
            merge_continuity_denial_count: counters.merge_continuity_denial_count(),
            merge_remap_publication_count: counters.merge_remap_publication_count(),
            merge_deletion_class_count: counters.merge_deletion_class_count(),
            merge_topology_rewire_class_count: counters.merge_topology_rewire_class_count(),
            merge_structural_contradiction_count: counters.merge_structural_contradiction_count(),
            merge_explanation_request_count: counters.merge_explanation_request_count(),
            merge_replay_request_count: counters.merge_replay_request_count(),
            merge_replay_mismatch_count: counters.merge_replay_mismatch_count(),
            merge_widened_scan_count: counters.merge_widened_scan_count(),
            digest_computation_count: counters.digest_computation_count(),
            digest_input_bytes: counters.digest_input_bytes(),
        }
    }

    pub(super) fn merge_declaration_count(&self) -> usize {
        self.merge_declaration_count
    }

    pub(super) fn merge_contract_count(&self) -> usize {
        self.merge_contract_count
    }

    pub(super) fn merge_parent_count(&self) -> usize {
        self.merge_parent_count
    }

    pub(super) fn merge_supported_class_count(&self) -> usize {
        self.merge_supported_class_count
    }

    pub(super) fn merge_unsupported_class_count(&self) -> usize {
        self.merge_unsupported_class_count
    }

    pub(super) fn merge_parent_order_rejection_count(&self) -> usize {
        self.merge_parent_order_rejection_count
    }

    pub(super) fn merge_causal_frontier_count(&self) -> usize {
        self.merge_causal_frontier_count
    }

    pub(super) fn merge_policy_outcome_count(&self) -> usize {
        self.merge_policy_outcome_count
    }

    pub(super) fn merge_history_packet_count(&self) -> usize {
        self.merge_history_packet_count
    }

    pub(super) fn merge_routing_result_count(&self) -> usize {
        self.merge_routing_result_count
    }

    pub(super) fn merge_lineage_resolution_width(&self) -> usize {
        self.merge_lineage_resolution_width
    }

    pub(super) fn merge_candidate_cohort_width(&self) -> usize {
        self.merge_candidate_cohort_width
    }

    pub(super) fn merge_structural_consult_width(&self) -> usize {
        self.merge_structural_consult_width
    }

    pub(super) fn merge_causal_frontier_lookup_count(&self) -> usize {
        self.merge_causal_frontier_lookup_count
    }

    pub(super) fn merge_history_segment_scan_count(&self) -> usize {
        self.merge_history_segment_scan_count
    }

    pub(super) fn merge_continuity_count(&self) -> usize {
        self.merge_continuity_count
    }

    pub(super) fn merge_continuity_denial_count(&self) -> usize {
        self.merge_continuity_denial_count
    }

    pub(super) fn merge_remap_publication_count(&self) -> usize {
        self.merge_remap_publication_count
    }

    pub(super) fn merge_deletion_class_count(&self) -> usize {
        self.merge_deletion_class_count
    }

    pub(super) fn merge_topology_rewire_class_count(&self) -> usize {
        self.merge_topology_rewire_class_count
    }

    pub(super) fn merge_structural_contradiction_count(&self) -> usize {
        self.merge_structural_contradiction_count
    }

    pub(super) fn merge_explanation_request_count(&self) -> usize {
        self.merge_explanation_request_count
    }

    pub(super) fn merge_replay_request_count(&self) -> usize {
        self.merge_replay_request_count
    }

    pub(super) fn merge_replay_mismatch_count(&self) -> usize {
        self.merge_replay_mismatch_count
    }

    pub(super) fn merge_widened_scan_count(&self) -> usize {
        self.merge_widened_scan_count
    }

    pub(super) fn digest_computation_count(&self) -> usize {
        self.digest_computation_count
    }

    pub(super) fn digest_input_bytes(&self) -> usize {
        self.digest_input_bytes
    }
}
