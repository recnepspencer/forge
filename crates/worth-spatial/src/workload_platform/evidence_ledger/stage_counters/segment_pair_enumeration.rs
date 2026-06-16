use super::WorkloadEvidenceStageCounters;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationCounters;

impl WorkloadEvidenceStageCounters {
    pub fn boolean_segment_pair_enumeration(
        counters: PlanarBooleanSegmentPairEnumerationCounters,
    ) -> Self {
        Self {
            boolean_segment_pair_enumeration_count: 1,
            boolean_segment_pair_left_segment_count: counters.left_segment_count(),
            boolean_segment_pair_right_segment_count: counters.right_segment_count(),
            boolean_segment_pair_expected_breadth: counters.expected_pair_breadth(),
            boolean_segment_pair_emitted_breadth: counters.emitted_pair_breadth(),
            boolean_segment_pair_skipped_count: counters.skipped_pair_count(),
            boolean_segment_pair_query_index_candidate_count: counters
                .query_index_candidate_count(),
            boolean_segment_pair_query_index_culled_count: counters.query_index_culled_pair_count(),
            boolean_segment_pair_envelope_expanded_count: counters.envelope_expanded_pair_count(),
            boolean_segment_pair_broad_phase_comparison_count: counters
                .broad_phase_comparison_count(),
            boolean_segment_pair_degenerate_skip_count: counters.degenerate_skip_count(),
            boolean_segment_pair_fallback_used_count: usize::from(counters.fallback_used()),
            ..Self::default()
        }
    }

    pub fn boolean_segment_pair_enumeration_count(self) -> usize {
        self.boolean_segment_pair_enumeration_count
    }

    pub fn boolean_segment_pair_left_segment_count(self) -> usize {
        self.boolean_segment_pair_left_segment_count
    }

    pub fn boolean_segment_pair_right_segment_count(self) -> usize {
        self.boolean_segment_pair_right_segment_count
    }

    pub fn boolean_segment_pair_expected_breadth(self) -> usize {
        self.boolean_segment_pair_expected_breadth
    }

    pub fn boolean_segment_pair_emitted_breadth(self) -> usize {
        self.boolean_segment_pair_emitted_breadth
    }

    pub fn boolean_segment_pair_skipped_count(self) -> usize {
        self.boolean_segment_pair_skipped_count
    }

    pub fn boolean_segment_pair_query_index_candidate_count(self) -> usize {
        self.boolean_segment_pair_query_index_candidate_count
    }

    pub fn boolean_segment_pair_query_index_culled_count(self) -> usize {
        self.boolean_segment_pair_query_index_culled_count
    }

    pub fn boolean_segment_pair_envelope_expanded_count(self) -> usize {
        self.boolean_segment_pair_envelope_expanded_count
    }

    pub fn boolean_segment_pair_broad_phase_comparison_count(self) -> usize {
        self.boolean_segment_pair_broad_phase_comparison_count
    }

    pub fn boolean_segment_pair_degenerate_skip_count(self) -> usize {
        self.boolean_segment_pair_degenerate_skip_count
    }

    pub fn boolean_segment_pair_fallback_used_count(self) -> usize {
        self.boolean_segment_pair_fallback_used_count
    }
}
