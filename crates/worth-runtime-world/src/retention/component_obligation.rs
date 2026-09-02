use std::fmt;
use std::sync::Arc;

use worth_relational::facade::branch::RelationalBranchBasisDenial;
use worth_signal::facade::branch::SignalBranchRetentionReleaseDenial;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::obligation_transfer::{
    ComponentBasisObligationTransferDestination, RetentionTransferDenial,
};
use super::unique_component_pin::{
    ComponentBasisLeaseIdentity, ComponentBasisPinClaim, ExactComponentBasisKey,
    ExactComponentPinRequest,
};
use super::ComponentBasisDependencyClass;

/// Typed refusal from the Runtime World retention owner when a live claim
/// cannot be terminated. The component owner lease remains bound to the
/// registry in every refusal path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RetentionReleaseDenial {
    UnknownPin,
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    Relational(RelationalBranchBasisDenial),
    Signal(SignalBranchRetentionReleaseDenial),
}

/// Why an explicit claim release reached its terminal outcome or refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentBasisReleaseOutcome {
    OwnerReleased,
    OwnerUnavailable,
    OwnerOperationPanicked,
    SharedOwnerLease,
}

/// Exact evidence for one dependency-count release. It carries no capability
/// and is not used to authorize another release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ComponentBasisReleaseReceipt {
    key: ExactComponentBasisKey,
    lease_identity: ComponentBasisLeaseIdentity,
    outcome: ComponentBasisReleaseOutcome,
}

impl ComponentBasisReleaseReceipt {
    pub(crate) fn owner_issued(
        key: ExactComponentBasisKey,
        lease_identity: ComponentBasisLeaseIdentity,
        outcome: ComponentBasisReleaseOutcome,
    ) -> Self {
        Self {
            key,
            lease_identity,
            outcome,
        }
    }

    pub(crate) const fn outcome(&self) -> ComponentBasisReleaseOutcome {
        self.outcome
    }

    pub(crate) fn key(&self) -> &ExactComponentBasisKey {
        &self.key
    }

    pub(crate) const fn lease_identity(&self) -> ComponentBasisLeaseIdentity {
        self.lease_identity
    }
}

/// Error carrier that returns the still-live move-only claim to an explicit
/// caller. A denied foreign release therefore cannot destroy retention.
#[derive(Debug)]
pub(crate) struct RetentionReleaseFailure {
    pub(super) claim: ComponentBasisPinClaim,
    pub(super) denial: RetentionReleaseDenial,
}

/// Internal authority surface implemented only by the concrete retention
/// owner. Claims carry this narrow object so the shared Phase 1 handoffs need
/// not become generic over component runtime types.
pub(super) trait RetentionControlSurface: Send + Sync {
    fn transfer_claim(
        &self,
        claim: ComponentBasisPinClaim,
        target: ComponentBasisDependencyClass,
    ) -> Result<ComponentBasisPinClaim, (ComponentBasisPinClaim, RetentionTransferDenial)>;

    fn transfer_pair(
        &self,
        relational: ComponentBasisPinClaim,
        signal: ComponentBasisPinClaim,
        target: ComponentBasisDependencyClass,
    ) -> Result<
        (ComponentBasisPinClaim, ComponentBasisPinClaim),
        (
            ComponentBasisPinClaim,
            ComponentBasisPinClaim,
            RetentionTransferDenial,
        ),
    >;

    fn release_claim(
        &self,
        claim: ComponentBasisPinClaim,
    ) -> Result<ComponentBasisReleaseReceipt, RetentionReleaseFailure>;

    fn abandon_claim(&self, claim: ComponentBasisPinClaim);
}

/// One non-cloneable, owner-issued exact component dependency claim.
pub(crate) struct ComponentBasisPinObligation {
    claim: Option<ComponentBasisPinClaim>,
}

impl fmt::Debug for ComponentBasisPinObligation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ComponentBasisPinObligation")
            .field("claim", &self.claim)
            .finish()
    }
}

impl ComponentBasisPinObligation {
    pub(super) fn new(claim: ComponentBasisPinClaim) -> Self {
        Self { claim: Some(claim) }
    }

    pub(crate) fn key(&self) -> &ExactComponentBasisKey {
        &self
            .claim
            .as_ref()
            .expect("a live component obligation carries its claim")
            .key
    }

    pub(crate) const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        match &self.claim {
            Some(claim) => claim.owner,
            None => panic!("a live component obligation carries its claim"),
        }
    }

    pub(crate) const fn dependency(&self) -> ComponentBasisDependencyClass {
        match &self.claim {
            Some(claim) => claim.dependency,
            None => panic!("a live component obligation carries its claim"),
        }
    }

    pub(crate) const fn lease_identity(&self) -> ComponentBasisLeaseIdentity {
        match &self.claim {
            Some(claim) => claim.lease_identity,
            None => panic!("a live component obligation carries its claim"),
        }
    }

    pub(super) fn into_claim(mut self) -> ComponentBasisPinClaim {
        self.claim
            .take()
            .expect("a live component obligation carries its claim")
    }

    /// Transfer this one exact dependency without acquiring another owner
    /// lease. Failure returns the original obligation unchanged.
    pub(crate) fn try_transfer_to(
        mut self,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Result<Self, (Self, RetentionTransferDenial)> {
        let Some(target) = destination.dependency_class() else {
            return Err((self, RetentionTransferDenial::ReleaseDestination));
        };
        let claim = self
            .claim
            .take()
            .expect("a live component obligation carries its claim");
        let control = Arc::clone(&claim.control);
        match control.transfer_claim(claim, target) {
            Ok(claim) => Ok(Self::new(claim)),
            Err((claim, denial)) => {
                self.claim = Some(claim);
                Err((self, denial))
            }
        }
    }

    /// Explicitly consume one claim. A denied release returns the original
    /// obligation so its caller can rebind it to the correct owner and retry.
    pub(crate) fn try_release(
        mut self,
    ) -> Result<ComponentBasisReleaseReceipt, (Self, RetentionReleaseDenial)> {
        let claim = self
            .claim
            .take()
            .expect("a live component obligation reaches release only once");
        let control = Arc::clone(&claim.control);
        match control.release_claim(claim) {
            Ok(receipt) => Ok(receipt),
            Err(failure) => {
                self.claim = Some(failure.claim);
                Err((self, failure.denial))
            }
        }
    }
}

impl Drop for ComponentBasisPinObligation {
    fn drop(&mut self) {
        let Some(claim) = self.claim.take() else {
            return;
        };
        let control = Arc::clone(&claim.control);
        control.abandon_claim(claim);
    }
}

/// Two independent exact component claims carried by one observation.
#[derive(Debug)]
pub(crate) struct ObservationRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl ObservationRetentionObligation {
    pub(super) fn new(
        relational: ComponentBasisPinObligation,
        signal: ComponentBasisPinObligation,
    ) -> Self {
        Self { relational, signal }
    }

    pub(crate) fn relational(&self) -> &ComponentBasisPinObligation {
        &self.relational
    }

    pub(crate) fn signal(&self) -> &ComponentBasisPinObligation {
        &self.signal
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.relational.owner_identity() == basis.owner_identity()
            && self.signal.owner_identity() == basis.owner_identity()
            && self.relational.key()
                == &ExactComponentPinRequest::relational(
                    basis,
                    ComponentBasisDependencyClass::AdmittedObservation,
                )
                .key()
            && self.signal.key()
                == &ExactComponentPinRequest::signal(
                    basis,
                    ComponentBasisDependencyClass::AdmittedObservation,
                )
                .key()
    }
}

/// Two independent exact component claims reserved for one publication
/// attempt. The prospective successor basis is checked before transfer.
#[derive(Debug)]
pub(crate) struct PublicationRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl PublicationRetentionObligation {
    pub(super) fn new(
        relational: ComponentBasisPinObligation,
        signal: ComponentBasisPinObligation,
    ) -> Self {
        Self { relational, signal }
    }

    pub(crate) fn relational(&self) -> &ComponentBasisPinObligation {
        &self.relational
    }

    pub(crate) fn signal(&self) -> &ComponentBasisPinObligation {
        &self.signal
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.relational.owner_identity() == basis.owner_identity()
            && self.signal.owner_identity() == basis.owner_identity()
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
    }

    pub(crate) fn try_transfer_to(
        self,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Result<Self, (Self, RetentionTransferDenial)> {
        let Some(target) = destination.dependency_class() else {
            return Err((self, RetentionTransferDenial::ReleaseDestination));
        };
        let Self { relational, signal } = self;
        let relational_claim = relational.into_claim();
        let signal_claim = signal.into_claim();
        let control = Arc::clone(&relational_claim.control);
        match control.transfer_pair(relational_claim, signal_claim, target) {
            Ok((relational, signal)) => Ok(Self::new(
                ComponentBasisPinObligation::new(relational),
                ComponentBasisPinObligation::new(signal),
            )),
            Err((relational, signal, denial)) => Err((
                Self::new(
                    ComponentBasisPinObligation::new(relational),
                    ComponentBasisPinObligation::new(signal),
                ),
                denial,
            )),
        }
    }
}

/// Two independent exact component claims retained with product-unpublished
/// owner effects.
#[derive(Debug)]
pub(crate) struct RetainedPartialRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

impl RetainedPartialRetentionObligation {
    pub(super) fn new(
        relational: ComponentBasisPinObligation,
        signal: ComponentBasisPinObligation,
    ) -> Self {
        Self { relational, signal }
    }

    pub(crate) fn relational(&self) -> &ComponentBasisPinObligation {
        &self.relational
    }

    pub(crate) fn signal(&self) -> &ComponentBasisPinObligation {
        &self.signal
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.relational.owner_identity() == basis.owner_identity()
            && self.signal.owner_identity() == basis.owner_identity()
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
    }
}
