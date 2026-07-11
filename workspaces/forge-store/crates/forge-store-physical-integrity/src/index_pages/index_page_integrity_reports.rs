use crate::{IndexPageIntegrityCounters, ManifestReferenceBasis, PhysicalScopeBasis};
use forge_store_physical_format::{PhysicalGenerationOwner, PhysicalReferenceScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexPageIntegrityReport {
    derived_basis: PhysicalScopeBasis,
    classification: DerivedDamageClassification,
    counters: IndexPageIntegrityCounters,
}

impl IndexPageIntegrityReport {
    pub(crate) const fn new(
        derived_basis: PhysicalScopeBasis,
        classification: DerivedDamageClassification,
        counters: IndexPageIntegrityCounters,
    ) -> Self {
        Self {
            derived_basis,
            classification,
            counters,
        }
    }

    pub const fn derived_basis(&self) -> &PhysicalScopeBasis {
        &self.derived_basis
    }

    pub const fn damage_classification(&self) -> &DerivedDamageClassification {
        &self.classification
    }

    pub const fn counters(&self) -> IndexPageIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedDamageClassification {
    IntactIndexPage(IntactIndexPageBoundary),
    RebuildableDerived(RebuildableDerivedDamage),
    Indeterminate(IndeterminatePhysicalDamage),
    UnrecoverableAuthority(UnrecoverableAuthorityDamage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntactIndexPageBoundary {
    scope: PhysicalReferenceScope,
}

impl IntactIndexPageBoundary {
    pub(crate) const fn new(scope: PhysicalReferenceScope) -> Self {
        Self { scope }
    }

    pub const fn scope(&self) -> PhysicalReferenceScope {
        self.scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildableDerivedDamage {
    damaged_scope: PhysicalReferenceScope,
    prerequisites: RebuildableDerivedDamagePrerequisites,
    rebuild_input: DerivedRebuildInput,
}

impl RebuildableDerivedDamage {
    pub(crate) const fn new(
        damaged_scope: PhysicalReferenceScope,
        prerequisites: RebuildableDerivedDamagePrerequisites,
        rebuild_input: DerivedRebuildInput,
    ) -> Self {
        Self {
            damaged_scope,
            prerequisites,
            rebuild_input,
        }
    }

    pub const fn damaged_scope(&self) -> PhysicalReferenceScope {
        self.damaged_scope
    }

    pub const fn prerequisites(&self) -> &RebuildableDerivedDamagePrerequisites {
        &self.prerequisites
    }

    pub const fn rebuild_input(&self) -> &DerivedRebuildInput {
        &self.rebuild_input
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildableDerivedDamagePrerequisites {
    damaged_scope: PhysicalReferenceScope,
    authority_basis: ManifestReferenceBasis,
    authority_owner: PhysicalGenerationOwner,
}

impl RebuildableDerivedDamagePrerequisites {
    pub(crate) const fn new(
        damaged_scope: PhysicalReferenceScope,
        authority_basis: ManifestReferenceBasis,
        authority_owner: PhysicalGenerationOwner,
    ) -> Self {
        Self {
            damaged_scope,
            authority_basis,
            authority_owner,
        }
    }

    pub const fn damaged_scope(&self) -> PhysicalReferenceScope {
        self.damaged_scope
    }

    pub const fn authority_basis(&self) -> &ManifestReferenceBasis {
        &self.authority_basis
    }

    pub const fn authority_owner(&self) -> PhysicalGenerationOwner {
        self.authority_owner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedRebuildInput {
    damaged_scope: PhysicalReferenceScope,
    authority_owner: PhysicalGenerationOwner,
}

impl DerivedRebuildInput {
    pub(crate) const fn new(
        damaged_scope: PhysicalReferenceScope,
        authority_owner: PhysicalGenerationOwner,
    ) -> Self {
        Self {
            damaged_scope,
            authority_owner,
        }
    }

    pub const fn damaged_scope(&self) -> PhysicalReferenceScope {
        self.damaged_scope
    }

    pub const fn authority_owner(&self) -> PhysicalGenerationOwner {
        self.authority_owner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminatePhysicalDamage {
    scope: PhysicalReferenceScope,
    missing_prerequisite: RebuildabilityPrerequisite,
}

impl IndeterminatePhysicalDamage {
    pub(crate) const fn new(
        scope: PhysicalReferenceScope,
        missing_prerequisite: RebuildabilityPrerequisite,
    ) -> Self {
        Self {
            scope,
            missing_prerequisite,
        }
    }

    pub const fn scope(&self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn missing_prerequisite(&self) -> RebuildabilityPrerequisite {
        self.missing_prerequisite
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebuildabilityPrerequisite {
    CurrentAuthorityBasis,
    GenerationLink,
    ExecutedManifestAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnrecoverableAuthorityDamage {
    boundary: AuthorityDamageBoundary,
    locality: Option<PhysicalGenerationOwner>,
}

impl UnrecoverableAuthorityDamage {
    pub(crate) const fn new(
        boundary: AuthorityDamageBoundary,
        locality: Option<PhysicalGenerationOwner>,
    ) -> Self {
        Self { boundary, locality }
    }

    pub const fn boundary(&self) -> AuthorityDamageBoundary {
        self.boundary
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityDamageBoundary {
    RootManifest,
    SegmentManifest,
    AllocationMap,
    ManifestReferenceTable,
    BackendResidue,
}
