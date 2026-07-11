use super::invariant_suite::S8AdmittedStrategyInvariants;
use super::{S8LayoutStrategyFamily, S8StrategyDeclaration, S8StrategyInvariantSuite};
use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::key_domain::PhysicalKeyDomainWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AdmittedLayoutStrategy {
    pub(super) declaration: S8StrategyDeclaration,
    pub(super) invariants: S8AdmittedStrategyInvariants,
}

impl S8AdmittedLayoutStrategy {
    pub(super) const fn new(
        declaration: S8StrategyDeclaration,
        invariants: S8AdmittedStrategyInvariants,
    ) -> Self {
        Self {
            declaration,
            invariants,
        }
    }

    pub const fn family(&self) -> S8LayoutStrategyFamily {
        self.declaration.family()
    }
    pub const fn invariant_suite(&self) -> S8StrategyInvariantSuite {
        self.invariants.suite()
    }
    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.declaration.key_domain()
    }
    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.declaration.lifecycle()
    }
}
