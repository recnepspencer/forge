#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedPlanarFactsCounters {
    retained_family_rows_inspected: usize,
    retained_fact_rows_inspected: usize,
    replay_basis_rows_inspected: usize,
    branch_basis_rows_inspected: usize,
    rejected_basis_rows: usize,
}

impl RetainedPlanarFactsCounters {
    pub(crate) const fn retained(
        retained_family_rows_inspected: usize,
        retained_fact_rows_inspected: usize,
        replay_basis_rows_inspected: usize,
    ) -> Self {
        Self {
            retained_family_rows_inspected,
            retained_fact_rows_inspected,
            replay_basis_rows_inspected,
            branch_basis_rows_inspected: 0,
            rejected_basis_rows: 0,
        }
    }

    pub(crate) const fn historical_replay(
        retained_family_rows_inspected: usize,
        retained_fact_rows_inspected: usize,
        replay_basis_rows_inspected: usize,
    ) -> Self {
        Self::retained(
            retained_family_rows_inspected,
            retained_fact_rows_inspected,
            replay_basis_rows_inspected,
        )
    }

    pub(crate) const fn branch_local_replay(
        retained_family_rows_inspected: usize,
        retained_fact_rows_inspected: usize,
        replay_basis_rows_inspected: usize,
        branch_basis_rows_inspected: usize,
    ) -> Self {
        Self {
            retained_family_rows_inspected,
            retained_fact_rows_inspected,
            replay_basis_rows_inspected,
            branch_basis_rows_inspected,
            rejected_basis_rows: 0,
        }
    }

    pub(crate) const fn rejected() -> Self {
        Self {
            retained_family_rows_inspected: 0,
            retained_fact_rows_inspected: 0,
            replay_basis_rows_inspected: 0,
            branch_basis_rows_inspected: 0,
            rejected_basis_rows: 1,
        }
    }

    pub fn retained_family_rows_inspected(self) -> usize {
        self.retained_family_rows_inspected
    }

    pub fn retained_fact_rows_inspected(self) -> usize {
        self.retained_fact_rows_inspected
    }

    pub fn replay_basis_rows_inspected(self) -> usize {
        self.replay_basis_rows_inspected
    }

    pub fn branch_basis_rows_inspected(self) -> usize {
        self.branch_basis_rows_inspected
    }

    pub fn rejected_basis_rows(self) -> usize {
        self.rejected_basis_rows
    }
}
