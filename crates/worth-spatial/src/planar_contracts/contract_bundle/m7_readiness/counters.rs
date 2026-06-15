#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarM7ReadinessCounters {
    closeout_rows: usize,
    retained_fact_rows: usize,
    projection_consumed_rows: usize,
    support_posture_rows: usize,
    rejected_rows: usize,
}

impl PlanarM7ReadinessCounters {
    pub(crate) const fn certified(
        closeout_rows: usize,
        retained_fact_rows: usize,
        projection_consumed_rows: usize,
        support_posture_rows: usize,
    ) -> Self {
        Self {
            closeout_rows,
            retained_fact_rows,
            projection_consumed_rows,
            support_posture_rows,
            rejected_rows: 0,
        }
    }

    pub(crate) const fn rejected() -> Self {
        Self {
            closeout_rows: 0,
            retained_fact_rows: 0,
            projection_consumed_rows: 0,
            support_posture_rows: 0,
            rejected_rows: 1,
        }
    }

    pub fn closeout_rows(self) -> usize {
        self.closeout_rows
    }

    pub fn retained_fact_rows(self) -> usize {
        self.retained_fact_rows
    }

    pub fn projection_consumed_rows(self) -> usize {
        self.projection_consumed_rows
    }

    pub fn support_posture_rows(self) -> usize {
        self.support_posture_rows
    }

    pub fn rejected_rows(self) -> usize {
        self.rejected_rows
    }
}
