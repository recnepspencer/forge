use super::{WorthUiRebindPhaseLane, WorthUiRebindPhaseSelectionStatus};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRebindPhaseSelectionRow {
    lane: WorthUiRebindPhaseLane,
    status: WorthUiRebindPhaseSelectionStatus,
    dependency_intersection_count: usize,
}

impl WorthUiRebindPhaseSelectionRow {
    pub(crate) fn new(
        lane: WorthUiRebindPhaseLane,
        status: WorthUiRebindPhaseSelectionStatus,
        dependency_intersection_count: usize,
    ) -> Self {
        Self {
            lane,
            status,
            dependency_intersection_count,
        }
    }

    pub fn lane(self) -> WorthUiRebindPhaseLane {
        self.lane
    }

    pub fn status(self) -> WorthUiRebindPhaseSelectionStatus {
        self.status
    }

    pub fn dependency_intersection_count(self) -> usize {
        self.dependency_intersection_count
    }
}
