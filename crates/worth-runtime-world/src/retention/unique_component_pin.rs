use crate::basis::AdmittedCompositeRuntimeWorldBasis;

use super::ComponentBasisDependencyClass;

/// Request to retain both exact component bases carried by one admitted
/// composite basis under one Runtime World dependency class. It carries no
/// descriptor, lease, owner runtime, or caller-controlled identity.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExactComponentPinRequest<'a> {
    basis: &'a AdmittedCompositeRuntimeWorldBasis,
    dependency: ComponentBasisDependencyClass,
}

impl<'a> ExactComponentPinRequest<'a> {
    pub(crate) const fn new(
        basis: &'a AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self { basis, dependency }
    }

    pub(crate) const fn basis(self) -> &'a AdmittedCompositeRuntimeWorldBasis {
        self.basis
    }

    pub(crate) const fn dependency(self) -> ComponentBasisDependencyClass {
        self.dependency
    }
}
