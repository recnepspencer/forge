use crate::publication::RuntimeWorldCancellationToken;
use crate::publication::{
    LoweredOwnerComponentPlan, NoEffectCause, SignalAttemptProgress, SignalComponentPlanPosture,
};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

use worth_signal::facade::branch::{SignalBranchAdvanceDenial, SignalBranchForkOperationDenial};
use worth_signal::facade::{SignalError, SignalTransaction};

pub(super) struct SignalExecutionFailure {
    pub(super) cause: ProductUnpublishedCause,
    pub(super) no_effect: NoEffectCause,
    pub(super) partial: SignalAttemptProgress,
}

/// The Signal borrow one publication is allowed to hold. The advancing arm
/// carries the caller's own mutation body unboxed, so the execution seam never
/// erases the closure it was handed.
pub(super) enum SignalExecutionRequest<'a, Ctx, F> {
    RetainExact,
    AdvanceExact { runtime_ctx: &'a mut Ctx, apply: F },
}

/// The mutation type used when a publication declares it will not touch the
/// Signal owner. It names a body that cannot exist, so `RetainExact` is the
/// only inhabited request on that path.
pub(super) type UntouchedSignalMutation<D, I, E, Ctx, T> =
    fn(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// The Signal publication leg. Publication either retains the exact
    /// component basis or advances it exactly once; creating a branch is a
    /// separate owner operation and never reaches this path.
    pub(super) fn execute_signal<F>(
        &self,
        plan: &LoweredOwnerComponentPlan,
        request: SignalExecutionRequest<'_, Ctx, F>,
        runtime_cancellation: &RuntimeWorldCancellationToken,
    ) -> Result<SignalAttemptProgress, SignalExecutionFailure>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        match (plan.signal().posture(), request) {
            (SignalComponentPlanPosture::RetainExact, _) => Ok(SignalAttemptProgress::untouched()),
            (
                SignalComponentPlanPosture::AdvanceExact,
                SignalExecutionRequest::AdvanceExact { runtime_ctx, apply },
            ) => self
                .state
                .signal
                .mutation_port()
                .advance_exact(
                    plan.signal().expected(),
                    runtime_ctx,
                    runtime_cancellation.signal_token(),
                    apply,
                )
                .map(SignalAttemptProgress::advanced)
                .map_err(|denial| advance_failure(&denial)),
            // An advancing plan reached the seam without the caller's Signal
            // borrow. The plan and the borrow are chosen together at the
            // typestate, so this is an owner the publication cannot reach.
            (SignalComponentPlanPosture::AdvanceExact, SignalExecutionRequest::RetainExact) => {
                Err(SignalExecutionFailure {
                    cause: ProductUnpublishedCause::SiblingOwnerDenied,
                    no_effect: NoEffectCause::OwnerUnavailable,
                    partial: SignalAttemptProgress::untouched(),
                })
            }
        }
    }
}

fn advance_failure(denial: &SignalBranchAdvanceDenial) -> SignalExecutionFailure {
    let cancellation = matches!(denial, SignalBranchAdvanceDenial::CancelledNoMovement);
    SignalExecutionFailure {
        cause: if cancellation {
            ProductUnpublishedCause::CancellationAfterEffect
        } else {
            ProductUnpublishedCause::SiblingOwnerDenied
        },
        no_effect: map_advance_no_effect(denial),
        partial: SignalAttemptProgress::untouched(),
    }
}

pub(crate) fn map_fork_no_effect(denial: &SignalBranchForkOperationDenial) -> NoEffectCause {
    match denial {
        SignalBranchForkOperationDenial::CancelledNoMovement => {
            NoEffectCause::CancelledBeforeEffect
        }
        SignalBranchForkOperationDenial::OperationCapacityExhausted { .. }
        | SignalBranchForkOperationDenial::LiveBranchCapacityExhausted { .. }
        | SignalBranchForkOperationDenial::ReservationCapacityExhausted { .. }
        | SignalBranchForkOperationDenial::RetentionUnavailable { .. } => {
            NoEffectCause::CapacityExhausted
        }
        SignalBranchForkOperationDenial::OwnerUnavailable(_) => NoEffectCause::OwnerUnavailable,
        _ => NoEffectCause::PreEffectFailure,
    }
}

fn map_advance_no_effect(denial: &SignalBranchAdvanceDenial) -> NoEffectCause {
    match denial {
        SignalBranchAdvanceDenial::CancelledNoMovement => NoEffectCause::CancelledBeforeEffect,
        SignalBranchAdvanceDenial::OperationCapacityExhausted { .. }
        | SignalBranchAdvanceDenial::RetentionUnavailable { .. } => {
            NoEffectCause::CapacityExhausted
        }
        SignalBranchAdvanceDenial::OwnerUnavailable(_) => NoEffectCause::OwnerUnavailable,
        _ => NoEffectCause::PreEffectFailure,
    }
}
