use worth_store_physical_integrity::PhysicalArtifactScope;

/// Future owner-issued truth for a real derived family and its exact current basis.
///
/// C9 has no current derived family, so this type intentionally has no issuer.
/// The first real family must add its concrete owner adapter here; index/blob
/// residue and byte similarity cannot construct it.
pub(in crate::physical_runtime) struct RebuildableDerivedArtifactOwnerTruth {
    derived_scope: PhysicalArtifactScope,
    authoritative_basis_scope: PhysicalArtifactScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RebuildablePhysicalDerivedObservation {
    damaged_derived_scope: PhysicalArtifactScope,
    intact_authoritative_basis_scope: PhysicalArtifactScope,
}

impl RebuildableDerivedArtifactOwnerTruth {
    pub(super) const fn derived_scope(&self) -> PhysicalArtifactScope {
        self.derived_scope
    }

    pub(super) const fn authoritative_basis_scope(&self) -> PhysicalArtifactScope {
        self.authoritative_basis_scope
    }
}

impl RebuildablePhysicalDerivedObservation {
    pub(super) const fn new(
        damaged_derived_scope: PhysicalArtifactScope,
        intact_authoritative_basis_scope: PhysicalArtifactScope,
    ) -> Self {
        Self {
            damaged_derived_scope,
            intact_authoritative_basis_scope,
        }
    }

    pub const fn damaged_derived_scope(self) -> PhysicalArtifactScope {
        self.damaged_derived_scope
    }

    pub const fn intact_authoritative_basis_scope(self) -> PhysicalArtifactScope {
        self.intact_authoritative_basis_scope
    }
}
