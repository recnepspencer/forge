use crate::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiHighFrequencyFramePolicy, WorthUiHudPlan,
    WorthUiRendererSurfaceHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeTargetSummary {
    plan_index: u32,
    overlay_row_limit: u16,
    declared_frame_cost_millis: u16,
    policy: WorthUiHighFrequencyFramePolicy,
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_basis_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeInspectionDenial {
    outcome: WorthUiHandleResolutionOutcome,
}

impl WorthUiRealtimeTargetSummary {
    fn from_node(node: crate::runtime::WorthUiHudNode) -> Self {
        let surface = node.renderer_surface_admission();
        Self {
            plan_index: node.plan_index(),
            overlay_row_limit: surface.overlay_row_limit(),
            declared_frame_cost_millis: surface.declared_frame_cost_millis(),
            policy: surface.policy(),
            host_session_identity: surface.host_session_identity(),
            host_observation_generation: surface.host_observation_generation(),
            plan_basis_digest: surface.plan_basis_digest(),
        }
    }

    pub fn plan_index(self) -> u32 {
        self.plan_index
    }
    pub fn overlay_row_limit(self) -> u16 {
        self.overlay_row_limit
    }
    pub fn declared_frame_cost_millis(self) -> u16 {
        self.declared_frame_cost_millis
    }
    pub fn policy(self) -> WorthUiHighFrequencyFramePolicy {
        self.policy
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

impl WorthUiRealtimeInspectionDenial {
    pub(crate) fn new(outcome: WorthUiHandleResolutionOutcome) -> Self {
        Self { outcome }
    }
    pub fn outcome(self) -> WorthUiHandleResolutionOutcome {
        self.outcome
    }
}

pub(crate) fn summarize(
    plan: &WorthUiHudPlan,
    handle: WorthUiRendererSurfaceHandle,
) -> Result<WorthUiRealtimeTargetSummary, WorthUiRealtimeInspectionDenial> {
    crate::runtime::execution::handle_allocation::resolve_handle_row(
        plan.handle_receipt().arena_identity(),
        crate::runtime::WorthUiPlanNodeInputFamily::RealtimeOverlay,
        handle.locator(),
        |index| plan.row_for_plan_index(index),
        |row| row.runtime_handle(),
    )
    .map(|(row, _)| WorthUiRealtimeTargetSummary::from_node(row))
    .map_err(|evidence| WorthUiRealtimeInspectionDenial::new(evidence.outcome()))
}
