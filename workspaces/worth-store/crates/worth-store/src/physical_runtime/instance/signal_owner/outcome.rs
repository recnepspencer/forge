#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSignalRuntimeIdentity([u8; 16]);

impl PhysicalSignalRuntimeIdentity {
    pub(super) const fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalConstructionFailure {
    IdentityEntropyUnavailable,
    WorkerSpawnRejected,
    WorkerReadinessLost,
    CapabilityDeclarationRejected,
    DependencyInitializationRejected,
    ClockBridgeUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalShutdownOutcome {
    Disposed,
    OwnerRevoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSignalClockObservation {
    current_tick: u64,
    last_advance_ordinal: u64,
}

impl PhysicalSignalClockObservation {
    pub(super) const fn new(current_tick: u64, last_advance_ordinal: u64) -> Self {
        Self {
            current_tick,
            last_advance_ordinal,
        }
    }
    pub const fn current_tick(self) -> u64 {
        self.current_tick
    }
    pub const fn last_advance_ordinal(self) -> u64 {
        self.last_advance_ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalClockObservationFailure {
    OwnerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalDeltaApplicationFailure {
    BindingNotInstalled,
    VersionExhausted,
    SignalMutationRejected,
    SignalEvaluationRejected,
    OwnerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSignalObservation {
    profile: crate::physical_runtime::PhysicalSignalProfileIdentity,
    graph_owner_count: u8,
    aspect_binding_count: u16,
    locality_owner_count: u16,
    async_family_count: u8,
    clock: PhysicalSignalClockObservation,
}

impl PhysicalSignalObservation {
    pub(super) const fn new(
        profile: crate::physical_runtime::PhysicalSignalProfileIdentity,
        graph_owner_count: u8,
        aspect_binding_count: u16,
        locality_owner_count: u16,
        async_family_count: u8,
        clock: PhysicalSignalClockObservation,
    ) -> Self {
        Self {
            profile,
            graph_owner_count,
            aspect_binding_count,
            locality_owner_count,
            async_family_count,
            clock,
        }
    }

    pub const fn profile(self) -> crate::physical_runtime::PhysicalSignalProfileIdentity {
        self.profile
    }

    pub const fn aspect_binding_count(self) -> u16 {
        self.aspect_binding_count
    }

    pub const fn graph_owner_count(self) -> u8 {
        self.graph_owner_count
    }

    pub const fn locality_owner_count(self) -> u16 {
        self.locality_owner_count
    }

    pub const fn async_family_count(self) -> u8 {
        self.async_family_count
    }

    pub const fn clock(self) -> PhysicalSignalClockObservation {
        self.clock
    }
}
