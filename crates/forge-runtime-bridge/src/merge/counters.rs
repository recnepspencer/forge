#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeMergeCounters {
    merge_history_declaration_count: usize,
    merge_history_contract_count: usize,
    merge_parent_count: usize,
    merge_supported_class_count: usize,
    merge_unsupported_class_count: usize,
    merge_parent_order_rejection_count: usize,
    merge_history_segment_scan_count: usize,
    merge_causal_frontier_count: usize,
    merge_causal_frontier_lookup_count: usize,
    merge_policy_outcome_count: usize,
    merge_packet_count: usize,
    merge_routing_result_count: usize,
    merge_continuity_count: usize,
    merge_continuity_denial_count: usize,
    merge_lineage_resolution_width: usize,
    merge_candidate_cohort_width: usize,
    merge_structural_consult_width: usize,
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

impl BridgeMergeCounters {
    pub(crate) fn for_contract(parent_count: usize, candidate_cohort_width: usize) -> Self {
        Self {
            merge_history_declaration_count: 1,
            merge_history_contract_count: 1,
            merge_parent_count: parent_count,
            merge_history_segment_scan_count: 1,
            merge_candidate_cohort_width: candidate_cohort_width,
            ..Self::default()
        }
    }

    pub fn merge_history_declaration_count(&self) -> usize {
        self.merge_history_declaration_count
    }

    pub fn merge_history_contract_count(&self) -> usize {
        self.merge_history_contract_count
    }

    pub fn merge_parent_count(&self) -> usize {
        self.merge_parent_count
    }

    pub fn merge_supported_class_count(&self) -> usize {
        self.merge_supported_class_count
    }

    pub fn merge_unsupported_class_count(&self) -> usize {
        self.merge_unsupported_class_count
    }

    pub fn merge_parent_order_rejection_count(&self) -> usize {
        self.merge_parent_order_rejection_count
    }

    pub fn merge_history_segment_scan_count(&self) -> usize {
        self.merge_history_segment_scan_count
    }

    pub fn merge_causal_frontier_count(&self) -> usize {
        self.merge_causal_frontier_count
    }

    pub fn merge_causal_frontier_lookup_count(&self) -> usize {
        self.merge_causal_frontier_lookup_count
    }

    pub fn merge_policy_outcome_count(&self) -> usize {
        self.merge_policy_outcome_count
    }

    pub fn merge_packet_count(&self) -> usize {
        self.merge_packet_count
    }

    pub fn merge_routing_result_count(&self) -> usize {
        self.merge_routing_result_count
    }

    pub fn merge_continuity_count(&self) -> usize {
        self.merge_continuity_count
    }

    pub fn merge_continuity_denial_count(&self) -> usize {
        self.merge_continuity_denial_count
    }

    pub fn merge_lineage_resolution_width(&self) -> usize {
        self.merge_lineage_resolution_width
    }

    pub fn merge_candidate_cohort_width(&self) -> usize {
        self.merge_candidate_cohort_width
    }

    pub fn merge_structural_consult_width(&self) -> usize {
        self.merge_structural_consult_width
    }

    pub fn merge_remap_publication_count(&self) -> usize {
        self.merge_remap_publication_count
    }

    pub fn merge_deletion_class_count(&self) -> usize {
        self.merge_deletion_class_count
    }

    pub fn merge_topology_rewire_class_count(&self) -> usize {
        self.merge_topology_rewire_class_count
    }

    pub fn merge_structural_contradiction_count(&self) -> usize {
        self.merge_structural_contradiction_count
    }

    pub fn merge_explanation_request_count(&self) -> usize {
        self.merge_explanation_request_count
    }

    pub fn merge_replay_request_count(&self) -> usize {
        self.merge_replay_request_count
    }

    pub fn merge_replay_mismatch_count(&self) -> usize {
        self.merge_replay_mismatch_count
    }

    pub fn merge_widened_scan_count(&self) -> usize {
        self.merge_widened_scan_count
    }

    pub fn digest_computation_count(&self) -> usize {
        self.digest_computation_count
    }

    pub fn digest_input_bytes(&self) -> usize {
        self.digest_input_bytes
    }

    pub(crate) fn with_supported_class(mut self) -> Self {
        self.merge_supported_class_count += 1;
        self
    }

    pub(crate) fn with_causal_frontier_lookup(mut self) -> Self {
        self.merge_causal_frontier_count += 1;
        self.merge_causal_frontier_lookup_count += 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_parent_order_rejection(mut self) -> Self {
        self.merge_parent_order_rejection_count += 1;
        self
    }

    pub(crate) fn with_policy_outcome(mut self) -> Self {
        self.merge_policy_outcome_count += 1;
        self
    }

    pub(crate) fn with_packet(mut self) -> Self {
        self.merge_packet_count += 1;
        self
    }

    pub(crate) fn with_routing_result(mut self) -> Self {
        self.merge_routing_result_count += 1;
        self
    }

    pub(crate) fn with_continuity(mut self) -> Self {
        self.merge_continuity_count += 1;
        self
    }

    pub(crate) fn with_continuity_denial(mut self) -> Self {
        self.merge_continuity_denial_count += 1;
        self
    }

    pub(crate) fn with_lineage_resolution_width(mut self, width: usize) -> Self {
        self.merge_lineage_resolution_width += width;
        self
    }

    pub(crate) fn with_structural_consult_width(mut self, width: usize) -> Self {
        self.merge_structural_consult_width += width;
        self
    }

    pub(crate) fn with_remap_publication(mut self) -> Self {
        self.merge_remap_publication_count += 1;
        self
    }

    pub(crate) fn with_deletion_class(mut self) -> Self {
        self.merge_deletion_class_count += 1;
        self
    }

    pub(crate) fn with_topology_rewire_class(mut self) -> Self {
        self.merge_topology_rewire_class_count += 1;
        self
    }

    pub(crate) fn with_structural_contradiction(mut self) -> Self {
        self.merge_structural_contradiction_count += 1;
        self
    }

    pub(crate) fn with_explanation_request(mut self) -> Self {
        self.merge_explanation_request_count += 1;
        self
    }

    pub(crate) fn with_replay_request(mut self) -> Self {
        self.merge_replay_request_count += 1;
        self
    }

    pub(crate) fn with_replay_mismatch(mut self) -> Self {
        self.merge_replay_mismatch_count += 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_widened_scan(mut self) -> Self {
        self.merge_widened_scan_count += 1;
        self
    }

    pub(crate) fn with_digest(mut self, input_bytes: usize) -> Self {
        self.digest_computation_count += 1;
        self.digest_input_bytes += input_bytes;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::BridgeMergeCounters;

    #[test]
    fn merge_counters_track_structural_contradictions() {
        let counters = BridgeMergeCounters::default().with_structural_contradiction();

        assert_eq!(counters.merge_structural_contradiction_count(), 1);
    }

    #[test]
    fn merge_counters_track_replay_and_rejection_proof_surfaces() {
        let counters = BridgeMergeCounters::default()
            .with_parent_order_rejection()
            .with_remap_publication()
            .with_explanation_request()
            .with_replay_request()
            .with_replay_mismatch()
            .with_widened_scan();

        assert_eq!(counters.merge_parent_order_rejection_count(), 1);
        assert_eq!(counters.merge_remap_publication_count(), 1);
        assert_eq!(counters.merge_explanation_request_count(), 1);
        assert_eq!(counters.merge_replay_request_count(), 1);
        assert_eq!(counters.merge_replay_mismatch_count(), 1);
        assert_eq!(counters.merge_widened_scan_count(), 1);
    }
}
