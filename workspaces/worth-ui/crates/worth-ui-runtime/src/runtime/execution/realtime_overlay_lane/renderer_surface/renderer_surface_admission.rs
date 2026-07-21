use crate::runtime::{
    WorthUiHighFrequencyFramePolicy, WorthUiRendererSurfaceHandle, WorthUiRuntimeHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRendererSurfaceAdmission {
    handle: WorthUiRendererSurfaceHandle,
    runtime_handle: WorthUiRuntimeHandle,
    policy: WorthUiHighFrequencyFramePolicy,
    overlay_row_limit: u16,
    declared_frame_cost_millis: u16,
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_basis_digest: u64,
}

impl WorthUiRendererSurfaceAdmission {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        contract: crate::capability::ComponentRealtimeOverlayContract,
        host_binding: crate::facade::WorthUiHostPlanBinding,
        plan_basis_digest: u64,
    ) -> Self {
        Self {
            handle: WorthUiRendererSurfaceHandle::new(runtime_handle.locator()),
            runtime_handle,
            policy: WorthUiHighFrequencyFramePolicy::from_contract(contract),
            overlay_row_limit: contract.overlay_row_limit(),
            declared_frame_cost_millis: contract.declared_frame_cost_millis(),
            host_session_identity: host_binding.session_identity().as_u64(),
            host_observation_generation: host_binding.observation_generation().as_u64(),
            plan_basis_digest,
        }
    }

    pub fn handle(self) -> WorthUiRendererSurfaceHandle {
        self.handle
    }
    pub fn runtime_handle(self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }
    pub fn policy(self) -> WorthUiHighFrequencyFramePolicy {
        self.policy
    }
    pub fn policy_digest(self) -> u64 {
        self.policy.canonical_digest()
    }
    pub fn overlay_row_limit(self) -> u16 {
        self.overlay_row_limit
    }
    pub fn declared_frame_cost_millis(self) -> u16 {
        self.declared_frame_cost_millis
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
