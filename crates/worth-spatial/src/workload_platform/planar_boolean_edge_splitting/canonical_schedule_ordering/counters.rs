#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOrderedEdgeSplitScheduleCounters {
    ordered_schedules: usize,
    ordered_entries: usize,
    equal_parameter_ties: usize,
}

impl PlanarBooleanOrderedEdgeSplitScheduleCounters {
    pub(crate) fn new(
        ordered_schedules: usize,
        ordered_entries: usize,
        equal_parameter_ties: usize,
    ) -> Self {
        Self {
            ordered_schedules,
            ordered_entries,
            equal_parameter_ties,
        }
    }

    pub fn ordered_schedules(self) -> usize {
        self.ordered_schedules
    }

    pub fn ordered_entries(self) -> usize {
        self.ordered_entries
    }

    pub fn equal_parameter_ties(self) -> usize {
        self.equal_parameter_ties
    }
}
