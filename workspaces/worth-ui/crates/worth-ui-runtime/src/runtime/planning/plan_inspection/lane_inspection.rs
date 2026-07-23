use crate::runtime::WorthUiPlanExecutionLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneInspection {
    lane: WorthUiPlanExecutionLane,
    plan_indexes: Vec<u32>,
    node_count: usize,
}

impl WorthUiLaneInspection {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn new(
        lane: WorthUiPlanExecutionLane,
        plan_indexes: Vec<u32>,
        node_count: usize,
    ) -> Self {
        Self {
            lane,
            plan_indexes,
            node_count,
        }
    }

    pub fn lane(&self) -> WorthUiPlanExecutionLane {
        self.lane
    }

    pub fn plan_indexes(&self) -> &[u32] {
        &self.plan_indexes
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }
}
