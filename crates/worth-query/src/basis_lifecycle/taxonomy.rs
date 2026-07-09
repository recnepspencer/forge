#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BasisFamily {
    CurrentHead,
    BranchHead,
    BranchSnapshot,
    Preview,
    PreviewDerived,
    RuntimeSnapshot,
    HistoricalSnapshot,
    TenantScoped,
    PolicyScoped,
    StoreBacked,
    DurableReload,
}

impl BasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead => "branch_head",
            Self::BranchSnapshot => "branch_snapshot",
            Self::Preview => "preview",
            Self::PreviewDerived => "preview_derived",
            Self::RuntimeSnapshot => "runtime_snapshot",
            Self::HistoricalSnapshot => "historical_snapshot",
            Self::TenantScoped => "tenant_scoped",
            Self::PolicyScoped => "policy_scoped",
            Self::StoreBacked => "store_backed",
            Self::DurableReload => "durable_reload",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisAuthorityPosture {
    Runtime,
    RelationalFacade,
    RuntimeBridgeFacade,
    SignalFacade,
    StoreDeferred,
}

impl BasisAuthorityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::RelationalFacade => "relational_facade",
            Self::RuntimeBridgeFacade => "runtime_bridge_facade",
            Self::SignalFacade => "signal_facade",
            Self::StoreDeferred => "store_deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisScopePosture {
    Global,
    Branch,
    Snapshot,
    Preview,
    Tenant,
    PolicyTenant,
    FutureNeighbor,
}

impl BasisScopePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Branch => "branch",
            Self::Snapshot => "snapshot",
            Self::Preview => "preview",
            Self::Tenant => "tenant",
            Self::PolicyTenant => "policy_tenant",
            Self::FutureNeighbor => "future_neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisVisibilityPosture {
    Full,
    Advisory,
    PolicyMasked,
    Deferred,
}

impl BasisVisibilityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Advisory => "advisory",
            Self::PolicyMasked => "policy_masked",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecyclePosture {
    Current,
    SnapshotPinned,
    PreviewActive,
    PreviewStale,
    HistoricalRetained,
    DeferredFuture,
}

impl BasisLifecyclePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::SnapshotPinned => "snapshot_pinned",
            Self::PreviewActive => "preview_active",
            Self::PreviewStale => "preview_stale",
            Self::HistoricalRetained => "historical_retained",
            Self::DeferredFuture => "deferred_future",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisIntentDenialKind {
    Malformed,
    Ambiguous,
    TemporalDeferred,
    AsyncResourceDeferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeniedBasisCapabilityKind {
    Inaccessible,
    PolicyMasked,
    TenantMismatched,
    SchemaIncompatible,
    OperationIneligible,
    PreviewDrifted,
    HistoricalReplayUnsupported,
    LowerRuntimeBindingMissing,
    BridgeAuthorityMismatch,
    RelationalAuthorityMismatch,
    SignalObservationMissing,
    RuntimeSnapshotStale,
    LowerRuntimeCapabilityUnsupported,
    DurableOverclaim,
    StoreBackedDeferred,
}

impl DeniedBasisCapabilityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inaccessible => "inaccessible",
            Self::PolicyMasked => "policy_masked",
            Self::TenantMismatched => "tenant_mismatched",
            Self::SchemaIncompatible => "schema_incompatible",
            Self::OperationIneligible => "operation_ineligible",
            Self::PreviewDrifted => "preview_drifted",
            Self::HistoricalReplayUnsupported => "historical_replay_unsupported",
            Self::LowerRuntimeBindingMissing => "lower_runtime_binding_missing",
            Self::BridgeAuthorityMismatch => "bridge_authority_mismatch",
            Self::RelationalAuthorityMismatch => "relational_authority_mismatch",
            Self::SignalObservationMissing => "signal_observation_missing",
            Self::RuntimeSnapshotStale => "runtime_snapshot_stale",
            Self::LowerRuntimeCapabilityUnsupported => "lower_runtime_capability_unsupported",
            Self::DurableOverclaim => "durable_overclaim",
            Self::StoreBackedDeferred => "store_backed_deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisEligibilityDenialCause {
    Inaccessible,
    PolicyMasked,
    TenantMismatched,
    SchemaIncompatible,
}

impl BasisEligibilityDenialCause {
    pub fn denied_capability_kind(&self) -> DeniedBasisCapabilityKind {
        match self {
            Self::Inaccessible => DeniedBasisCapabilityKind::Inaccessible,
            Self::PolicyMasked => DeniedBasisCapabilityKind::PolicyMasked,
            Self::TenantMismatched => DeniedBasisCapabilityKind::TenantMismatched,
            Self::SchemaIncompatible => DeniedBasisCapabilityKind::SchemaIncompatible,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inaccessible => "inaccessible",
            Self::PolicyMasked => "policy_masked",
            Self::TenantMismatched => "tenant_mismatched",
            Self::SchemaIncompatible => "schema_incompatible",
        }
    }
}
