#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanIntervalSubdivisionNormalizationCounters {
    normalized_schedules: usize,
    retained_interval_rows_inspected: usize,
    normalized_interval_subdivisions: usize,
    redundant_interval_rows_collapsed: usize,
    micro_intervals_admitted: usize,
    micro_intervals_policy_required: usize,
    opposite_sense_rows_preserved: usize,
    fragment_point_cuts_retained: usize,
    endpoint_contact_decisions_retained: usize,
}

impl PlanarBooleanIntervalSubdivisionNormalizationCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        normalized_schedules: usize,
        retained_interval_rows_inspected: usize,
        normalized_interval_subdivisions: usize,
        redundant_interval_rows_collapsed: usize,
        micro_intervals_admitted: usize,
        micro_intervals_policy_required: usize,
        opposite_sense_rows_preserved: usize,
        fragment_point_cuts_retained: usize,
        endpoint_contact_decisions_retained: usize,
    ) -> Self {
        Self {
            normalized_schedules,
            retained_interval_rows_inspected,
            normalized_interval_subdivisions,
            redundant_interval_rows_collapsed,
            micro_intervals_admitted,
            micro_intervals_policy_required,
            opposite_sense_rows_preserved,
            fragment_point_cuts_retained,
            endpoint_contact_decisions_retained,
        }
    }

    pub fn normalized_schedules(self) -> usize {
        self.normalized_schedules
    }
    pub fn retained_interval_rows_inspected(self) -> usize {
        self.retained_interval_rows_inspected
    }
    pub fn normalized_interval_subdivisions(self) -> usize {
        self.normalized_interval_subdivisions
    }
    pub fn redundant_interval_rows_collapsed(self) -> usize {
        self.redundant_interval_rows_collapsed
    }
    pub fn micro_intervals_admitted(self) -> usize {
        self.micro_intervals_admitted
    }
    pub fn micro_intervals_policy_required(self) -> usize {
        self.micro_intervals_policy_required
    }
    pub fn opposite_sense_rows_preserved(self) -> usize {
        self.opposite_sense_rows_preserved
    }
    pub fn fragment_point_cuts_retained(self) -> usize {
        self.fragment_point_cuts_retained
    }
    pub fn endpoint_contact_decisions_retained(self) -> usize {
        self.endpoint_contact_decisions_retained
    }
}
