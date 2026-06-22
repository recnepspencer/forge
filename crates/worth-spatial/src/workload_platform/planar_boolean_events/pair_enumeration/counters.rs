#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSegmentPairEnumerationCounters {
    left_segment_count: usize,
    right_segment_count: usize,
    expected_pair_breadth: usize,
    expected_pair_breadth_overflowed: bool,
    emitted_pair_breadth: usize,
    skipped_pair_count: usize,
    query_index_candidate_count: usize,
    query_index_culled_pair_count: usize,
    envelope_expanded_pair_count: usize,
    broad_phase_comparison_count: usize,
    degenerate_skip_count: usize,
    fallback_used: bool,
}

impl PlanarBooleanSegmentPairEnumerationCounters {
    pub(crate) fn new(
        left_segment_count: usize,
        right_segment_count: usize,
        emitted_pair_breadth: usize,
        skipped_pair_count: usize,
    ) -> Self {
        Self::from_index_counts(
            left_segment_count,
            right_segment_count,
            emitted_pair_breadth,
            skipped_pair_count,
            emitted_pair_breadth,
            skipped_pair_count,
        )
    }

    pub(crate) fn from_index_counts(
        left_segment_count: usize,
        right_segment_count: usize,
        emitted_pair_breadth: usize,
        skipped_pair_count: usize,
        query_index_candidate_count: usize,
        query_index_culled_pair_count: usize,
    ) -> Self {
        let expected_pair_breadth = left_segment_count.checked_mul(right_segment_count);
        Self {
            left_segment_count,
            right_segment_count,
            expected_pair_breadth: expected_pair_breadth.unwrap_or(usize::MAX),
            expected_pair_breadth_overflowed: expected_pair_breadth.is_none(),
            emitted_pair_breadth,
            skipped_pair_count,
            query_index_candidate_count,
            query_index_culled_pair_count,
            envelope_expanded_pair_count: query_index_candidate_count,
            broad_phase_comparison_count: 0,
            degenerate_skip_count: 0,
            fallback_used: false,
        }
    }

    pub(crate) fn with_strategy_counts(
        mut self,
        envelope_expanded_pair_count: usize,
        broad_phase_comparison_count: usize,
        degenerate_skip_count: usize,
        fallback_used: bool,
    ) -> Self {
        self.envelope_expanded_pair_count = envelope_expanded_pair_count;
        self.broad_phase_comparison_count = broad_phase_comparison_count;
        self.degenerate_skip_count = degenerate_skip_count;
        self.fallback_used = fallback_used;
        self
    }

    pub fn left_segment_count(self) -> usize {
        self.left_segment_count
    }

    pub fn right_segment_count(self) -> usize {
        self.right_segment_count
    }

    pub fn expected_pair_breadth(self) -> usize {
        self.expected_pair_breadth
    }

    pub fn expected_pair_breadth_overflowed(self) -> bool {
        self.expected_pair_breadth_overflowed
    }

    pub fn emitted_pair_breadth(self) -> usize {
        self.emitted_pair_breadth
    }

    pub fn skipped_pair_count(self) -> usize {
        self.skipped_pair_count
    }

    pub fn query_index_candidate_count(self) -> usize {
        self.query_index_candidate_count
    }

    pub fn query_index_culled_pair_count(self) -> usize {
        self.query_index_culled_pair_count
    }

    pub fn envelope_expanded_pair_count(self) -> usize {
        self.envelope_expanded_pair_count
    }

    pub fn broad_phase_comparison_count(self) -> usize {
        self.broad_phase_comparison_count
    }

    pub fn degenerate_skip_count(self) -> usize {
        self.degenerate_skip_count
    }

    pub fn fallback_used(self) -> bool {
        self.fallback_used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_pair_breadth_overflow_is_visible_in_counters() {
        let counters = PlanarBooleanSegmentPairEnumerationCounters::new(usize::MAX, 2, 0, 0);

        assert_eq!(counters.expected_pair_breadth(), usize::MAX);
        assert!(counters.expected_pair_breadth_overflowed());
    }
}
