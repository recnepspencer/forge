use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::{CompositeBasisIdentity, CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::super::obligation_transfer::{
    ProductHeadRetentionTransfer, RetentionTransferDenial, RetentionTransferReceipt,
};
use super::super::unique_component_pin::ExactComponentPinRequest;
use super::super::ComponentBasisDependencyClass;
use super::product_head::ProductHeadRetentionObligation;
use super::ComponentBasisPinObligation;

/// The fixed two-component proof issued by the Runtime World retention owner.
/// Its private metadata binds one owner, one exact composite basis, one
/// dependency class, and both exact component claims before any wrapper can be
/// constructed.
#[derive(Debug)]
pub(crate) struct IssuedComponentPinPair {
    owner: RuntimeWorldOwnerIdentity,
    basis: CompositeBasisIdentity,
    dependency: ComponentBasisDependencyClass,
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl IssuedComponentPinPair {
    pub(crate) fn owner_issued(
        basis: &AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
        relational: ComponentBasisPinObligation,
        signal: ComponentBasisPinObligation,
    ) -> Self {
        let owner = basis.owner_identity();
        assert_eq!(relational.owner_identity(), owner);
        assert_eq!(signal.owner_identity(), owner);
        assert_eq!(relational.dependency(), dependency);
        assert_eq!(signal.dependency(), dependency);
        assert_ne!(relational.key(), signal.key());
        assert_eq!(
            relational.key(),
            &ExactComponentPinRequest::relational(basis, dependency).key()
        );
        assert_eq!(
            signal.key(),
            &ExactComponentPinRequest::signal(basis, dependency).key()
        );
        Self {
            owner,
            basis: basis.identity().clone(),
            dependency,
            relational,
            signal,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RuntimeWorldOwnerIdentity,
        CompositeBasisIdentity,
        ComponentBasisDependencyClass,
        ComponentBasisPinObligation,
        ComponentBasisPinObligation,
    ) {
        (
            self.owner,
            self.basis,
            self.dependency,
            self.relational,
            self.signal,
        )
    }
}

/// Two exact component claims carried by one observation and bound to the
/// exact commit occurrence and basis from which the owner issued them.
#[derive(Debug)]
pub(crate) struct ObservationRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
    captured_commit: CompositeCommitIdentity,
    captured_basis: CompositeBasisIdentity,
}

impl ObservationRetentionObligation {
    pub(crate) fn owner_issued(
        commit: &CompositeRuntimeWorldCommit,
        pair: IssuedComponentPinPair,
    ) -> Self {
        let (owner, basis, dependency, relational, signal) = pair.into_parts();
        assert_eq!(commit.identity().owner_identity(), owner);
        assert_eq!(commit.basis().owner_identity(), owner);
        assert_eq!(basis, *commit.basis().identity());
        assert_eq!(
            dependency,
            ComponentBasisDependencyClass::AdmittedObservation
        );
        Self {
            relational,
            signal,
            captured_commit: commit.identity().clone(),
            captured_basis: basis,
        }
    }

    pub(crate) fn relational(&self) -> &ComponentBasisPinObligation {
        &self.relational
    }

    pub(crate) fn signal(&self) -> &ComponentBasisPinObligation {
        &self.signal
    }

    pub(crate) fn matches_captured_head(&self, commit: &CompositeRuntimeWorldCommit) -> bool {
        self.captured_commit.eq(commit.identity())
            && self.captured_basis.eq(commit.basis().identity())
    }
}

/// Two exact component claims reserved for one publication attempt. The
/// prospective successor basis is checked before transfer.
#[derive(Debug)]
pub(crate) struct PublicationRetentionObligation {
    owner: RuntimeWorldOwnerIdentity,
    basis: CompositeBasisIdentity,
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl PublicationRetentionObligation {
    pub(crate) fn owner_issued(pair: IssuedComponentPinPair) -> Self {
        let (owner, basis, dependency, relational, signal) = pair.into_parts();
        assert_eq!(
            dependency,
            ComponentBasisDependencyClass::ActivePublicationAttempt
        );
        Self {
            owner,
            basis,
            relational,
            signal,
        }
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
                    ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
            && self.signal.key()
                == &ExactComponentPinRequest::signal(
                    basis,
                    ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
            && self.relational.dependency()
                == ComponentBasisDependencyClass::ActivePublicationAttempt
            && self.signal.dependency() == ComponentBasisDependencyClass::ActivePublicationAttempt
    }

    /// Transfer publication custody into the sole product-head authority.
    /// The exact successor basis is checked before either claim changes class.
    pub(crate) fn into_product_head_transfer(
        self,
        successor_basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<ProductHeadRetentionTransfer, (Self, RetentionTransferDenial)> {
        if !self.matches_basis(successor_basis) {
            return Err((self, RetentionTransferDenial::BasisMismatch));
        }
        let Self {
            owner,
            basis,
            relational,
            signal,
        } = self;
        let relational_claim = relational.into_claim();
        let signal_claim = signal.into_claim();
        let control = Arc::clone(&relational_claim.control);
        match control.transfer_pair(
            relational_claim,
            signal_claim,
            ComponentBasisDependencyClass::ProductBranchHead,
        ) {
            Ok((relational, signal)) => {
                let receipt = RetentionTransferReceipt::product_head(
                    owner,
                    basis.clone(),
                    relational.key().clone(),
                    signal.key().clone(),
                );
                let obligation =
                    ProductHeadRetentionObligation::transferred(owner, basis, relational, signal);
                Ok(ProductHeadRetentionTransfer::new(obligation, receipt))
            }
            Err((relational, signal, denial)) => Err((
                Self {
                    owner,
                    basis,
                    relational: ComponentBasisPinObligation::new(relational),
                    signal: ComponentBasisPinObligation::new(signal),
                },
                denial,
            )),
        }
    }
}
