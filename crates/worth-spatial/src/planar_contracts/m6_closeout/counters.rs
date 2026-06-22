#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct M6PlanarCloseoutCounters {
    premetaboss_rows: usize,
    legacy_deletion_rows: usize,
    query_boundary_rows: usize,
    closeout_rows: usize,
    rejected_shortcut_rows: usize,
}

impl M6PlanarCloseoutCounters {
    pub(crate) fn certified(
        premetaboss_rows: usize,
        legacy_deletion_rows: usize,
        query_boundary_rows: usize,
        closeout_rows: usize,
        rejected_shortcut_rows: usize,
    ) -> Self {
        Self {
            premetaboss_rows,
            legacy_deletion_rows,
            query_boundary_rows,
            closeout_rows,
            rejected_shortcut_rows,
        }
    }

    pub fn premetaboss_rows(&self) -> usize {
        self.premetaboss_rows
    }

    pub fn legacy_deletion_rows(&self) -> usize {
        self.legacy_deletion_rows
    }

    pub fn query_boundary_rows(&self) -> usize {
        self.query_boundary_rows
    }

    pub fn closeout_rows(&self) -> usize {
        self.closeout_rows
    }

    pub fn rejected_shortcut_rows(&self) -> usize {
        self.rejected_shortcut_rows
    }
}
