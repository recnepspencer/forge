use super::WorthUiProjectionRebindStatus;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiProjectionRebindCounters {
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    rebuild_attempt_count: usize,
    preserved_frame_count: usize,
    denied_frame_count: usize,
    rebuilt_frame_count: usize,
}

impl WorthUiProjectionRebindCounters {
    pub(crate) fn inspected_without_intersection(status: WorthUiProjectionRebindStatus) -> Self {
        Self::for_row(status, false, false)
    }

    pub(crate) fn after_rebuild(status: WorthUiProjectionRebindStatus) -> Self {
        Self::for_row(status, true, true)
    }

    pub(crate) fn aggregate(rows: impl IntoIterator<Item = Self>) -> Self {
        let mut aggregate = Self::default();
        for row in rows {
            aggregate.inspected_projection_count += row.inspected_projection_count;
            aggregate.dependency_intersection_count += row.dependency_intersection_count;
            aggregate.rebuild_attempt_count += row.rebuild_attempt_count;
            aggregate.preserved_frame_count += row.preserved_frame_count;
            aggregate.denied_frame_count += row.denied_frame_count;
            aggregate.rebuilt_frame_count += row.rebuilt_frame_count;
        }
        aggregate
    }

    #[cfg(test)]
    pub(crate) fn from_counts_for_test(
        inspected_projection_count: usize,
        dependency_intersection_count: usize,
        rebuild_attempt_count: usize,
        preserved_frame_count: usize,
        denied_frame_count: usize,
        rebuilt_frame_count: usize,
    ) -> Self {
        Self {
            inspected_projection_count,
            dependency_intersection_count,
            rebuild_attempt_count,
            preserved_frame_count,
            denied_frame_count,
            rebuilt_frame_count,
        }
    }

    fn for_row(
        status: WorthUiProjectionRebindStatus,
        dependency_intersected: bool,
        rebuild_attempted: bool,
    ) -> Self {
        Self {
            inspected_projection_count: 1,
            dependency_intersection_count: usize::from(dependency_intersected),
            rebuild_attempt_count: usize::from(rebuild_attempted),
            preserved_frame_count: usize::from(!rebuild_attempted && status.preserves_frame()),
            denied_frame_count: usize::from(status.denied_frame()),
            rebuilt_frame_count: usize::from(rebuild_attempted),
        }
    }

    pub fn inspected_projection_count(self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(self) -> usize {
        self.dependency_intersection_count
    }

    pub fn rebuild_attempt_count(self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_frame_count(self) -> usize {
        self.preserved_frame_count
    }

    pub fn denied_frame_count(self) -> usize {
        self.denied_frame_count
    }

    pub fn rebuilt_frame_count(self) -> usize {
        self.rebuilt_frame_count
    }
}
