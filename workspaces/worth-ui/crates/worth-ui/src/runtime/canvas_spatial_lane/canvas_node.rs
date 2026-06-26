use crate::runtime::{WorthUiLaneHandle, WorthUiRuntimeHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialNode {
    runtime_handle: WorthUiRuntimeHandle,
    lane_handle: WorthUiLaneHandle,
    render_resource_ref_count: usize,
}

impl WorthUiCanvasSpatialNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        lane_handle: WorthUiLaneHandle,
        render_resource_ref_count: usize,
    ) -> Self {
        Self {
            runtime_handle,
            lane_handle,
            render_resource_ref_count,
        }
    }

    pub fn runtime_handle(self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn lane_handle(self) -> WorthUiLaneHandle {
        self.lane_handle
    }

    pub fn plan_index(&self) -> u32 {
        self.runtime_handle.plan_index()
    }

    pub fn render_resource_ref_count(&self) -> usize {
        self.render_resource_ref_count
    }
}
