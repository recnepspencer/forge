use super::invariant_suite::AdmittedStrategyInvariants;
use super::{LayoutStrategyFamily, StrategyDeclaration, StrategyInvariantSuite};
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::{AdmittedPhysicalArtifactFamily, AdmittedPhysicalKeyDomain};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedLayoutStrategy {
    pub(super) declaration: StrategyDeclaration,
    pub(super) invariants: AdmittedStrategyInvariants,
}

impl AdmittedLayoutStrategy {
    pub(super) const fn new(
        declaration: StrategyDeclaration,
        invariants: AdmittedStrategyInvariants,
    ) -> Self {
        Self {
            declaration,
            invariants,
        }
    }

    pub const fn family(&self) -> LayoutStrategyFamily {
        self.declaration.family()
    }
    pub const fn invariant_suite(&self) -> StrategyInvariantSuite {
        self.invariants.suite()
    }
    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.declaration.key_domain()
    }
    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.declaration.lifecycle()
    }

    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.declaration.authority_basis().admitted_family()
    }

    pub const fn admitted_key_domain(&self) -> AdmittedPhysicalKeyDomain {
        self.declaration.authority_basis().admitted_key_domain()
    }
}
