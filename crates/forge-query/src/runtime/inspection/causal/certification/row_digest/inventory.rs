use super::CausalInspectionRepresentativeRowDigestSet;

impl CausalInspectionRepresentativeRowDigestSet {
    pub fn populated_non_writeback_bridge_runtime_slot_count(&self) -> usize {
        [
            self.bridge_route_digest(),
            self.bridge_evaluation_digest(),
            self.bridge_source_materialization_digest(),
            self.bridge_structural_digest(),
            self.bridge_stream_digest(),
            self.bridge_preview_digest(),
            self.bridge_replay_digest(),
        ]
        .into_iter()
        .flatten()
        .count()
    }

    pub fn populated_bridge_runtime_slot_count(&self) -> usize {
        self.populated_non_writeback_bridge_runtime_slot_count()
            + usize::from(self.bridge_writeback_digest().is_some())
    }

    pub fn has_retained_source_structural_stream_replay_slot_coverage(&self) -> bool {
        self.bridge_source_materialization_digest().is_some()
            && self.bridge_structural_digest().is_some()
            && self.bridge_stream_digest().is_some()
            && self.bridge_replay_digest().is_some()
    }

    pub fn has_retained_source_structural_stream_writeback_replay_slot_coverage(&self) -> bool {
        self.has_retained_source_structural_stream_replay_slot_coverage()
            && self.bridge_writeback_digest().is_some()
    }

    pub fn populated_signal_slot_count(&self) -> usize {
        [
            self.signal_invalidation_digest(),
            self.signal_evaluation_digest(),
            self.signal_forensic_availability_digest(),
            self.signal_replay_cursor_digest(),
            self.signal_lineage_digest(),
            self.signal_provenance_digest(),
        ]
        .into_iter()
        .flatten()
        .count()
    }

    pub fn has_signal_evaluation_forensic_replay_lineage_provenance_reference_coverage(
        &self,
    ) -> bool {
        self.signal_evaluation_digest().is_some()
            && self.signal_forensic_availability_digest().is_some()
            && self.signal_replay_cursor_digest().is_some()
            && self.signal_lineage_digest().is_some()
            && self.signal_provenance_digest().is_some()
    }

    pub fn has_replay_posture_coverage(&self) -> bool {
        self.bridge_replay_digest().is_some() && self.replay_posture_digest().is_some()
    }
}
