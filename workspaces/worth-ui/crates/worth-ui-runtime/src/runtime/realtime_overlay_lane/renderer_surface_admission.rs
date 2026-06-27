use crate::runtime::{
    WorthUiHighFrequencyFramePolicy, WorthUiRendererSurfaceHandle, WorthUiRuntimeHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRendererSurfaceAdmission {
    handle: WorthUiRendererSurfaceHandle,
    runtime_handle: WorthUiRuntimeHandle,
    policy_digest: u64,
    support_digest: u64,
    command_identity_count: usize,
    accessibility_posture_count: usize,
    diagnostics_posture_count: usize,
}

impl WorthUiRendererSurfaceAdmission {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        policy: WorthUiHighFrequencyFramePolicy,
        support_digest: u64,
        command_identity_count: usize,
        accessibility_posture_count: usize,
        diagnostics_posture_count: usize,
    ) -> Self {
        Self {
            handle: WorthUiRendererSurfaceHandle::new(
                runtime_handle.plan_index(),
                runtime_handle.plan_generation(),
            ),
            runtime_handle,
            policy_digest: policy.canonical_digest(),
            support_digest,
            command_identity_count,
            accessibility_posture_count,
            diagnostics_posture_count,
        }
    }

    pub fn handle(self) -> WorthUiRendererSurfaceHandle {
        self.handle
    }

    pub fn runtime_handle(self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn policy_digest(self) -> u64 {
        self.policy_digest
    }

    pub fn support_digest(self) -> u64 {
        self.support_digest
    }

    pub fn command_identity_count(self) -> usize {
        self.command_identity_count
    }

    pub fn accessibility_posture_count(self) -> usize {
        self.accessibility_posture_count
    }

    pub fn diagnostics_posture_count(self) -> usize {
        self.diagnostics_posture_count
    }
}
