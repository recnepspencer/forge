#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeOverlayHook {
    host_session_identity: u64,
    host_observation_generation: u64,
    plan_basis_digest: u64,
    owner_plan_index: u32,
}

impl WorthUiRealtimeOverlayHook {
    pub(crate) fn from_host_binding(
        binding: crate::facade::WorthUiHostPlanBinding,
        plan_basis_digest: u64,
        owner_plan_index: u32,
    ) -> Self {
        Self {
            host_session_identity: binding.session_identity().as_u64(),
            host_observation_generation: binding.observation_generation().as_u64(),
            plan_basis_digest,
            owner_plan_index,
        }
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
    pub fn owner_plan_index(self) -> u32 {
        self.owner_plan_index
    }
}
