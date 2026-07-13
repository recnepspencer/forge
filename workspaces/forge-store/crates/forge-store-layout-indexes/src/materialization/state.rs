use crate::catalog::PhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterializationStateClass {
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
pub struct LayoutMaterializationState {
    family: PhysicalArtifactFamily,
    class: MaterializationStateClass,
}

impl LayoutMaterializationState {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        class: MaterializationStateClass,
    ) -> Self {
        Self { family, class }
    }

    #[cfg(test)]
    pub(crate) const fn declared_only(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::DeclaredOnly)
    }

    #[cfg(test)]
    pub(crate) const fn absent(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::Absent)
    }

    #[cfg(test)]
    pub(crate) const fn partially_covered(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::PartiallyCovered)
    }

    #[cfg(test)]
    pub(crate) const fn exact(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::Exact)
    }

    pub(crate) const fn exact_through_physical_basis(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::ExactThroughPhysicalBasis)
    }

    #[cfg(test)]
    pub(crate) const fn lagged(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::Lagged)
    }

    #[cfg(test)]
    pub(crate) const fn stale(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::Stale)
    }

    #[cfg(test)]
    pub(crate) const fn quarantined(family: PhysicalArtifactFamily) -> Self {
        Self::new(family, MaterializationStateClass::Quarantined)
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn class(self) -> MaterializationStateClass {
        self.class
    }

    pub const fn supports_exact_access(self) -> bool {
        matches!(
            self.class,
            MaterializationStateClass::Exact
                | MaterializationStateClass::ExactThroughPhysicalBasis
                | MaterializationStateClass::EmptyInitialized
        )
    }
}
