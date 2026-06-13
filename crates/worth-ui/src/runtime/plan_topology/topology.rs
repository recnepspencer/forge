use crate::runtime::{WorthUiPlanChildRange, WorthUiPlanNode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanTopology {
    traversal_order: Vec<WorthUiPlanNode>,
    child_ranges: Vec<WorthUiPlanChildRange>,
}

impl WorthUiPlanTopology {
    pub(crate) fn new(
        traversal_order: Vec<WorthUiPlanNode>,
        child_ranges: Vec<WorthUiPlanChildRange>,
    ) -> Self {
        Self {
            traversal_order,
            child_ranges,
        }
    }

    pub fn traversal_order(&self) -> &[WorthUiPlanNode] {
        &self.traversal_order
    }

    pub fn child_ranges(&self) -> &[WorthUiPlanChildRange] {
        &self.child_ranges
    }
}
