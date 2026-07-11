use forge_store_security::{
    StoreAuthenticityRequirement, StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{AdmittedBackendCapabilityWitness, BackendTargetProfile, CapabilityEvidenceClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendQueueExecutionAdaptation {
    None,
    SplitIoVector,
    RetryShortWrite,
    RetryPartialRead,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendQueueExecutionPostureDenial {
    MissingPositiveGroupingBasis,
    UnsupportedMechanicalAdaptation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendQueueExecutionBackpressure {
    QueueDepthSaturated,
    BandwidthSaturated,
    FlushDelayed,
    WriteBackWindowSaturated,
    ReadAheadDenied,
    BackgroundYielded,
    BackendTemporarilySaturated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionPosture {
    profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    adaptation: BackendQueueExecutionAdaptation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionPlanBinding {
    primary: BackendQueueExecutionReplayBinding,
    secondary: Option<BackendQueueExecutionReplayBinding>,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    grouped_writes: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionReplayBinding {
    work_class: u8,
    backend_requirement: u8,
    durability_class: u8,
    security_scope_identity: StoreSecurityScopeIdentity,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    flush_epoch: u64,
    recovery_ordering: u8,
    writeback_policy: u8,
    requested_budget: BackendQueueExecutionBudgetBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionBudgetBinding {
    queue_slots: u64,
    bandwidth_tokens: u64,
    flush_permits: u64,
    sync_debt: u64,
    read_ahead_window: u64,
    write_back_window: u64,
    dirty_page_budget: u64,
    worker_permits: u64,
    cache_residency_hints: u64,
    reclaim_permits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionCompletion {
    pub(super) binding: BackendQueueExecutionPlanBinding,
    pub(super) posture: BackendQueueExecutionPosture,
    pub(super) queue_depth_sample: u32,
    pub(super) read_ahead_units: u64,
    pub(super) read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    pub(super) write_back_units: u64,
    pub(super) write_back_scope: Option<BackendQueueSpeculativeScope>,
    pub(super) mechanical_retries: u64,
    pub(super) partial_read_events: u64,
    pub(super) short_write_events: u64,
    pub(super) backpressure: Option<BackendQueueExecutionBackpressure>,
    pub(super) foreground_wait_events: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueSpeculativeScope {
    pub(super) security_scope_identity: StoreSecurityScopeIdentity,
    pub(super) tenant_scope: StoreTenantScope,
    pub(super) key_scope: StoreKeyScope,
}

impl BackendQueueExecutionPlanBinding {
    pub const fn from_store_replay_binding(
        primary: BackendQueueExecutionReplayBinding,
        secondary: Option<BackendQueueExecutionReplayBinding>,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
        grouped_writes: u32,
    ) -> Self {
        Self {
            primary,
            secondary,
            backend_profile,
            backend_evidence_class,
            grouped_writes,
        }
    }

    pub const fn primary(self) -> BackendQueueExecutionReplayBinding {
        self.primary
    }

    pub const fn secondary(self) -> Option<BackendQueueExecutionReplayBinding> {
        self.secondary
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn grouped_writes(self) -> u32 {
        self.grouped_writes
    }
}

impl BackendQueueExecutionReplayBinding {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_store_queue_replay(
        work_class: u8,
        backend_requirement: u8,
        durability_class: u8,
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        flush_epoch: u64,
        recovery_ordering: u8,
        writeback_policy: u8,
        requested_budget: BackendQueueExecutionBudgetBinding,
    ) -> Self {
        Self {
            work_class,
            backend_requirement,
            durability_class,
            security_scope_identity,
            tenant_scope,
            key_scope,
            authenticity_requirement,
            flush_epoch,
            recovery_ordering,
            writeback_policy,
            requested_budget,
        }
    }

    pub const fn backend_requirement(self) -> u8 {
        self.backend_requirement
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

impl BackendQueueExecutionBudgetBinding {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        queue_slots: u64,
        bandwidth_tokens: u64,
        flush_permits: u64,
        sync_debt: u64,
        read_ahead_window: u64,
        write_back_window: u64,
        dirty_page_budget: u64,
        worker_permits: u64,
        cache_residency_hints: u64,
        reclaim_permits: u64,
    ) -> Self {
        Self {
            queue_slots,
            bandwidth_tokens,
            flush_permits,
            sync_debt,
            read_ahead_window,
            write_back_window,
            dirty_page_budget,
            worker_permits,
            cache_residency_hints,
            reclaim_permits,
        }
    }
}

impl BackendQueueExecutionPosture {
    pub const fn from_admitted_capability(
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> Result<Self, BackendQueueExecutionPostureDenial> {
        Ok(Self {
            profile: witness.profile(),
            evidence_class: witness.evidence_class(),
            adaptation,
        })
    }

    pub(crate) const fn from_admitted_capability_unchecked(
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> Self {
        Self {
            profile: witness.profile(),
            evidence_class: witness.evidence_class(),
            adaptation,
        }
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn adaptation(self) -> BackendQueueExecutionAdaptation {
        self.adaptation
    }
}
