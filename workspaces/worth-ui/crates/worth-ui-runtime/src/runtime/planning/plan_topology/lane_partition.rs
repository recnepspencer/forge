#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiPlanExecutionLane {
    UiStructure,
    QueryView,
    Command,
    Style,
    Diagnostics,
    LaneBoundary,
    RenderResource,
    CanvasSpatial,
    RealtimeOverlay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanLanePartition {
    lane: WorthUiPlanExecutionLane,
    plan_indexes: Vec<u32>,
}

impl WorthUiPlanLanePartition {
    pub(crate) fn new(lane: WorthUiPlanExecutionLane, plan_indexes: Vec<u32>) -> Self {
        Self { lane, plan_indexes }
    }

    pub fn lane(&self) -> WorthUiPlanExecutionLane {
        self.lane
    }

    pub fn plan_indexes(&self) -> &[u32] {
        &self.plan_indexes
    }

    pub fn node_count(&self) -> usize {
        self.plan_indexes.len()
    }
}
