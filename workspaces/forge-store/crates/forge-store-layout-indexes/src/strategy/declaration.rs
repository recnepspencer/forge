use super::S8LayoutStrategyFamily;
use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::key_domain::PhysicalKeyDomainWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8StrategyDeclaration {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
}

impl S8StrategyDeclaration {
    pub(crate) const fn new(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        family: S8LayoutStrategyFamily,
    ) -> Self {
        Self {
            lifecycle,
            key_domain,
            family,
        }
    }

    pub const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        self.key_domain
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.family
    }
}
