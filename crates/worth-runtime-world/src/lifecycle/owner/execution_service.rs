use crate::basis;
use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::lifecycle::cancellation::RuntimeWorldCancellationBoundary;
use crate::lifecycle::{RuntimeWorldCancellationToken, RuntimeWorldInstant};
use crate::publication::{
    CompositeAttemptProgress, CompositeExecutionBorrow, NoEffectCause,
    NoEffectCompositePublication, OwnerExecutionOutcome, RelationalAttemptProgress,
    RelationalAttemptProgressPosture, ReservedCompositePublicationAttempt, SignalAttemptProgress,
    SignalComponentPlanPosture,
};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

#[path = "execution_service/relational.rs"]
mod relational;
#[path = "execution_service/signal.rs"]
mod signal;
#[path = "execution_service/successor.rs"]
mod successor;

#[cfg(test)]
#[path = "execution_service/tests.rs"]
mod tests;

use relational::RelationalExecutionFailure;
use signal::{map_fork_no_effect, SignalExecutionFailure};

impl<D, I, E, Ctx, T> super::super::ports::RuntimeWorldOwnerExecutionService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    type SignalDefinition = D;
    type SignalIdentity = I;
    type SignalEvent = E;
    type SignalContext = Ctx;
    type SignalTransactionKey = T;

    fn execute(
        &self,
        mut attempt: crate::publication::ReservedCompositePublicationAttempt,
        borrow: CompositeExecutionBorrow<'_, D, I, E, Ctx, T>,
        cancellation: &crate::lifecycle::RuntimeWorldCancellationToken,
    ) -> OwnerExecutionOutcome {
        attempt.begin_owner_execution();
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeFirstOwnerEffect)
            .is_err()
        {
            return self.no_effect(attempt, NoEffectCause::CancelledBeforeEffect);
        }

        if let Err(cause) = self.validate_execution_inputs(&attempt, cancellation) {
            return self.no_effect(attempt, cause);
        }

        let signal_expected = attempt.plan().signal().expected().clone();
        let fork_reservation = match self.reserve_signal_destination(&attempt) {
            Ok(reservation) => reservation,
            Err(cause) => return self.no_effect(attempt, cause),
        };

        // The destination reservation is owner custody, not a product-head
        // movement. Recheck the exact product cell immediately before the
        // first component owner effect so a concurrent product publication
        // cannot make this attempt appear current after its reservation.
        if !self.current_product_head_is(attempt.expected_head()) {
            return self.no_effect(attempt, NoEffectCause::StaleExpectedProductHead);
        }
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeFirstOwnerEffect)
            .is_err()
        {
            return self.no_effect(attempt, NoEffectCause::CancelledBeforeEffect);
        }
        if self.deadline_expired(attempt.deadline()) {
            return self.no_effect(attempt, NoEffectCause::DeadlineBeforeEffect);
        }

        let relational = match self.execute_relational(&mut attempt) {
            Ok(progress) => progress,
            Err(RelationalExecutionFailure {
                cause,
                no_effect,
                partial,
            }) => {
                let progress =
                    CompositeAttemptProgress::new(partial, SignalAttemptProgress::untouched());
                return self.retain_or_no_effect(attempt, progress, cause, no_effect);
            }
        };
        let relational_successor = relational.successor_basis().cloned();
        let progress =
            CompositeAttemptProgress::new(relational, SignalAttemptProgress::untouched());

        if progress.relational_requires_settlement() {
            let successor = self.issue_successor_basis(
                relational_successor.expect("pending Relational progress carries its basis"),
                signal_expected,
                attempt.predecessor_basis().correspondence_basis().clone(),
            );
            return OwnerExecutionOutcome::ProductUnpublished(
                attempt.settle(progress).retain_with_cause(
                    successor,
                    ProductUnpublishedCause::SettlementPending,
                    None,
                ),
            );
        }

        if let Err((cause, no_effect)) =
            self.pre_advance_signal_gate(attempt.expected_head(), attempt.deadline(), cancellation)
        {
            if cause == ProductUnpublishedCause::CancellationAfterEffect {
                attempt.observe_cancellation();
            }
            return self.retain_or_no_effect(attempt, progress, cause, no_effect);
        }

        let signal_step = self.execute_signal(
            attempt.plan(),
            attempt.expected_head(),
            attempt.deadline(),
            fork_reservation,
            borrow,
            cancellation,
        );
        let (signal, signal_failure) = match signal_step {
            Ok(signal) => (signal, None),
            Err(SignalExecutionFailure {
                cause,
                no_effect,
                partial,
            }) => (
                partial,
                Some(SignalExecutionFailure {
                    cause,
                    no_effect,
                    partial: SignalAttemptProgress::untouched(),
                }),
            ),
        };
        let progress = CompositeAttemptProgress::new(progress.into_relational(), signal);
        if let Some(failure) = signal_failure {
            return self.retain_or_no_effect(attempt, progress, failure.cause, failure.no_effect);
        }

        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeProductMovement)
            .is_err()
        {
            attempt.observe_cancellation();
            return self.retain_after_effect(
                attempt,
                progress,
                ProductUnpublishedCause::CancellationAfterEffect,
                None,
            );
        }

        if self.deadline_expired(attempt.deadline()) {
            return self.retain_after_effect(
                attempt,
                progress,
                ProductUnpublishedCause::DeadlineAfterEffect,
                None,
            );
        }

        if !self.current_product_head_is(attempt.expected_head()) {
            return self.retain_or_no_effect(
                attempt,
                progress,
                ProductUnpublishedCause::StaleProductHead,
                NoEffectCause::StaleExpectedProductHead,
            );
        }

        let successor =
            self.issue_successor_basis_from_progress(&progress, attempt.predecessor_basis());
        if !self.successor_owners_are_current(&successor) {
            return self.retain_or_no_effect(
                attempt,
                progress,
                ProductUnpublishedCause::OwnerLost,
                NoEffectCause::OwnerUnavailable,
            );
        }
        OwnerExecutionOutcome::Settled(attempt.settle_with_successor_basis(progress, successor))
    }
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn validate_execution_inputs(
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
        if !self.current_product_head_is(attempt.expected_head()) {
            return Err(NoEffectCause::StaleExpectedProductHead);
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
        basis::validate_current(
            &self.state.relational.basis_port(),
            &self.state.signal.basis_port(),
            &self.state.bridge,
            attempt.expected_head().basis().relational_basis(),
            attempt.expected_head().basis().signal_basis(),
            attempt.expected_head().basis().correspondence_basis(),
        )
        .map_err(|_| NoEffectCause::OwnerUnavailable)
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

    fn deadline_expired(&self, deadline: Option<RuntimeWorldInstant>) -> bool {
        deadline.is_some_and(|deadline| self.state.clock.now() >= deadline)
    }

    fn reserve_signal_destination(
        &self,
        attempt: &crate::publication::ReservedCompositePublicationAttempt,
    ) -> Result<
        Option<worth_signal::facade::branch::SignalBranchForkReservation<D, I, T>>,
        NoEffectCause,
    > {
        if !matches!(
            attempt.plan().signal().posture(),
            SignalComponentPlanPosture::ForkExact | SignalComponentPlanPosture::ForkAndAdvance
        ) {
            return Ok(None);
        }
        let name = attempt
            .plan()
            .signal()
            .requested_branch_name()
            .cloned()
            .ok_or(NoEffectCause::PreEffectFailure)?;
        self.state
            .signal
            .mutation_port()
            .reserve_fork_exact(name, attempt.plan().signal().expected())
            .map(Some)
            .map_err(|denial| map_fork_no_effect(&denial))
    }

    fn retain_after_effect(
        &self,
        attempt: crate::publication::ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
        cause: ProductUnpublishedCause,
        observed: Option<ProductBranchReferenceSnapshot>,
    ) -> OwnerExecutionOutcome {
        let successor =
            self.issue_successor_basis_from_progress(&progress, attempt.predecessor_basis());
        OwnerExecutionOutcome::ProductUnpublished(
            attempt
                .settle(progress)
                .retain_with_cause(successor, cause, observed),
        )
    }

    fn retain_or_no_effect(
        &self,
        attempt: crate::publication::ReservedCompositePublicationAttempt,
        progress: CompositeAttemptProgress,
        cause: ProductUnpublishedCause,
        no_effect: NoEffectCause,
    ) -> OwnerExecutionOutcome {
        if progress.owner_effect_count() == 0 {
            return self.no_effect(attempt, no_effect);
        }
        let observed = self.current_product_head_snapshot(attempt.expected_head());
        self.retain_after_effect(attempt, progress, cause, observed)
    }

    fn no_effect(
        &self,
        attempt: crate::publication::ReservedCompositePublicationAttempt,
        cause: NoEffectCause,
    ) -> OwnerExecutionOutcome {
        let expected = attempt.expected_head().clone();
        let observed = (cause == NoEffectCause::StaleExpectedProductHead)
            .then(|| self.current_product_head_snapshot(&expected))
            .flatten();
        OwnerExecutionOutcome::NoEffect(
            NoEffectCompositePublication::new(cause, Some(expected)).with_observed_head(observed),
        )
    }
}

impl CompositeAttemptProgress {
    fn into_relational(self) -> RelationalAttemptProgress {
        let (relational, _) = self.into_parts();
        relational
    }
}
