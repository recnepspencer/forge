use crate::runtime::{WorthUiRendererSurfaceAdmission, WorthUiRuntimeHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHudNode {
    runtime_handle: WorthUiRuntimeHandle,
    renderer_surface_admission: WorthUiRendererSurfaceAdmission,
}

impl WorthUiHudNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    ) -> Self {
        Self {
            runtime_handle,
            renderer_surface_admission,
        }
    }

    pub fn runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn plan_index(&self) -> u32 {
        self.runtime_handle.plan_index()
    }

    pub fn renderer_surface_admission(&self) -> WorthUiRendererSurfaceAdmission {
        self.renderer_surface_admission
    }
}
