use crate::catalog::PhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MaterializationStateClass {
    DeclaredOnly,
    Absent,
    EmptyInitialized,
    Building,
    PartiallyCovered,
    Exact,
    ExactThroughPhysicalBasis,
    Lagged,
    Stale,
    RebuildRequired,
    Migrating,
    Quarantined,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutMaterializationState {
    family: PhysicalArtifactFamily,
    class: S8MaterializationStateClass,
}

impl S8LayoutMaterializationState {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        class: S8MaterializationStateClass,
    ) -> Self {
        Self { family, class }
    }

    pub(crate) const fn declared_only(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::DeclaredOnly)
    }

    pub(crate) const fn absent(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Absent)
    }

    pub(crate) const fn empty_initialized(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::EmptyInitialized)
    }

    pub(crate) const fn building(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Building)
    }

    pub(crate) const fn partially_covered(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::PartiallyCovered)
    }

    pub(crate) const fn exact(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Exact)
    }

    pub(crate) const fn exact_through_physical_basis(family: PhysicalArtifactFamily) -> Self {
        Self::new(
            family,
            S8MaterializationStateClass::ExactThroughPhysicalBasis,
        )
    }

    pub(crate) const fn lagged(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Lagged)
    }

    pub(crate) const fn stale(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Stale)
    }

    pub(crate) const fn rebuild_required(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::RebuildRequired)
    }

    pub(crate) const fn migrating(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Migrating)
    }

    pub(crate) const fn quarantined(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Quarantined)
    }

    pub(crate) const fn retired(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, S8MaterializationStateClass::Retired)
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn class(self) -> S8MaterializationStateClass {
        self.class
    }

    pub const fn supports_exact_access(self) -> bool {
        matches!(
            self.class,
            S8MaterializationStateClass::Exact
                | S8MaterializationStateClass::ExactThroughPhysicalBasis
                | S8MaterializationStateClass::EmptyInitialized
        )
    }
}
