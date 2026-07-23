#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQuerySharedExecutionOwnerIdentity {
    runtime_authority: u64,
    slot: u64,
    generation: u64,
}

impl WorthQuerySharedExecutionOwnerIdentity {
    pub(crate) const fn new(runtime_authority: u64, slot: u64, generation: u64) -> Self {
        Self {
            runtime_authority,
            slot,
            generation,
        }
    }

    pub const fn runtime_authority(self) -> u64 {
        self.runtime_authority
    }

    pub const fn slot(self) -> u64 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQuerySharedProjectionLeaseIdentity {
    runtime_authority: u64,
    slot: u64,
    generation: u64,
}

impl WorthQuerySharedProjectionLeaseIdentity {
    pub(crate) const fn new(runtime_authority: u64, slot: u64, generation: u64) -> Self {
        Self {
            runtime_authority,
            slot,
            generation,
        }
    }

    pub const fn runtime_authority(self) -> u64 {
        self.runtime_authority
    }

    pub const fn slot(self) -> u64 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthQuerySharedProjectionLeaseToken {
    owner: WorthQuerySharedExecutionOwnerIdentity,
    lease: WorthQuerySharedProjectionLeaseIdentity,
}

impl WorthQuerySharedProjectionLeaseToken {
    pub(crate) const fn new(
        owner: WorthQuerySharedExecutionOwnerIdentity,
        lease: WorthQuerySharedProjectionLeaseIdentity,
    ) -> Self {
        Self { owner, lease }
    }

    pub(crate) const fn owner(&self) -> WorthQuerySharedExecutionOwnerIdentity {
        self.owner
    }

    pub(crate) const fn lease(&self) -> WorthQuerySharedProjectionLeaseIdentity {
        self.lease
    }
}
