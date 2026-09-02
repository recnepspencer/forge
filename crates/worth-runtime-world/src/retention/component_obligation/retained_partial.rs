use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::{CompositeBasisIdentity, RuntimeWorldOwnerIdentity};

use super::super::unique_component_pin::{ComponentBasisPinClaim, ExactComponentPinRequest};
use super::super::ComponentBasisDependencyClass;
use super::{ComponentBasisPinObligation, IssuedComponentPinPair};

/// Two exact component claims retained with product-unpublished owner effects.
#[derive(Debug)]
pub(crate) struct RetainedPartialRetentionObligation {
    owner: RuntimeWorldOwnerIdentity,
    basis: CompositeBasisIdentity,
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl RetainedPartialRetentionObligation {
    pub(crate) fn owner_issued(pair: IssuedComponentPinPair) -> Self {
        let (owner, basis, dependency, relational, signal) = pair.into_parts();
        assert_eq!(
            dependency,
            ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
        );
        Self {
            owner,
            basis,
            relational,
            signal,
        }
    }

    pub(super) fn transferred(
        owner: RuntimeWorldOwnerIdentity,
        basis: CompositeBasisIdentity,
        relational: ComponentBasisPinClaim,
        signal: ComponentBasisPinClaim,
    ) -> Self {
        assert_eq!(relational.owner_identity(), owner);
        assert_eq!(signal.owner_identity(), owner);
        assert_eq!(
            relational.dependency(),
            ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
        );
        assert_eq!(
            signal.dependency(),
            ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
        );
        Self {
            owner,
            basis,
            relational: ComponentBasisPinObligation::new(relational),
            signal: ComponentBasisPinObligation::new(signal),
        }
    }

    pub(crate) const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) fn basis(&self) -> &CompositeBasisIdentity {
        &self.basis
    }

    pub(crate) fn relational(&self) -> &ComponentBasisPinObligation {
        &self.relational
    }

    pub(crate) fn signal(&self) -> &ComponentBasisPinObligation {
        &self.signal
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.owner == basis.owner_identity()
            && self.basis == *basis.identity()
            && self.relational.key()
                == &ExactComponentPinRequest::relational(
                    basis,
                    ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects,
                )
                .key()
            && self.signal.key()
                == &ExactComponentPinRequest::signal(
                    basis,
                    ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects,
                )
                .key()
            && self.relational.dependency()
                == ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
            && self.signal.dependency()
                == ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
    }
}
