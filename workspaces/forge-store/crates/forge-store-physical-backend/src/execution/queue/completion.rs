use super::{
    BackendQueueExecutionBackpressure, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding, BackendQueueExecutionPosture, BackendQueueSpeculativeScope,
};
use forge_store_security::{StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope};

impl BackendQueueExecutionCompletion {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn from_backend_ticket(
        binding: BackendQueueExecutionPlanBinding,
        posture: BackendQueueExecutionPosture,
        queue_depth_sample: u32,
        read_ahead_units: u64,
        read_ahead_scope: Option<BackendQueueSpeculativeScope>,
        write_back_units: u64,
        write_back_scope: Option<BackendQueueSpeculativeScope>,
        mechanical_retries: u64,
        partial_read_events: u64,
        short_write_events: u64,
        backpressure: Option<BackendQueueExecutionBackpressure>,
        foreground_wait_events: u64,
    ) -> Self {
        Self {
            binding,
            posture,
            queue_depth_sample,
            read_ahead_units,
            read_ahead_scope,
            write_back_units,
            write_back_scope,
            mechanical_retries,
            partial_read_events,
            short_write_events,
            backpressure,
            foreground_wait_events,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification(
        binding: BackendQueueExecutionPlanBinding,
        posture: BackendQueueExecutionPosture,
    ) -> Self {
        Self {
            binding,
            posture,
            queue_depth_sample: 0,
            read_ahead_units: 0,
            read_ahead_scope: None,
            write_back_units: 0,
            write_back_scope: None,
            mechanical_retries: 0,
            partial_read_events: 0,
            short_write_events: 0,
            backpressure: None,
            foreground_wait_events: 0,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_queue_depth(mut self, queue_depth_sample: u32) -> Self {
        self.queue_depth_sample = queue_depth_sample;
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_read_ahead(
        mut self,
        units: u64,
        scope: BackendQueueSpeculativeScope,
    ) -> Self {
        self.read_ahead_units = units;
        self.read_ahead_scope = Some(scope);
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_write_back(
        mut self,
        units: u64,
        scope: BackendQueueSpeculativeScope,
    ) -> Self {
        self.write_back_units = units;
        self.write_back_scope = Some(scope);
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_mechanical_adaptation(
        mut self,
        retries: u64,
        partial_reads: u64,
        short_writes: u64,
    ) -> Self {
        self.mechanical_retries = retries;
        self.partial_read_events = partial_reads;
        self.short_write_events = short_writes;
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_backpressure(
        mut self,
        backpressure: BackendQueueExecutionBackpressure,
    ) -> Self {
        self.backpressure = Some(backpressure);
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn observe_foreground_wait_events(mut self, foreground_wait_events: u64) -> Self {
        self.foreground_wait_events = foreground_wait_events;
        self
    }

    pub const fn binding(self) -> BackendQueueExecutionPlanBinding {
        self.binding
    }

    pub const fn grouped_writes(self) -> u32 {
        self.binding.grouped_writes()
    }

    pub const fn posture(self) -> BackendQueueExecutionPosture {
        self.posture
    }

    pub const fn queue_depth_sample(self) -> u32 {
        self.queue_depth_sample
    }

    pub const fn read_ahead_units(self) -> u64 {
        self.read_ahead_units
    }

    pub const fn read_ahead_scope(self) -> Option<BackendQueueSpeculativeScope> {
        self.read_ahead_scope
    }

    pub const fn write_back_units(self) -> u64 {
        self.write_back_units
    }

    pub const fn write_back_scope(self) -> Option<BackendQueueSpeculativeScope> {
        self.write_back_scope
    }

    pub const fn mechanical_retries(self) -> u64 {
        self.mechanical_retries
    }

    pub const fn partial_read_events(self) -> u64 {
        self.partial_read_events
    }

    pub const fn short_write_events(self) -> u64 {
        self.short_write_events
    }

    pub const fn backpressure(self) -> Option<BackendQueueExecutionBackpressure> {
        self.backpressure
    }

    pub const fn foreground_wait_events(self) -> u64 {
        self.foreground_wait_events
    }
}

impl BackendQueueSpeculativeScope {
    pub const fn admitted(
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
    ) -> Self {
        Self {
            security_scope_identity,
            tenant_scope,
            key_scope,
        }
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }
}
