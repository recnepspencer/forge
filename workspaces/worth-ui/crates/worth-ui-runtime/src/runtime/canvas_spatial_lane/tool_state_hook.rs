#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiSpatialToolStateHook {
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_digest: u64,
}

impl WorthUiSpatialToolStateHook {
    pub(crate) fn from_host_binding(
        binding: crate::facade::WorthUiHostPlanBinding,
        plan_digest: u64,
    ) -> Self {
        Self {
            host_session_identity: binding.session_identity().as_u64(),
            host_observation_generation: binding.observation_generation().as_u64(),
            plan_digest,
        }
    }
    pub fn hook_id(&self) -> &str {
        "canvas-spatial-tool-state"
    }
    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
    pub fn host_observation_generation(self) -> u64 {
        self.host_observation_generation
    }
    pub fn plan_digest(self) -> u64 {
        self.plan_digest
    }
    pub fn selection_identity_digest(self) -> u64 {
        self.plan_digest
    }
}
