use crate::basis;
use crate::branch::ProductBranchObservation;
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{
    NoEffectCause, ReservedCompositePublicationAttempt, RuntimeWorldCancellationBoundary,
    RuntimeWorldCancellationToken, SignalComponentPlanPosture,
};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// The pre-effect boundary checks, in the one order the attempt must see
    /// them. A denial here has provably moved no owner.
    pub(super) fn admit_owner_execution(
        &self,
        attempt: &ReservedCompositePublicationAttempt,
        cancellation: &RuntimeWorldCancellationToken,
    ) -> Result<(), NoEffectCause> {
        let plan = attempt.plan();
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeFirstOwnerEffect)
            .is_err()
        {
            return Err(NoEffectCause::CancelledBeforeEffect);
        }
        if self.deadline_expired(attempt.deadline()) {
            return Err(NoEffectCause::DeadlineBeforeEffect);
        }
        if !plan.is_compatible_with(plan.expected().expected()) {
            return Err(NoEffectCause::PreEffectFailure);
        }
        if matches!(
            plan.relational().posture(),
            crate::publication::RelationalComponentPlanPosture::RetainExact
        ) && matches!(
            plan.signal().posture(),
            SignalComponentPlanPosture::RetainExact
        ) {
            return Err(NoEffectCause::PreEffectFailure);
        }
        // A product publication is the only thing that moves an owner off the
        // observed head, so a replaced product cell is the specific truth
        // behind a stale attempt. It is named here, before the derived
        // consequence that the component bases are no longer current.
        if !self.current_product_head_is(attempt.expected_head()) {
            return Err(NoEffectCause::StaleExpectedProductHead);
        }
        basis::validate_current(
            &self.state.relational.basis_port(),
            &self.state.signal.basis_port(),
            &self.state.bridge,
            attempt.expected_head().basis().relational_basis(),
            attempt.expected_head().basis().signal_basis(),
            attempt.expected_head().basis().correspondence_basis(),
        )
        .map_err(|_| NoEffectCause::OwnerUnavailable)?;
        // The exact product cell is rechecked last, immediately before the
        // first owner effect, so a concurrent product publication cannot make
        // this attempt appear current after its earlier checks passed.
        if !self.current_product_head_is(attempt.expected_head()) {
            return Err(NoEffectCause::StaleExpectedProductHead);
        }
        Ok(())
    }

    pub(super) fn pre_advance_signal_gate(
        &self,
        expected_head: &ProductBranchObservation,
        deadline: Option<RuntimeWorldInstant>,
        cancellation: &RuntimeWorldCancellationToken,
    ) -> Result<(), (ProductUnpublishedCause, NoEffectCause)> {
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BetweenOwnerEffects)
            .is_err()
        {
            return Err((
                ProductUnpublishedCause::CancellationAfterEffect,
                NoEffectCause::CancelledBeforeEffect,
            ));
        }
        if self.deadline_expired(deadline) {
            return Err((
                ProductUnpublishedCause::DeadlineAfterEffect,
                NoEffectCause::DeadlineBeforeEffect,
            ));
        }
        if !self.current_product_head_is(expected_head) {
            return Err((
                ProductUnpublishedCause::StaleProductHead,
                NoEffectCause::StaleExpectedProductHead,
            ));
        }
        Ok(())
    }

    pub(crate) fn deadline_expired(&self, deadline: Option<RuntimeWorldInstant>) -> bool {
        deadline.is_some_and(|deadline| self.state.clock.now() >= deadline)
    }
}
