#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanIntervalSplitCandidateCounters {
    inspected_interval_events: usize,
    emitted_interval_candidates: usize,
    rejected_missing_source_ranges: usize,
}

impl PlanarBooleanIntervalSplitCandidateCounters {
    pub(crate) fn new(
        inspected_interval_events: usize,
        emitted_interval_candidates: usize,
        rejected_missing_source_ranges: usize,
    ) -> Self {
        Self {
            inspected_interval_events,
            emitted_interval_candidates,
            rejected_missing_source_ranges,
        }
    }

    pub fn inspected_interval_events(self) -> usize {
        self.inspected_interval_events
    }

    pub fn emitted_interval_candidates(self) -> usize {
        self.emitted_interval_candidates
    }

    pub fn rejected_missing_source_ranges(self) -> usize {
        self.rejected_missing_source_ranges
    }
}
