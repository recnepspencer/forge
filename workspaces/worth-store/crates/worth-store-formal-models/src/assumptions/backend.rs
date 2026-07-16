use worth_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, BackendDurabilitySupport,
    WalDurabilityBarrierSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeledBackendDurabilityAssumption {
    runtime_profile: BackendDurabilityProfileId,
    required_barriers: WalDurabilityBarrierSet,
    support: BackendDurabilitySupport,
}

impl ModeledBackendDurabilityAssumption {
    pub fn from_runtime_profile<P: BackendDurabilityProfile>() -> Self {
        Self {
            runtime_profile: P::ID,
            required_barriers: P::REQUIRED_BARRIERS,
            support: P::SUPPORT,
        }
    }

    pub const fn runtime_profile(self) -> BackendDurabilityProfileId {
        self.runtime_profile
    }

    pub const fn required_barriers(self) -> WalDurabilityBarrierSet {
        self.required_barriers
    }

    pub const fn support(self) -> BackendDurabilitySupport {
        self.support
    }
}
