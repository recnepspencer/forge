use std::sync::{Arc, Mutex};

use crate::budget::RuntimeWorldBudgetLimit;
use crate::history::{CompositeCommitParent, OrdinaryParent};
use crate::lifecycle::{
    RuntimeWorldCancellationBoundary, RuntimeWorldCancellationToken, RuntimeWorldInstant,
};
use crate::publication::{
    lower_component_plans, LoweredOwnerComponentPlan, NoEffectCause, NoEffectCompositePublication,
    ReservedCompositePublicationAttempt, ResolvedExpectedProductHead,
};
use crate::recovery::RecoveryCatalogDenial;

use super::RuntimeWorldOwnerRoot;

/// Explicit owner operation posture. It keeps lifecycle/operation state
/// separate from identity, component ports, and the product registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeWorldOperationState {
    Idle,
    Preparing,
    Executing,
    Publishing,
    Recovering,
}

/// Serial operation custody held by every reserved publication attempt. It
/// keeps close from racing a live attempt and restores Idle on all unwind and
/// terminal paths.
#[must_use = "a live owner operation must remain held by its attempt"]
pub(crate) struct RuntimeWorldOperationReservation {
    state: Arc<Mutex<RuntimeWorldOperationState>>,
    armed: bool,
}

impl std::fmt::Debug for RuntimeWorldOperationReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeWorldOperationReservation")
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for RuntimeWorldOperationReservation {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        assert_ne!(
            *state,
            RuntimeWorldOperationState::Idle,
            "a live operation reservation cannot be dropped while Idle"
        );
        *state = RuntimeWorldOperationState::Idle;
        self.armed = false;
    }
}

#[derive(Debug)]
struct PublicationAttemptCapacityState {
    maximum: usize,
    active: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct RuntimeWorldPublicationCapacityLedger {
    state: Arc<Mutex<PublicationAttemptCapacityState>>,
}

#[must_use = "a publication attempt capacity reservation must be retained or dropped"]
pub(crate) struct ReservedPublicationAttemptCapacity {
    ledger: RuntimeWorldPublicationCapacityLedger,
    armed: bool,
}

impl std::fmt::Debug for ReservedPublicationAttemptCapacity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReservedPublicationAttemptCapacity")
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for ReservedPublicationAttemptCapacity {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let mut state = self
            .ledger
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        state.active = state
            .active
            .checked_sub(1)
            .expect("a live publication reservation owns one active slot");
        self.armed = false;
    }
}

impl RuntimeWorldPublicationCapacityLedger {
    pub(super) fn new(limit: RuntimeWorldBudgetLimit) -> Self {
        Self {
            state: Arc::new(Mutex::new(PublicationAttemptCapacityState {
                maximum: limit.get(),
                active: 0,
            })),
        }
    }

    pub(crate) fn reserve(&self) -> Result<ReservedPublicationAttemptCapacity, ()> {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.active >= state.maximum {
            return Err(());
        }
        state.active += 1;
        Ok(ReservedPublicationAttemptCapacity {
            ledger: self.clone(),
            armed: true,
        })
    }

    pub(crate) fn active(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .active
    }
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// The owner-only preparation transition. Identities and all bounded
    /// reservations are issued here; a lowered plan cannot reserve itself.
    pub(crate) fn reserve(
        &self,
        plan: LoweredOwnerComponentPlan,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedCompositePublicationAttempt, NoEffectCompositePublication> {
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeReservation)
            .is_err()
        {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::CancelledBeforeEffect,
                None,
            ));
        }
        let expected_head = plan.expected().expected().clone();
        if expected_head.owner_identity() != self.owner_identity() {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected_head),
            ));
        }
        if !self.is_open_and_bootstrapped() {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerUnavailable,
                Some(expected_head),
            ));
        }
        let operation = match self.reserve_operation() {
            Ok(operation) => operation,
            Err(()) => {
                return Err(NoEffectCompositePublication::new(
                    NoEffectCause::OwnerUnavailable,
                    Some(expected_head),
                ))
            }
        };

        let (attempt_identity, commit_identity, product_unpublished_identity) = {
            let mut identities = self
                .state
                .identities
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            let attempt_identity = match identities.publication_attempt() {
                Ok(identity) => identity,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::OwnerDeniedBeforeEffect,
                        Some(expected_head),
                    ))
                }
            };
            let commit_identity = match identities.composite_commit() {
                Ok(identity) => identity,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::OwnerDeniedBeforeEffect,
                        Some(expected_head),
                    ))
                }
            };
            let product_unpublished_identity = match identities.product_unpublished() {
                Ok(identity) => identity,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::OwnerDeniedBeforeEffect,
                        Some(expected_head),
                    ))
                }
            };
            (
                attempt_identity,
                commit_identity,
                product_unpublished_identity,
            )
        };

        let history = self.state.history.clone();
        let parent = CompositeCommitParent::Ordinary(OrdinaryParent::new(
            expected_head.selected_commit().clone(),
        ));
        let reserved_commit_capacity =
            match history.reserve_commit_capacity(commit_identity.clone(), parent) {
                Ok(capacity) => capacity,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::CapacityExhausted,
                        Some(expected_head),
                    ))
                }
            };
        let reserved_recovery_slot = match self
            .state
            .recovery
            .reserve_product_unpublished(self.owner_identity())
        {
            Ok(slot) => slot,
            Err(RecoveryCatalogDenial::CapacityExhausted { .. }) => {
                return Err(NoEffectCompositePublication::new(
                    NoEffectCause::CapacityExhausted,
                    Some(expected_head),
                ))
            }
            Err(RecoveryCatalogDenial::ForeignOwner { .. }) => {
                return Err(NoEffectCompositePublication::new(
                    NoEffectCause::OwnerUnavailable,
                    Some(expected_head),
                ))
            }
        };
        let reserved_component_pin_pair =
            match self.state.retention.reserve_product_publication_pair() {
                Ok(capacity) => capacity,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::CapacityExhausted,
                        Some(expected_head),
                    ))
                }
            };
        let reserved_publication_capacity = match self.state.publication_capacity.reserve() {
            Ok(capacity) => capacity,
            Err(()) => {
                return Err(NoEffectCompositePublication::new(
                    NoEffectCause::CapacityExhausted,
                    Some(expected_head),
                ))
            }
        };
        Ok(ReservedCompositePublicationAttempt::new(
            attempt_identity,
            expected_head.clone(),
            expected_head.basis().clone(),
            plan,
            commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
            history,
            deadline,
            operation,
        ))
    }

    fn reserve_operation(&self) -> Result<RuntimeWorldOperationReservation, ()> {
        let state = Arc::clone(&self.state.operation);
        let mut operation = state.lock().unwrap_or_else(|error| error.into_inner());
        if *operation != RuntimeWorldOperationState::Idle {
            return Err(());
        }
        *operation = RuntimeWorldOperationState::Preparing;
        drop(operation);
        Ok(RuntimeWorldOperationReservation { state, armed: true })
    }

    fn is_open_and_bootstrapped(&self) -> bool {
        let bootstrap = self
            .state
            .bootstrap
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *bootstrap != super::RuntimeWorldBootstrapState::Performed {
            return false;
        }
        self.state
            .close
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .state()
            == super::super::close::RuntimeWorldCloseState::Open
    }
}

impl<D, I, E, Ctx, T> super::super::ports::RuntimeWorldPreparationService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn prepare(
        &self,
        expected: crate::branch::ProductBranchObservation,
        intent: crate::publication::ProductBranchIntent,
    ) -> Result<LoweredOwnerComponentPlan, NoEffectCompositePublication> {
        if expected.owner_identity() != self.owner_identity() {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected),
            ));
        }
        if !self.is_open_and_bootstrapped() {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerUnavailable,
                Some(expected),
            ));
        }
        let component_intent = intent.component_intent();
        let resolved = ResolvedExpectedProductHead::new(intent, expected);
        Ok(lower_component_plans(resolved, component_intent))
    }

    fn reserve(
        &self,
        plan: LoweredOwnerComponentPlan,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedCompositePublicationAttempt, NoEffectCompositePublication> {
        RuntimeWorldOwnerRoot::reserve(self, plan, cancellation, deadline)
    }
}
