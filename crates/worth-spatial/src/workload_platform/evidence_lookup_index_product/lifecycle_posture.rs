#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexLifecyclePostureKind {
    SparseLookupOnly,
    BoundedDenseConstruction,
    DeclarationOnlyNoIndex,
    EquivalentReuse,
    PersistentCapabilityRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexLifecyclePosture {
    kind: EvidenceLookupIndexLifecyclePostureKind,
}

impl EvidenceLookupIndexLifecyclePosture {
    pub(crate) const fn sparse_lookup_only() -> Self {
        Self {
            kind: EvidenceLookupIndexLifecyclePostureKind::SparseLookupOnly,
        }
    }

    pub(crate) const fn bounded_dense_construction() -> Self {
        Self {
            kind: EvidenceLookupIndexLifecyclePostureKind::BoundedDenseConstruction,
        }
    }

    pub(crate) const fn declaration_only_no_index() -> Self {
        Self {
            kind: EvidenceLookupIndexLifecyclePostureKind::DeclarationOnlyNoIndex,
        }
    }

    pub(crate) const fn equivalent_reuse() -> Self {
        Self {
            kind: EvidenceLookupIndexLifecyclePostureKind::EquivalentReuse,
        }
    }

    pub(crate) const fn persistent_capability_required() -> Self {
        Self {
            kind: EvidenceLookupIndexLifecyclePostureKind::PersistentCapabilityRequired,
        }
    }

    pub const fn kind(&self) -> EvidenceLookupIndexLifecyclePostureKind {
        self.kind
    }

    pub const fn claims_persistent_capability(&self) -> bool {
        matches!(
            self.kind,
            EvidenceLookupIndexLifecyclePostureKind::PersistentCapabilityRequired
        )
    }
}
