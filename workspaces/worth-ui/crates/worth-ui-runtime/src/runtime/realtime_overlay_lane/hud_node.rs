use crate::runtime::{
    WorthUiRealtimeOverlayHook, WorthUiRendererSurfaceAdmission, WorthUiRuntimeHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHudNode {
    runtime_handle: WorthUiRuntimeHandle,
    renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    overlay_hook: WorthUiRealtimeOverlayHook,
}

impl WorthUiHudNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        contract: crate::capability::ComponentRealtimeOverlayContract,
        host_binding: crate::facade::WorthUiHostPlanBinding,
        plan_basis_digest: u64,
    ) -> Self {
        Self {
            runtime_handle,
            renderer_surface_admission: WorthUiRendererSurfaceAdmission::new(
                runtime_handle,
                contract,
                host_binding,
                plan_basis_digest,
            ),
            overlay_hook: WorthUiRealtimeOverlayHook::from_host_binding(
                host_binding,
                plan_basis_digest,
                runtime_handle.plan_index(),
            ),
        }
    }

    pub fn runtime_handle(self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }
    pub fn plan_index(self) -> u32 {
        self.runtime_handle.plan_index()
    }
    pub fn renderer_surface_admission(self) -> WorthUiRendererSurfaceAdmission {
        self.renderer_surface_admission
    }
    pub fn overlay_hook(self) -> WorthUiRealtimeOverlayHook {
        self.overlay_hook
    }
}
