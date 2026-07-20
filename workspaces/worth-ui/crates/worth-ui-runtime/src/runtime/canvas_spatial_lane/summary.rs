use crate::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiLaneHandle, WorthUiSpatialIndexStrategy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialTargetSummary {
    plan_index: u32,
    strategy: WorthUiSpatialIndexStrategy,
    visible_primitive_limit: u32,
    overlay_row_limit: u16,
    tool_state_row_limit: u16,
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_basis_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialInspectionDenial {
    outcome: WorthUiHandleResolutionOutcome,
}

impl WorthUiCanvasSpatialTargetSummary {
    pub(crate) fn from_node(node: crate::runtime::WorthUiCanvasSpatialNode) -> Self {
        let resource = node.render_resource();
        Self {
            plan_index: node.plan_index(),
            strategy: node.strategy(),
            visible_primitive_limit: node.visible_primitive_limit(),
            overlay_row_limit: node.overlay_row_limit(),
            tool_state_row_limit: node.tool_state_row_limit(),
            host_session_identity: resource.host_session_identity(),
            host_observation_generation: resource.host_observation_generation(),
            plan_basis_digest: resource.plan_basis_digest(),
        }
    }
    pub fn plan_index(self) -> u32 {
        self.plan_index
    }
    pub fn strategy(self) -> WorthUiSpatialIndexStrategy {
        self.strategy
    }
    pub fn visible_primitive_limit(self) -> u32 {
        self.visible_primitive_limit
    }
    pub fn overlay_row_limit(self) -> u16 {
        self.overlay_row_limit
    }
    pub fn tool_state_row_limit(self) -> u16 {
        self.tool_state_row_limit
    }
    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
    pub fn host_observation_generation(self) -> u64 {
        self.host_observation_generation
    }
    pub fn plan_basis_digest(self) -> u64 {
        self.plan_basis_digest
    }
}

impl WorthUiCanvasSpatialInspectionDenial {
    pub(crate) fn new(outcome: WorthUiHandleResolutionOutcome) -> Self {
        Self { outcome }
    }
    pub fn outcome(self) -> WorthUiHandleResolutionOutcome {
        self.outcome
    }
}

pub(crate) fn summarize(
    plan: &crate::runtime::WorthUiCanvasSpatialPlan,
    handle: WorthUiLaneHandle,
) -> Result<WorthUiCanvasSpatialTargetSummary, WorthUiCanvasSpatialInspectionDenial> {
    crate::runtime::handle_allocation::resolve_handle_row(
        plan.handle_receipt().arena_identity(),
        crate::runtime::WorthUiPlanNodeInputFamily::CanvasSpatial,
        handle.locator(),
        |index| plan.row_for_plan_index(index),
        |row| row.runtime_handle(),
    )
    .map(|(row, _)| WorthUiCanvasSpatialTargetSummary::from_node(row))
    .map_err(|evidence| WorthUiCanvasSpatialInspectionDenial::new(evidence.outcome()))
}
