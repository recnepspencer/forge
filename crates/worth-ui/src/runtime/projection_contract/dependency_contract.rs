use crate::runtime::{WorthUiChangedRuntimeFacts, WorthUiProjectionDependencySet};

use super::{WorthUiProjectionFamily, WorthUiProjectionIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionDependencyDeclaration {
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionDependencyValidationProof {
    dependency_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiValidatedProjectionDependencyContract {
    identity: WorthUiProjectionIdentity,
    family: WorthUiProjectionFamily,
    dependencies: WorthUiProjectionDependencySet,
    validation_proof: WorthUiProjectionDependencyValidationProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionDependencyAdmissionDenial {
    EmptyDependencies,
}

impl WorthUiProjectionDependencyDeclaration {
    pub(crate) fn from_set(dependencies: WorthUiProjectionDependencySet) -> Self {
        Self { dependencies }
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub fn digest(&self) -> u64 {
        self.dependencies.digest().value()
    }
}

impl WorthUiProjectionDependencyValidationProof {
    pub fn dependency_digest(self) -> u64 {
        self.dependency_digest
    }
}

impl WorthUiValidatedProjectionDependencyContract {
    pub(crate) fn admit(
        identity: WorthUiProjectionIdentity,
        family: WorthUiProjectionFamily,
        declaration: WorthUiProjectionDependencyDeclaration,
    ) -> Result<Self, WorthUiProjectionDependencyAdmissionDenial> {
        if declaration.is_empty() {
            return Err(WorthUiProjectionDependencyAdmissionDenial::EmptyDependencies);
        }
        let dependencies = declaration.dependencies;
        let validation_proof = WorthUiProjectionDependencyValidationProof {
            dependency_digest: dependencies.digest().value(),
        };
        Ok(Self {
            identity,
            family,
            dependencies,
            validation_proof,
        })
    }

    pub fn identity(&self) -> &WorthUiProjectionIdentity {
        &self.identity
    }

    pub fn family(&self) -> WorthUiProjectionFamily {
        self.family
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    pub fn validation_proof(&self) -> WorthUiProjectionDependencyValidationProof {
        self.validation_proof
    }

    pub fn intersects_changed_facts(&self, changed_facts: &WorthUiChangedRuntimeFacts) -> bool {
        self.dependencies.intersects(changed_facts.facts())
    }
}
