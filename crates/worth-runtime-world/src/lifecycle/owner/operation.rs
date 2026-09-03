use crate::branch::ProductBranchObservation;
use crate::lifecycle::{
    RuntimeWorldCancellationBoundary, RuntimeWorldCancellationToken, RuntimeWorldInstant,
};
use crate::publication::{
    lower_component_plans, LoweredOwnerComponentPlan, NoEffectCause, NoEffectCompositePublication,
    ReservedCompositePublicationAttempt, ResolvedExpectedProductHead,
};

use super::RuntimeWorldOwnerRoot;

mod attempt;
mod publication_capacity;
mod reservation_steps;

#[cfg(test)]
#[path = "operation/preparation_tests.rs"]
mod preparation_tests;

#[cfg(test)]
#[path = "operation/preparation_test_support.rs"]
mod preparation_test_support;

pub(crate) use attempt::{
    RuntimeWorldOperationLedger, RuntimeWorldOperationReservation, RuntimeWorldOperationState,
};
pub(crate) use publication_capacity::{
    ReservedPublicationAttemptCapacity, RuntimeWorldPublicationCapacityLedger,
};
use reservation_steps::{
    issue_publication_identities, reserve_publication_resources, IssuedPublicationIdentities,
};

struct ReservationContext<'a> {
    plan: LoweredOwnerComponentPlan,
    expected_head: ProductBranchObservation,
    cancellation: &'a RuntimeWorldCancellationToken,
    deadline: Option<RuntimeWorldInstant>,
    operation: RuntimeWorldOperationReservation,
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
        let expected_head = plan.expected().expected().clone();
        self.validate_reservation_preconditions(&plan, &expected_head, cancellation, deadline)
            .map_err(|cause| {
                NoEffectCompositePublication::new(cause, Some(expected_head.clone()))
            })?;
        let operation = match self.reserve_operation_if_open_and_bootstrapped() {
            Ok(operation) => operation,
            Err(()) => {
                return Err(NoEffectCompositePublication::new(
                    NoEffectCause::OwnerUnavailable,
                    Some(expected_head),
                ))
            }
        };
        self.complete_reservation(ReservationContext {
            plan,
            expected_head,
            cancellation,
            deadline,
            operation,
        })
    }

    fn complete_reservation(
        &self,
        context: ReservationContext<'_>,
    ) -> Result<ReservedCompositePublicationAttempt, NoEffectCompositePublication> {
        let ReservationContext {
            plan,
            expected_head,
            cancellation,
            deadline,
            operation,
        } = context;
        if let Some(denied) = self.reservation_denial(&expected_head, cancellation, deadline) {
            return Err(denied);
        }
        let IssuedPublicationIdentities {
            attempt_identity,
            commit_identity,
            product_unpublished_identity,
        } = issue_publication_identities(self).map_err(|cause| {
            NoEffectCompositePublication::new(cause, Some(expected_head.clone()))
        })?;
        if let Some(denied) = self.reservation_denial(&expected_head, cancellation, deadline) {
            return Err(denied);
        }
        let resources = reserve_publication_resources(self, &expected_head, &commit_identity)
            .map_err(|cause| {
                NoEffectCompositePublication::new(cause, Some(expected_head.clone()))
            })?;
        if let Some(denied) = self.reservation_denial(&expected_head, cancellation, deadline) {
            return Err(denied);
        }
        if !self.current_product_head_is(&expected_head) {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::StaleExpectedProductHead,
                Some(expected_head.clone()),
            ));
        }
        let reservation_steps::ReservedPublicationResources {
            history,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
        } = resources;
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

    fn validate_reservation_preconditions(
        &self,
        plan: &LoweredOwnerComponentPlan,
        expected: &crate::branch::ProductBranchObservation,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<(), NoEffectCause> {
        if let Some(cause) = self.pre_effect_denial(cancellation, deadline) {
            return Err(cause);
        }
        if expected.owner_identity() != self.owner_identity() {
            return Err(NoEffectCause::OwnerDeniedBeforeEffect);
        }
        if !plan.is_compatible_with(expected) {
            return Err(NoEffectCause::PreEffectFailure);
        }
        if !self.current_product_head_is(expected) {
            return Err(NoEffectCause::StaleExpectedProductHead);
        }
        if expected.reference_generation().advance().is_err() {
            return Err(NoEffectCause::ReferenceGenerationExhausted);
        }
        Ok(())
    }

    fn reservation_denial(
        &self,
        expected: &crate::branch::ProductBranchObservation,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Option<NoEffectCompositePublication> {
        self.pre_effect_denial(cancellation, deadline)
            .map(|cause| NoEffectCompositePublication::new(cause, Some(expected.clone())))
    }

    fn current_product_head_is(&self, expected: &crate::branch::ProductBranchObservation) -> bool {
        self.state
            .branches
            .root_snapshot()
            .is_some_and(|current| expected.mismatch_against_snapshot(&current).is_none())
    }

    fn pre_effect_denial(
        &self,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Option<NoEffectCause> {
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeReservation)
            .is_err()
        {
            return Some(NoEffectCause::CancelledBeforeEffect);
        }
        deadline
            .filter(|deadline| self.state.clock.now() >= *deadline)
            .map(|_| NoEffectCause::DeadlineBeforeEffect)
    }

    fn reserve_operation_if_open_and_bootstrapped(
        &self,
    ) -> Result<RuntimeWorldOperationReservation, ()> {
        let bootstrap = self
            .state
            .bootstrap
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *bootstrap != super::RuntimeWorldBootstrapState::Performed {
            return Err(());
        }
        let mut ledger = self
            .state
            .operation
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let close = self
            .state
            .close
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if close.state() != super::super::close::RuntimeWorldCloseState::Open {
            return Err(());
        }
        ledger.active = ledger.active.checked_add(1).ok_or(())?;
        drop(close);
        drop(ledger);
        drop(bootstrap);
        Ok(self.state.operation.preparing_reservation())
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
        let Some(current) = self.state.branches.root_snapshot() else {
            return Err(NoEffectCompositePublication::new(
                NoEffectCause::OwnerUnavailable,
                Some(expected),
            ));
        };
        let component_intent = intent.component_intent();
        let resolved =
            match ResolvedExpectedProductHead::from_current(intent, expected.clone(), &current) {
                Ok(resolved) => resolved,
                Err(_) => {
                    return Err(NoEffectCompositePublication::new(
                        NoEffectCause::StaleExpectedProductHead,
                        Some(expected),
                    ))
                }
            };
        lower_component_plans(resolved, component_intent)
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
