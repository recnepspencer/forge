#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphIndexPosture {
    Verified,
    RuntimeMaintained,
    LowerRuntimeOwned,
    EphemeralAvailable,
    RequiresStoreBackedPersistentIndex,
    RequiresAccessCapabilityRegistration,
    TemporarilyUnavailable,
    Denied,
}

impl WorthQueryGraphIndexPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::RuntimeMaintained => "runtime_maintained",
            Self::LowerRuntimeOwned => "lower_runtime_owned",
            Self::EphemeralAvailable => "ephemeral_available",
            Self::RequiresStoreBackedPersistentIndex => "requires_store_backed_persistent_index",
            Self::RequiresAccessCapabilityRegistration => "requires_access_capability_registration",
            Self::TemporarilyUnavailable => "temporarily_unavailable",
            Self::Denied => "denied",
        }
    }

    pub fn is_supported(&self) -> bool {
        matches!(
            self,
            Self::Verified
                | Self::RuntimeMaintained
                | Self::LowerRuntimeOwned
                | Self::EphemeralAvailable
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphIndexSupportState {
    Declared,
    Measured,
    Certified,
    Available,
    TemporarilyUnavailable,
    StoreOwnedUnavailable,
    Unsupported,
}

impl WorthQueryGraphIndexSupportState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Declared => "declared",
            Self::Measured => "measured",
            Self::Certified => "certified",
            Self::Available => "available",
            Self::TemporarilyUnavailable => "temporarily_unavailable",
            Self::StoreOwnedUnavailable => "store_owned_unavailable",
            Self::Unsupported => "unsupported",
        }
    }

    pub fn certifies_verified_support(&self) -> bool {
        matches!(self, Self::Certified | Self::Available)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphIndexInventoryMatchOutcome {
    ExactMatch,
    DirectionMismatch,
    PredicateMismatch,
    OrderingMismatch,
    LifecycleMismatch,
    ComplexityMismatch,
    RebuildBasisMismatch,
    InvalidationBasisMismatch,
    MissingSupportRow,
}

impl WorthQueryGraphIndexInventoryMatchOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactMatch => "exact_match",
            Self::DirectionMismatch => "direction_mismatch",
            Self::PredicateMismatch => "predicate_mismatch",
            Self::OrderingMismatch => "ordering_mismatch",
            Self::LifecycleMismatch => "lifecycle_mismatch",
            Self::ComplexityMismatch => "complexity_mismatch",
            Self::RebuildBasisMismatch => "rebuild_basis_mismatch",
            Self::InvalidationBasisMismatch => "invalidation_basis_mismatch",
            Self::MissingSupportRow => "missing_support_row",
        }
    }
}
