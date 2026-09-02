use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, RelationalBranchBasisAdmissionIdentity,
};
use worth_signal::facade::branch::{AdmittedSignalBranchBasis, SignalBranchBasisAdmissionIdentity};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;

use super::ComponentBasisDependencyClass;

/// Exact component value selected by a Runtime World retention owner. The
/// composite basis is only the source of the owner-issued component value;
/// retention never keys a lease by the composite identity.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExactComponentBasis<'a> {
    Relational(&'a AdmittedRelationalBranchBasis),
    Signal(&'a AdmittedSignalBranchBasis),
}

/// Independent owner-issued key used by the future unique-pin registry.
/// Descriptors and composite identities are intentionally absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ExactComponentBasisKey {
    Relational(RelationalBranchBasisAdmissionIdentity),
    Signal(SignalBranchBasisAdmissionIdentity),
}

/// Request to retain one exact component basis under one Runtime World
/// dependency class. A pair of these requests represents one composite
/// observation or publication, but each key is counted independently.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExactComponentPinRequest<'a> {
    component: ExactComponentBasis<'a>,
    dependency: ComponentBasisDependencyClass,
}

impl<'a> ExactComponentPinRequest<'a> {
    pub(crate) fn relational(
        basis: &'a AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self {
            component: ExactComponentBasis::Relational(basis.relational_basis()),
            dependency,
        }
    }

    pub(crate) fn signal(
        basis: &'a AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self {
            component: ExactComponentBasis::Signal(basis.signal_basis()),
            dependency,
        }
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
