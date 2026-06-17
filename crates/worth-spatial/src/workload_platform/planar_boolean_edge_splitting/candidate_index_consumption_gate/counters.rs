use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationCounters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanCandidateIndexConsumptionCounters {
    expected_pair_breadth: usize,
    indexed_candidate_pair_count: usize,
    culled_pair_count: usize,
    emitted_pair_count: usize,
    fallback_used: bool,
}

impl PlanarBooleanCandidateIndexConsumptionCounters {
    pub(crate) fn from_segment_pair_counters(
        counters: PlanarBooleanSegmentPairEnumerationCounters,
    ) -> Self {
        Self {
            expected_pair_breadth: counters.expected_pair_breadth(),
            indexed_candidate_pair_count: counters.query_index_candidate_count(),
            culled_pair_count: counters.query_index_culled_pair_count(),
            emitted_pair_count: counters.emitted_pair_breadth(),
            fallback_used: counters.fallback_used(),
        }
    }

    pub fn expected_pair_breadth(self) -> usize {
        self.expected_pair_breadth
    }

    pub fn indexed_candidate_pair_count(self) -> usize {
        self.indexed_candidate_pair_count
    }

    pub fn culled_pair_count(self) -> usize {
        self.culled_pair_count
    }

    pub fn emitted_pair_count(self) -> usize {
        self.emitted_pair_count
    }

    pub fn fallback_used(self) -> bool {
        self.fallback_used
    }
}
