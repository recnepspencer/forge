use std::sync::Arc;

use super::{
    UiIntentExecutionSlotPhase, UiIntentExecutionState, UiRecoveringFrameworkIntentAttempt,
    UiSettledFrameworkIntentAttempt,
};
use crate::runtime::intent_execution::provider::UiManagedIntentRecoveryPoll;
use crate::runtime::intent_execution::{
    UiIntentConsequenceHandle, UiIntentExecutionPollContext, UiIntentRecoveryHandle,
    UiIntentRecoveryProgressOutcome, UiIntentRecoveryProgressPosture,
    UiIntentRecoveryProgressReceipt, UiIntentRecoveryProgressStop,
};

struct UiRecoveringIntentProgression {
    reservation: super::UiActiveIntentExecutionReservation,
    attempt: crate::runtime::intent_execution::UiIntentExecutionAttemptIdentity,
    idempotency: crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity,
    lease: Arc<crate::runtime::intent_execution::UiIntentRecoveryLease>,
    partial: Option<crate::runtime::intent_execution::provider::UiManagedIntentPartialEffect>,
}

struct UiIntentRecoveryContinuation {
    recovery: Box<dyn crate::runtime::intent_execution::provider::UiManagedIntentRecovery>,
    handle_lease: Arc<crate::runtime::intent_execution::UiIntentRecoveryLease>,
    posture: UiIntentRecoveryProgressPosture,
}

impl UiIntentExecutionState {
    pub(crate) fn retry_recovery(
        &mut self,
        recovery: UiIntentRecoveryHandle,
        tick: u64,
    ) -> UiIntentRecoveryProgressOutcome {
        if let Err((previous, observed)) = self.admit_monotonic_tick(tick) {
            return stopped(
                UiIntentRecoveryProgressStop::MonotonicTimeRegressed { previous, observed },
                recovery,
            );
        }
        if !self.exact_recovery(&recovery) {
            return stopped(UiIntentRecoveryProgressStop::StaleOrForeign, recovery);
        }
        let (attempt, _validated_idempotency, handle_lease) = recovery.into_parts();
        let index = attempt.slot() as usize;
        let Some(UiIntentExecutionSlotPhase::Recovery(recovering)) = self.slots[index].phase.take()
        else {
            unreachable!("exact recovery identity points at recovery phase")
        };
        let UiRecoveringFrameworkIntentAttempt {
            reservation,
            recovery: recovery_lane,
            attempt,
            idempotency,
            lease,
            partial,
        } = recovering;
        let progression = UiRecoveringIntentProgression {
            reservation,
            attempt,
            idempotency,
            lease,
            partial,
        };
        let poll = recovery_lane.poll(UiIntentExecutionPollContext::at_tick(tick));
        self.accept_recovery_poll(index, progression, poll, handle_lease)
    }

    fn accept_recovery_poll(
        &mut self,
        index: usize,
        mut progression: UiRecoveringIntentProgression,
        poll: UiManagedIntentRecoveryPoll,
        handle_lease: Arc<crate::runtime::intent_execution::UiIntentRecoveryLease>,
    ) -> UiIntentRecoveryProgressOutcome {
        match poll {
            UiManagedIntentRecoveryPoll::Pending(recovery) => self.retain_recovery(
                index,
                progression,
                UiIntentRecoveryContinuation {
                    recovery,
                    handle_lease,
                    posture: UiIntentRecoveryProgressPosture::Pending,
                },
            ),
            UiManagedIntentRecoveryPoll::Partial { effect, recovery } => {
                let posture = UiIntentRecoveryProgressPosture::Partial {
                    outcome: effect.outcome_schema(),
                    detail: effect.detail(),
                };
                progression.partial = Some(effect);
                self.retain_recovery(
                    index,
                    progression,
                    UiIntentRecoveryContinuation {
                        recovery,
                        handle_lease,
                        posture,
                    },
                )
            }
            UiManagedIntentRecoveryPoll::Indeterminate { detail, recovery } => self
                .retain_recovery(
                    index,
                    progression,
                    UiIntentRecoveryContinuation {
                        recovery,
                        handle_lease,
                        posture: UiIntentRecoveryProgressPosture::Indeterminate { detail },
                    },
                ),
            UiManagedIntentRecoveryPoll::Failed { detail, recovery } => self.retain_recovery(
                index,
                progression,
                UiIntentRecoveryContinuation {
                    recovery,
                    handle_lease,
                    posture: UiIntentRecoveryProgressPosture::Failed { detail },
                },
            ),
            UiManagedIntentRecoveryPoll::Completed(outcome) => {
                self.complete_recovery(index, progression, outcome)
            }
        }
    }

    fn complete_recovery(
        &mut self,
        index: usize,
        progression: UiRecoveringIntentProgression,
        outcome: Box<
            dyn crate::runtime::intent_execution::provider::UiManagedIntentOutcomeMaterial,
        >,
    ) -> UiIntentRecoveryProgressOutcome {
        let posture = UiIntentRecoveryProgressPosture::Completed {
            outcome: outcome.schema(),
        };
        let (consequence, consequence_lease) =
            UiIntentConsequenceHandle::new(progression.attempt, progression.idempotency);
        let basis = progression.reservation.consequence_basis();
        self.release_reservation(progression.reservation);
        self.slots[index].phase = Some(UiIntentExecutionSlotPhase::ConsequencePending(
            UiSettledFrameworkIntentAttempt {
                attempt: progression.attempt,
                idempotency: progression.idempotency,
                outcome,
                consequence_lease,
                basis,
            },
        ));
        UiIntentRecoveryProgressOutcome::Progressed(UiIntentRecoveryProgressReceipt::new(
            progression.attempt,
            progression.idempotency,
            posture,
            None,
            Some(consequence),
        ))
    }

    fn exact_recovery(&self, handle: &UiIntentRecoveryHandle) -> bool {
        let Some(slot) = self.slots.get(handle.attempt().slot() as usize) else {
            return false;
        };
        slot.generation == handle.attempt().generation()
            && matches!(
                slot.phase.as_ref(),
                Some(UiIntentExecutionSlotPhase::Recovery(recovering))
                    if recovering.attempt == handle.attempt()
                        && recovering.idempotency == handle.idempotency()
                        && Arc::ptr_eq(&recovering.lease, handle.lease())
            )
    }

    fn retain_recovery(
        &mut self,
        index: usize,
        recovering: UiRecoveringIntentProgression,
        continuation: UiIntentRecoveryContinuation,
    ) -> UiIntentRecoveryProgressOutcome {
        let continuation_handle = UiIntentRecoveryHandle::from_parts(
            recovering.attempt,
            recovering.idempotency,
            continuation.handle_lease,
        );
        let receipt = UiIntentRecoveryProgressReceipt::new(
            recovering.attempt,
            recovering.idempotency,
            continuation.posture,
            Some(continuation_handle),
            None,
        );
        self.slots[index].phase = Some(UiIntentExecutionSlotPhase::Recovery(
            UiRecoveringFrameworkIntentAttempt {
                reservation: recovering.reservation,
                recovery: continuation.recovery,
                attempt: recovering.attempt,
                idempotency: recovering.idempotency,
                lease: recovering.lease,
                partial: recovering.partial,
            },
        ));
        UiIntentRecoveryProgressOutcome::Progressed(receipt)
    }
}

fn stopped(
    reason: UiIntentRecoveryProgressStop,
    recovery: UiIntentRecoveryHandle,
) -> UiIntentRecoveryProgressOutcome {
    UiIntentRecoveryProgressOutcome::Stopped { reason, recovery }
}
