use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::RuntimeWorldOwnerIdentity;

use super::unique_component_pin::ExactComponentBasisKey;
use super::unique_component_pin::ExactComponentPinRequest;
use super::{ComponentBasisDependencyClass, ComponentBasisObligationTransferDestination};

/// Owner-local state for bounded, weak exact-component canonicalization. The
/// map never keeps an obligation or an external component lease alive.
#[derive(Debug)]
pub(super) struct RetentionOwnerState {
    pub(super) owner_identity: RuntimeWorldOwnerIdentity,
    pub(super) maximum_unique_component_pins: usize,
    pub(super) maximum_component_obligation_reservations: usize,
    pub(super) active_component_obligations: AtomicUsize,
    pub(super) pins: Mutex<HashMap<ExactComponentBasisKey, Weak<ComponentPinBinding>>>,
}

impl RetentionOwnerState {
    pub(super) fn new(
        owner_identity: RuntimeWorldOwnerIdentity,
        maximum_unique_component_pins: usize,
        maximum_component_obligation_reservations: usize,
    ) -> Self {
        Self {
            owner_identity,
            maximum_unique_component_pins,
            maximum_component_obligation_reservations,
            active_component_obligations: AtomicUsize::new(0),
            pins: Mutex::new(HashMap::new()),
        }
    }
}

/// Two independent exact component pins carried by one observation. The
/// relational and Signal keys are deliberately separate even though the
/// obligation is transferred as one observation-level value.
#[derive(Debug)]
pub(crate) struct ObservationRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

/// Two independent exact component pins reserved for one active publication's
/// prospective successor basis. The expected predecessor remains pinned by
/// the owner-issued observation carried by that attempt.
#[derive(Debug)]
pub(crate) struct PublicationRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

/// Two independent exact component pins retained with owner effects after a
/// product-reference publication did not occur.
#[derive(Debug)]
pub(crate) struct RetainedPartialRetentionObligation {
    relational: ComponentBasisPinObligation,
    signal: ComponentBasisPinObligation,
}

/// One non-cloneable, owner-issued exact component binding. Its Drop path
/// releases only this component key and only this dependency registration.
#[derive(Debug)]
pub(crate) struct ComponentBasisPinObligation {
    owner_identity: RuntimeWorldOwnerIdentity,
    key: ExactComponentBasisKey,
    dependency: ComponentBasisDependencyClass,
    release: ComponentPinRelease,
}

#[derive(Debug)]
struct ComponentPinRelease {
    state: Weak<RetentionOwnerState>,
    binding: Arc<ComponentPinBinding>,
    dependency: ComponentBasisDependencyClass,
    released: AtomicBool,
}

/// One exact component key has one binding. Obligations share that binding and
/// update its independent dependency counts. The owner map is weak, so it
/// cannot keep a component owner lease or an obligation alive.
#[derive(Debug)]
pub(super) struct ComponentPinBinding {
    pub(super) key: ExactComponentBasisKey,
    pub(super) counts: Mutex<super::ComponentBasisDependencyCounts>,
    pub(super) owner: Weak<RetentionOwnerState>,
}

impl ComponentBasisPinObligation {
    pub(super) fn new(
        key: ExactComponentBasisKey,
        dependency: ComponentBasisDependencyClass,
        binding: Arc<ComponentPinBinding>,
        state: &Arc<RetentionOwnerState>,
    ) -> Self {
        Self {
            owner_identity: state.owner_identity,
            key,
            dependency,
            release: ComponentPinRelease {
                state: Arc::downgrade(state),
                binding,
                dependency,
                released: AtomicBool::new(false),
            },
        }
    }

    pub(crate) fn key(&self) -> &ExactComponentBasisKey {
        &self.key
    }

    pub(crate) const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner_identity
    }

    pub(crate) const fn dependency(&self) -> ComponentBasisDependencyClass {
        self.dependency
    }

    #[cfg(test)]
    pub(crate) fn dependency_count(&self, dependency: ComponentBasisDependencyClass) -> usize {
        self.release
            .binding
            .counts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(dependency)
    }

    #[cfg(test)]
    pub(crate) fn binding_identity(&self) -> usize {
        Arc::as_ptr(&self.release.binding) as usize
    }

    pub(crate) fn transfer_to(
        mut self,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Self {
        let next = destination
            .dependency_class()
            .expect("a retained component obligation cannot transfer to Release");
        if self.dependency == next {
            return self;
        }
        let mut counts = self
            .release
            .binding
            .counts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        counts
            .decrement(self.dependency)
            .expect("an owned component obligation has one dependency count");
        counts
            .increment(next)
            .expect("transferring an owned dependency cannot overflow its count");
        drop(counts);
        self.dependency = next;
        self.release.dependency = next;
        self
    }
}

impl Drop for ComponentPinRelease {
    fn drop(&mut self) {
        if self.released.swap(true, Ordering::AcqRel) {
            return;
        }
        if let Some(state) = self.state.upgrade() {
            let mut counts = self
                .binding
                .counts
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            counts
                .decrement(self.dependency)
                .expect("a live component obligation owns its dependency count");
            drop(counts);
            state
                .active_component_obligations
                .fetch_sub(1, Ordering::AcqRel);
        }
    }
}

impl Drop for ComponentPinBinding {
    fn drop(&mut self) {
        let Some(owner) = self.owner.upgrade() else {
            return;
        };
        let mut pins = owner
            .pins
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if pins
            .get(&self.key)
            .is_some_and(|entry| entry.upgrade().is_none())
        {
            pins.remove(&self.key);
        }
    }
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

    /// The publication reservation is for the exact basis that the owner
    /// intends to install as the successor. The expected predecessor remains
    /// pinned by the attempt's owner-issued observation. This check must run
    /// before the ready token can be issued, so a later transfer cannot swap
    /// in an obligation for an equal-looking basis.
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

    pub(crate) fn transfer_to(
        self,
        destination: ComponentBasisObligationTransferDestination,
    ) -> Self {
        Self {
            relational: self.relational.transfer_to(destination),
            signal: self.signal.transfer_to(destination),
        }
    }
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
