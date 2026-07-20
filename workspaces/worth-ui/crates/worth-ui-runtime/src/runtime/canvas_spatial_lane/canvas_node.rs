use crate::runtime::{WorthUiLaneHandle, WorthUiRuntimeHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSpatialIndexStrategy {
    Dense,
    Tiled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasRenderResourceRef {
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_basis_digest: u64,
    owner_plan_index: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialNode {
    runtime_handle: WorthUiRuntimeHandle,
    lane_handle: WorthUiLaneHandle,
    contract: crate::capability::ComponentCanvasSpatialContract,
    strategy: WorthUiSpatialIndexStrategy,
    render_resource: WorthUiCanvasRenderResourceRef,
}

impl WorthUiCanvasSpatialNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        contract: crate::capability::ComponentCanvasSpatialContract,
        host_binding: crate::facade::WorthUiHostPlanBinding,
        plan_basis_digest: u64,
    ) -> Self {
        let strategy = if contract.visible_primitive_limit() <= 256 {
            WorthUiSpatialIndexStrategy::Dense
        } else {
            WorthUiSpatialIndexStrategy::Tiled
        };
        Self {
            runtime_handle,
            lane_handle: WorthUiLaneHandle::from_locator(runtime_handle.locator()),
            contract,
            strategy,
            render_resource: WorthUiCanvasRenderResourceRef {
                host_session_identity: host_binding.session_identity().as_u64(),
                host_observation_generation: host_binding.observation_generation().as_u64(),
                plan_basis_digest,
                owner_plan_index: runtime_handle.plan_index(),
            },
        }
    }

    pub fn runtime_handle(self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }
    pub fn lane_handle(self) -> WorthUiLaneHandle {
        self.lane_handle
    }
    pub fn plan_index(self) -> u32 {
        self.runtime_handle.plan_index()
    }
    pub fn visible_primitive_limit(self) -> u32 {
        self.contract.visible_primitive_limit()
    }
    pub fn overlay_row_limit(self) -> u16 {
        self.contract.overlay_row_limit()
    }
    pub fn tool_state_row_limit(self) -> u16 {
        self.contract.tool_state_row_limit()
    }
    pub fn strategy(self) -> WorthUiSpatialIndexStrategy {
        self.strategy
    }
    pub fn render_resource(self) -> WorthUiCanvasRenderResourceRef {
        self.render_resource
    }
    pub fn render_resource_ref_count(self) -> usize {
        1
    }
}

impl WorthUiCanvasRenderResourceRef {
    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
    pub fn host_observation_generation(self) -> u64 {
        self.host_observation_generation
    }
    pub fn plan_basis_digest(self) -> u64 {
        self.plan_basis_digest
    }
    pub fn owner_plan_index(self) -> u32 {
        self.owner_plan_index
    }
}
