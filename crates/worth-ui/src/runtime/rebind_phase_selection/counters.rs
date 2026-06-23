use super::WorthUiRebindPhaseSelectionRow;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRebindPhaseSelectionCounters {
    phase_row_count: usize,
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    skipped_phase_count: usize,
    rebuild_attempt_count: usize,
    preserved_projection_count: usize,
    rebuilt_projection_count: usize,
}

impl WorthUiRebindPhaseSelectionCounters {
    pub(crate) fn from_rows(rows: &[WorthUiRebindPhaseSelectionRow]) -> Self {
        let mut counters = Self::default();
        for row in rows {
            counters.phase_row_count += 1;
            counters.inspected_projection_count += 1;
            counters.dependency_intersection_count += row.dependency_intersection_count();
            counters.skipped_phase_count += usize::from(row.status().skipped_phase());
            counters.rebuild_attempt_count += usize::from(row.status().rebuild_attempt());
            counters.preserved_projection_count += usize::from(row.status().preserved_projection());
            counters.rebuilt_projection_count += usize::from(row.status().rebuilt_projection());
        }
        counters
    }

    pub fn phase_row_count(self) -> usize {
        self.phase_row_count
    }

    pub fn inspected_projection_count(self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(self) -> usize {
        self.dependency_intersection_count
    }

    pub fn skipped_phase_count(self) -> usize {
        self.skipped_phase_count
    }

    pub fn rebuild_attempt_count(self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_projection_count(self) -> usize {
        self.preserved_projection_count
    }

    pub fn rebuilt_projection_count(self) -> usize {
        self.rebuilt_projection_count
    }
}
