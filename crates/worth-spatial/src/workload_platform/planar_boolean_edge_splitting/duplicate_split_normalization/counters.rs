#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanNormalizedEdgeSplitScheduleCounters {
    normalized_schedules: usize,
    raw_point_cuts: usize,
    normalized_point_cuts: usize,
    duplicate_reports_collapsed: usize,
    provenance_rows_retained: usize,
    retained_interval_entries: usize,
}

impl PlanarBooleanNormalizedEdgeSplitScheduleCounters {
    pub(crate) fn new(
        normalized_schedules: usize,
        raw_point_cuts: usize,
        normalized_point_cuts: usize,
        duplicate_reports_collapsed: usize,
        provenance_rows_retained: usize,
        retained_interval_entries: usize,
    ) -> Self {
        Self {
            normalized_schedules,
            raw_point_cuts,
            normalized_point_cuts,
            duplicate_reports_collapsed,
            provenance_rows_retained,
            retained_interval_entries,
        }
    }

    pub fn normalized_schedules(self) -> usize {
        self.normalized_schedules
    }
    pub fn raw_point_cuts(self) -> usize {
        self.raw_point_cuts
    }
    pub fn normalized_point_cuts(self) -> usize {
        self.normalized_point_cuts
    }
    pub fn duplicate_reports_collapsed(self) -> usize {
        self.duplicate_reports_collapsed
    }
    pub fn provenance_rows_retained(self) -> usize {
        self.provenance_rows_retained
    }
    pub fn retained_interval_entries(self) -> usize {
        self.retained_interval_entries
    }
}
