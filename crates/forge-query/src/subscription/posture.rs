#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionCostPosture {
    BoundedExact,
    BoundedMembership,
    BoundedWithViewGrouping,
    DeniedWouldWiden,
    DeferredStoreBacked,
    DebtExplicit,
}

impl QuerySubscriptionCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BoundedExact => "bounded_exact",
            Self::BoundedMembership => "bounded_membership",
            Self::BoundedWithViewGrouping => "bounded_with_view_grouping",
            Self::DeniedWouldWiden => "denied_would_widen",
            Self::DeferredStoreBacked => "deferred_store_backed",
            Self::DebtExplicit => "debt_explicit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionBasisPosture {
    CurrentHead,
    BranchHead,
    RuntimeHistoricalSnapshot,
    PreviewScoped,
    DeniedUnsupportedBasis,
}

impl QuerySubscriptionBasisPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead => "branch_head",
            Self::RuntimeHistoricalSnapshot => "runtime_historical_snapshot",
            Self::PreviewScoped => "preview_scoped",
            Self::DeniedUnsupportedBasis => "denied_unsupported_basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionBridgePosture {
    BridgeDeclarationAdmitted,
    BridgeFamilyUnsupported,
    BridgeSliceUnsupported,
    BridgeBasisBindingDenied,
    BridgeLoweringDeferred,
}

impl QuerySubscriptionBridgePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BridgeDeclarationAdmitted => "bridge_declaration_admitted",
            Self::BridgeFamilyUnsupported => "bridge_family_unsupported",
            Self::BridgeSliceUnsupported => "bridge_slice_unsupported",
            Self::BridgeBasisBindingDenied => "bridge_basis_binding_denied",
            Self::BridgeLoweringDeferred => "bridge_lowering_deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionAllocationPosture {
    NoAllocation,
    ScratchBufferOnly,
    DeniedAllocationRequired,
}

impl QuerySubscriptionAllocationPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoAllocation => "no_allocation",
            Self::ScratchBufferOnly => "scratch_buffer_only",
            Self::DeniedAllocationRequired => "denied_allocation_required",
        }
    }
}
