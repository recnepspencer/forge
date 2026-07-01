use super::classification::PlannerOwnedRoutingReplacementLane;
use super::row::PlannerOwnedRoutingInventoryRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingReplacementLaneCount {
    lane: PlannerOwnedRoutingReplacementLane,
    row_count: usize,
}

impl PlannerOwnedRoutingReplacementLaneCount {
    pub const fn lane(&self) -> PlannerOwnedRoutingReplacementLane {
        self.lane
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingCutLine {
    lane_counts: Vec<PlannerOwnedRoutingReplacementLaneCount>,
}

impl PlannerOwnedRoutingCutLine {
    pub(super) fn from_rows(rows: &[PlannerOwnedRoutingInventoryRow]) -> Self {
        let mut lane_counts = Vec::new();
        for lane in PlannerOwnedRoutingReplacementLane::ALL {
            let row_count = rows
                .iter()
                .filter(|row| row.replacement_lane() == lane)
                .count();
            if row_count > 0 {
                lane_counts.push(PlannerOwnedRoutingReplacementLaneCount { lane, row_count });
            }
        }
        Self { lane_counts }
    }

    pub fn lane_counts(&self) -> &[PlannerOwnedRoutingReplacementLaneCount] {
        &self.lane_counts
    }
}
