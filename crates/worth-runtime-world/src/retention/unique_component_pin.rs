use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, RelationalBranchBasisAdmissionIdentity,
};
use worth_signal::facade::branch::{AdmittedSignalBranchBasis, SignalBranchBasisAdmissionIdentity};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::ComponentBasisDependencyClass;

/// Exact component value selected by the future Runtime World retention
/// owner. The composite basis is only the source of the component value;
/// retention never keys a lease by the composite identity.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExactComponentBasis<'a> {
    Relational(&'a AdmittedRelationalBranchBasis),
    Signal(&'a AdmittedSignalBranchBasis),
}

/// Independent key used by the future unique-pin registry.
/// Descriptors and composite identities are intentionally absent.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ExactComponentBasisKey {
    Relational(RelationalBranchBasisAdmissionIdentity),
    Signal(SignalBranchBasisAdmissionIdentity),
}

/// Request to retain one exact component basis under one Runtime World
/// dependency class. A pair of these requests represents one composite
/// observation or publication, but each key is counted independently.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExactComponentPinRequest<'a> {
    owner: RuntimeWorldOwnerIdentity,
    component: ExactComponentBasis<'a>,
    dependency: ComponentBasisDependencyClass,
}

#[allow(dead_code)]
impl<'a> ExactComponentPinRequest<'a> {
    pub(crate) fn relational(
        basis: &'a AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self {
            owner: basis.owner_identity(),
            component: ExactComponentBasis::Relational(basis.relational_basis()),
            dependency,
        }
    }

    pub(crate) fn signal(
        basis: &'a AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self {
            owner: basis.owner_identity(),
            component: ExactComponentBasis::Signal(basis.signal_basis()),
            dependency,
        }
    }

    pub(crate) const fn owner(self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) const fn dependency(self) -> ComponentBasisDependencyClass {
        self.dependency
    }

    pub(crate) fn key(self) -> ExactComponentBasisKey {
        match self.component {
            ExactComponentBasis::Relational(basis) => {
                ExactComponentBasisKey::Relational(basis.admission_identity().clone())
            }
            ExactComponentBasis::Signal(basis) => {
                ExactComponentBasisKey::Signal(basis.admission_identity().clone())
            }
        }
    }
}
