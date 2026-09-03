use crate::branch::ProductBranchObservation;
use crate::lifecycle::{RuntimeWorldCancellationToken, RuntimeWorldInstant};
use crate::publication::{
    CompositeExecutionBorrow, LoweredOwnerComponentPlan, NoEffectCause, SignalAttemptProgress,
    SignalComponentPlanPosture,
};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

use worth_signal::facade::branch::{
    SignalBranchAdvanceDenial, SignalBranchForkOperationDenial, SignalOwnerCancellationSource,
};

pub(super) struct SignalExecutionFailure {
    pub(super) cause: ProductUnpublishedCause,
    pub(super) no_effect: NoEffectCause,
    pub(super) partial: SignalAttemptProgress,
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(super) fn execute_signal(
        &self,
        plan: &LoweredOwnerComponentPlan,
        expected_head: &ProductBranchObservation,
        deadline: Option<RuntimeWorldInstant>,
        reservation: Option<worth_signal::facade::branch::SignalBranchForkReservation<D, I, T>>,
        borrow: CompositeExecutionBorrow<'_, D, I, E, Ctx, T>,
        runtime_cancellation: &RuntimeWorldCancellationToken,
    ) -> Result<SignalAttemptProgress, SignalExecutionFailure> {
        match plan.signal().posture() {
            SignalComponentPlanPosture::RetainExact => Ok(SignalAttemptProgress::untouched()),
            SignalComponentPlanPosture::AdvanceExact => match borrow {
                CompositeExecutionBorrow::Signal {
                    context,
                    mutation,
                    cancellation,
                } => self
                    .state
                    .signal
                    .mutation_port()
                    .advance_exact(plan.signal().expected(), context, cancellation, mutation)
                    .map(SignalAttemptProgress::advanced)
                    .map_err(|denial| advance_failure(&denial, SignalAttemptProgress::untouched())),
                CompositeExecutionBorrow::WithoutSignal => Err(SignalExecutionFailure {
                    cause: ProductUnpublishedCause::SiblingOwnerDenied,
                    no_effect: NoEffectCause::OwnerUnavailable,
                    partial: SignalAttemptProgress::untouched(),
                }),
            },
            SignalComponentPlanPosture::ForkExact => {
                let reservation = reservation.ok_or_else(fork_failure_missing)?;
                match borrow {
                    CompositeExecutionBorrow::Signal {
                        context: _,
                        mutation: _,
                        cancellation,
                    } => self
                        .state
                        .signal
                        .mutation_port()
                        .fork_reserved_exact(reservation, cancellation)
                        .map(SignalAttemptProgress::forked)
                        .map_err(|denial| {
                            fork_failure(&denial, SignalAttemptProgress::untouched())
                        }),
                    CompositeExecutionBorrow::WithoutSignal => {
                        let source = SignalOwnerCancellationSource::new();
                        let cancellation = source.token();
                        self.state
                            .signal
                            .mutation_port()
                            .fork_reserved_exact(reservation, &cancellation)
                            .map(SignalAttemptProgress::forked)
                            .map_err(|denial| {
                                fork_failure(&denial, SignalAttemptProgress::untouched())
                            })
                    }
                }
            }
            SignalComponentPlanPosture::ForkAndAdvance => match borrow {
                CompositeExecutionBorrow::Signal {
                    context,
                    mutation,
                    cancellation,
                } => {
                    let reservation = reservation.ok_or_else(fork_failure_missing)?;
                    let forked = self
                        .state
                        .signal
                        .mutation_port()
                        .fork_reserved_exact(reservation, cancellation)
                        .map_err(|denial| {
                            fork_failure(&denial, SignalAttemptProgress::untouched())
                        })?;
                    let partial = SignalAttemptProgress::forked(forked.clone());
                    if let Err((cause, no_effect)) =
                        self.pre_advance_signal_gate(expected_head, deadline, runtime_cancellation)
                    {
                        return Err(SignalExecutionFailure {
                            cause,
                            no_effect,
                            partial,
                        });
                    }
                    let advanced = self
                        .state
                        .signal
                        .mutation_port()
                        .advance_exact(forked.created_basis(), context, cancellation, mutation)
                        .map_err(|denial| advance_failure(&denial, partial))?;
                    Ok(SignalAttemptProgress::forked_and_advanced(forked, advanced))
                }
                CompositeExecutionBorrow::WithoutSignal => Err(SignalExecutionFailure {
                    cause: ProductUnpublishedCause::SiblingOwnerDenied,
                    no_effect: NoEffectCause::OwnerUnavailable,
                    partial: SignalAttemptProgress::untouched(),
                }),
            },
        }
    }
}

fn fork_failure_missing() -> SignalExecutionFailure {
    SignalExecutionFailure {
        cause: ProductUnpublishedCause::SiblingOwnerDenied,
        no_effect: NoEffectCause::PreEffectFailure,
        partial: SignalAttemptProgress::untouched(),
    }
}

fn fork_failure(
    denial: &SignalBranchForkOperationDenial,
    partial: SignalAttemptProgress,
) -> SignalExecutionFailure {
    let cancellation = matches!(denial, SignalBranchForkOperationDenial::CancelledNoMovement);
    SignalExecutionFailure {
        cause: if cancellation {
            ProductUnpublishedCause::CancellationAfterEffect
        } else {
            ProductUnpublishedCause::SiblingOwnerDenied
        },
        no_effect: map_fork_no_effect(denial),
        partial,
    }
}

fn advance_failure(
    denial: &SignalBranchAdvanceDenial,
    partial: SignalAttemptProgress,
) -> SignalExecutionFailure {
    let cancellation = matches!(denial, SignalBranchAdvanceDenial::CancelledNoMovement);
    SignalExecutionFailure {
        cause: if cancellation {
            ProductUnpublishedCause::CancellationAfterEffect
        } else {
            ProductUnpublishedCause::SiblingOwnerDenied
        },
        no_effect: map_advance_no_effect(denial),
        partial,
    }
}

pub(super) fn map_fork_no_effect(denial: &SignalBranchForkOperationDenial) -> NoEffectCause {
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
