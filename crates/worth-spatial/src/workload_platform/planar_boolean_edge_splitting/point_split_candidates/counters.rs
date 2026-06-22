#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanPointSplitCandidateCounters {
    inspected_point_events: usize,
    emitted_point_candidates: usize,
    rejected_missing_parameter_facts: usize,
}

impl PlanarBooleanPointSplitCandidateCounters {
    pub(crate) fn new(
        inspected_point_events: usize,
        emitted_point_candidates: usize,
        rejected_missing_parameter_facts: usize,
    ) -> Self {
        Self {
            inspected_point_events,
            emitted_point_candidates,
            rejected_missing_parameter_facts,
        }
    }

    pub fn inspected_point_events(self) -> usize {
        self.inspected_point_events
    }

    pub fn emitted_point_candidates(self) -> usize {
        self.emitted_point_candidates
    }

    pub fn rejected_missing_parameter_facts(self) -> usize {
        self.rejected_missing_parameter_facts
    }
}
