use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, Weak};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::budget::RuntimeWorldBudgetLimit;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::component_obligation::{
    ComponentBasisPinObligation, ComponentPinBinding, RetentionOwnerState,
};
pub(crate) use super::component_obligation::{
    ObservationRetentionObligation, PublicationRetentionObligation,
    RetainedPartialRetentionObligation,
};
use super::unique_component_pin::{ExactComponentBasisKey, ExactComponentPinRequest};
use super::{ComponentBasisDependencyClass, ComponentBasisObligationTransferDestination};

/// The Phase 1 retention owner issues bounded, opaque obligations for exactly
/// one Runtime World owner. The full unique-pin map and owner-lease
/// single-flight protocol remain a Phase 2 implementation behind this seam.
#[derive(Debug, Clone)]
pub(crate) struct RuntimeWorldRetentionOwner {
    state: Arc<RetentionOwnerState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetentionObligationDenial {
    CapacityExhausted,
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetentionTransferDenial {
    BasisMismatch,
    ReleaseDestination,
}

/// Receipt carried when an owner-issued pair changes Runtime World semantic
/// custody. It owns the transferred RAII obligation and records independent
/// exact keys and destination; it is not a second lease authority.
#[derive(Debug)]
pub(crate) struct RetentionTransferReceipt {
    relational: ExactComponentBasisKey,
    signal: ExactComponentBasisKey,
    destination: ComponentBasisObligationTransferDestination,
    obligation: PublicationRetentionObligation,
}

impl RuntimeWorldRetentionOwner {
    pub(crate) fn new(
        owner_identity: RuntimeWorldOwnerIdentity,
        unique_pin_limit: RuntimeWorldBudgetLimit,
        obligation_reservation_limit: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            state: Arc::new(RetentionOwnerState::new(
                owner_identity,
                unique_pin_limit.get(),
                obligation_reservation_limit.get(),
            )),
        }
    }

    pub(crate) fn issue_observation(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<ObservationRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(basis, ComponentBasisDependencyClass::AdmittedObservation)
            .map(|(relational, signal)| ObservationRetentionObligation::new(relational, signal))
    }

    pub(crate) fn issue_publication(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<PublicationRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(
            basis,
            ComponentBasisDependencyClass::ActivePublicationAttempt,
        )
        .map(|(relational, signal)| PublicationRetentionObligation::new(relational, signal))
    }

    pub(crate) fn issue_retained_partial(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
    ) -> Result<RetainedPartialRetentionObligation, RetentionObligationDenial> {
        self.issue_pair(
            basis,
            ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects,
        )
        .map(|(relational, signal)| RetainedPartialRetentionObligation::new(relational, signal))
    }

    #[cfg(test)]
    pub(crate) fn active_component_obligation_count(&self) -> usize {
        self.state
            .active_component_obligations
            .load(Ordering::Acquire)
    }

    fn issue_pair(
        &self,
        basis: &AdmittedCompositeRuntimeWorldBasis,
        dependency: ComponentBasisDependencyClass,
    ) -> Result<(ComponentBasisPinObligation, ComponentBasisPinObligation), RetentionObligationDenial>
    {
        if basis.owner_identity() != self.state.owner_identity {
            return Err(RetentionObligationDenial::ForeignOwner {
                expected: self.state.owner_identity,
                actual: basis.owner_identity(),
            });
        }
        let relational =
            self.issue_component(ExactComponentPinRequest::relational(basis, dependency))?;
        let signal = match self.issue_component(ExactComponentPinRequest::signal(basis, dependency))
        {
            Ok(signal) => signal,
            Err(denial) => {
                drop(relational);
                return Err(denial);
            }
        };
        Ok((relational, signal))
    }

    fn issue_component(
        &self,
        request: ExactComponentPinRequest<'_>,
    ) -> Result<ComponentBasisPinObligation, RetentionObligationDenial> {
        let key = request.key();
        let dependency = request.dependency();
        self.reserve_obligation_slot()?;
        let mut pins = self
            .state
            .pins
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let binding = match pins.get(&key).and_then(Weak::upgrade) {
            Some(binding) => binding,
            None => {
                if pins.get(&key).is_some() {
                    pins.remove(&key);
                }
                if pins.len() >= self.state.maximum_unique_component_pins {
                    self.release_obligation_slot();
                    return Err(RetentionObligationDenial::CapacityExhausted);
                }
                let binding = Arc::new(ComponentPinBinding {
                    key: key.clone(),
                    counts: Mutex::new(super::ComponentBasisDependencyCounts::zero()),
                    owner: Arc::downgrade(&self.state),
                });
                pins.insert(key.clone(), Arc::downgrade(&binding));
                binding
            }
        };
        let mut counts = binding
            .counts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if counts.increment(dependency).is_none() {
            drop(counts);
            self.release_obligation_slot();
            return Err(RetentionObligationDenial::CapacityExhausted);
        }
        drop(counts);
        Ok(ComponentBasisPinObligation::new(
            key,
            dependency,
            binding,
            &self.state,
        ))
    }

    fn reserve_obligation_slot(&self) -> Result<(), RetentionObligationDenial> {
        let mut current = self
            .state
            .active_component_obligations
            .load(Ordering::Acquire);
        loop {
            let next = current
                .checked_add(1)
                .ok_or(RetentionObligationDenial::CapacityExhausted)?;
            if next > self.state.maximum_component_obligation_reservations {
                return Err(RetentionObligationDenial::CapacityExhausted);
            }
            match self
                .state
                .active_component_obligations
                .compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return Ok(()),
                Err(observed) => current = observed,
            }
        }
    }

    fn release_obligation_slot(&self) {
        self.state
            .active_component_obligations
            .fetch_sub(1, Ordering::AcqRel);
    }
}

impl RetentionTransferReceipt {
    pub(crate) fn from_publication(
        obligation: PublicationRetentionObligation,
        basis: &AdmittedCompositeRuntimeWorldBasis,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Result<Self, RetentionTransferDenial> {
        if !obligation.matches_basis(basis) {
            return Err(RetentionTransferDenial::BasisMismatch);
        }
        if destination.dependency_class().is_none() {
            return Err(RetentionTransferDenial::ReleaseDestination);
        }
        let obligation = obligation.transfer_to(destination);
        Ok(Self {
            relational: obligation.relational().key().clone(),
            signal: obligation.signal().key().clone(),
            destination,
            obligation,
        })
    }

    pub(crate) const fn destination(&self) -> ComponentBasisObligationTransferDestination {
        self.destination
    }

    pub(crate) fn obligation(&self) -> &PublicationRetentionObligation {
        &self.obligation
    }

    pub(crate) fn matches_basis(&self, basis: &AdmittedCompositeRuntimeWorldBasis) -> bool {
        self.obligation.relational().owner_identity() == basis.owner_identity()
            && self.obligation.signal().owner_identity() == basis.owner_identity()
            && self.relational
                == ExactComponentPinRequest::relational(
                    basis,
                    ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
            && self.signal
                == ExactComponentPinRequest::signal(
                    basis,
                    ComponentBasisDependencyClass::ActivePublicationAttempt,
                )
                .key()
    }
}
