use super::counters::PlannerOwnedRoutingInventoryCounters;
use super::cut_line::PlannerOwnedRoutingCutLine;
use super::row::PlannerOwnedRoutingInventoryRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingInventoryReport {
    rows: Vec<PlannerOwnedRoutingInventoryRow>,
    counters: PlannerOwnedRoutingInventoryCounters,
    cut_line: PlannerOwnedRoutingCutLine,
}

impl PlannerOwnedRoutingInventoryReport {
    pub(super) fn new(rows: Vec<PlannerOwnedRoutingInventoryRow>) -> Self {
        let counters = PlannerOwnedRoutingInventoryCounters::from_rows(&rows);
        let cut_line = PlannerOwnedRoutingCutLine::from_rows(&rows);
        Self {
            rows,
            counters,
            cut_line,
        }
    }

    pub fn rows(&self) -> &[PlannerOwnedRoutingInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &PlannerOwnedRoutingInventoryCounters {
        &self.counters
    }

    pub const fn cut_line(&self) -> &PlannerOwnedRoutingCutLine {
        &self.cut_line
    }
}
