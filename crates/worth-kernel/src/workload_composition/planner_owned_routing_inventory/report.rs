use super::row::PlannerOwnedRoutingInventoryRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingInventoryReport {
    rows: Vec<PlannerOwnedRoutingInventoryRow>,
}

impl PlannerOwnedRoutingInventoryReport {
    pub(super) fn new(rows: Vec<PlannerOwnedRoutingInventoryRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[PlannerOwnedRoutingInventoryRow] {
        &self.rows
    }
}
