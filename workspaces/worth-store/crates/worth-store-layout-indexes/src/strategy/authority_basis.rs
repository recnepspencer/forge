use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::{AdmittedPhysicalKeyDomain, PhysicalKeyDomainWitness};
use crate::AdmittedPhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StrategyAuthorityBasis {
    family: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
}

impl StrategyAuthorityBasis {
    pub const fn admitted(
        family: AdmittedPhysicalArtifactFamily,
        key_domain: AdmittedPhysicalKeyDomain,
    ) -> Self {
        Self { family, key_domain }
    }

    pub const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.family.lifecycle()
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        self.key_domain.witness()
    }

    pub const fn admitted_family(self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }

    pub const fn admitted_key_domain(self) -> AdmittedPhysicalKeyDomain {
        self.key_domain
    }
}
