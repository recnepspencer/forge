#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitIntervalAdmissionCounters {
    inspected_interval_candidates: usize,
    admitted_interval_candidates: usize,
    collapsed_interval_denials: usize,
    rejected_non_finite_intervals: usize,
    rejected_out_of_domain_intervals: usize,
    rejected_contradictory_sense_intervals: usize,
}

impl PlanarBooleanSplitIntervalAdmissionCounters {
    pub(crate) fn new(
        inspected_interval_candidates: usize,
        admitted_interval_candidates: usize,
        collapsed_interval_denials: usize,
        rejected_non_finite_intervals: usize,
        rejected_out_of_domain_intervals: usize,
        rejected_contradictory_sense_intervals: usize,
    ) -> Self {
        Self {
            inspected_interval_candidates,
            admitted_interval_candidates,
            collapsed_interval_denials,
            rejected_non_finite_intervals,
            rejected_out_of_domain_intervals,
            rejected_contradictory_sense_intervals,
        }
    }

    pub fn inspected_interval_candidates(self) -> usize {
        self.inspected_interval_candidates
    }

    pub fn admitted_interval_candidates(self) -> usize {
        self.admitted_interval_candidates
    }

    pub fn collapsed_interval_denials(self) -> usize {
        self.collapsed_interval_denials
    }

    pub fn rejected_non_finite_intervals(self) -> usize {
        self.rejected_non_finite_intervals
    }

    pub fn rejected_out_of_domain_intervals(self) -> usize {
        self.rejected_out_of_domain_intervals
    }

    pub fn rejected_contradictory_sense_intervals(self) -> usize {
        self.rejected_contradictory_sense_intervals
    }
}
