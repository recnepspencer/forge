#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLaneParityCounters {
    lane_transition_count: usize,
    semantic_reference_count: usize,
    query_binding_checked_count: usize,
    query_rebind_receipt_count: usize,
    semantic_mismatch_count: usize,
    visual_only_evidence_rejected_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    frame_execution_count: usize,
}

impl WorthUiLaneParityCounters {
    pub(crate) fn record_lane_transition(&mut self) {
        self.lane_transition_count += 1;
    }

    pub(crate) fn record_semantic_reference(&mut self) {
        self.semantic_reference_count += 1;
    }

    pub(crate) fn record_query_binding_checked(&mut self) {
        self.query_binding_checked_count += 1;
    }

    pub(crate) fn record_query_rebind_receipt(&mut self) {
        self.query_rebind_receipt_count += 1;
    }

    pub(crate) fn record_semantic_mismatch(&mut self) {
        self.semantic_mismatch_count += 1;
    }

    pub(crate) fn record_visual_only_evidence_rejected(&mut self) {
        self.visual_only_evidence_rejected_count += 1;
    }

    pub fn lane_transition_count(self) -> usize {
        self.lane_transition_count
    }

    pub fn semantic_reference_count(self) -> usize {
        self.semantic_reference_count
    }

    pub fn query_binding_checked_count(self) -> usize {
        self.query_binding_checked_count
    }

    pub fn query_rebind_receipt_count(self) -> usize {
        self.query_rebind_receipt_count
    }

    pub fn semantic_mismatch_count(self) -> usize {
        self.semantic_mismatch_count
    }

    pub fn visual_only_evidence_rejected_count(self) -> usize {
        self.visual_only_evidence_rejected_count
    }

    pub fn source_parse_count(self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(self) -> usize {
        self.registry_lookup_count
    }

    pub fn frame_execution_count(self) -> usize {
        self.frame_execution_count
    }
}
