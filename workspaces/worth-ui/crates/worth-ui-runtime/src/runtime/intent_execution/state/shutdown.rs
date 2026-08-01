use super::{UiIntentExecutionSlotPhase, UiIntentExecutionState};
use crate::runtime::intent::UiIntentAdmissionCancellationReason;
use crate::runtime::intent_execution::provider::{
    UiManagedIntentExecutionPoll, UiManagedIntentSettlement,
};
use crate::runtime::intent_execution::{
    UiIntentExecutionCancellationContext, UiIntentExecutionCancellationReason,
    UiIntentExecutionShutdownCounts, UiIntentExecutionShutdownReport,
};

impl UiIntentExecutionState {
    pub(crate) fn shutdown(&mut self) -> UiIntentExecutionShutdownReport {
        let mut counts = UiIntentExecutionShutdownCounts::default();
        let tick = self.last_tick.unwrap_or(0);
        for index in 0..self.slots.len() {
            let Some(phase) = self.slots[index].phase.take() else {
                continue;
            };
            self.dispose_phase(phase, tick, &mut counts);
        }
        counts.active_after = self.census().active_attempts;
        debug_assert_eq!(counts.active_after, 0);
        debug_assert_eq!(self.occupancy.active_count(), 0);
        UiIntentExecutionShutdownReport::from_counts(counts)
    }

    fn dispose_phase(
        &mut self,
        phase: UiIntentExecutionSlotPhase,
        tick: u64,
        counts: &mut UiIntentExecutionShutdownCounts,
    ) {
        match phase {
            UiIntentExecutionSlotPhase::Admitted(reserved) => {
                reserved
                    .reservation
                    .core
                    .lease
                    .mark_cancelled(UiIntentAdmissionCancellationReason::Shutdown);
                let _ = self.occupancy.release(reserved.reservation.core.occupancy);
                counts.execution_entries_disposed += 1;
                counts.reservation_backed_entries_disposed += 1;
                counts.before_effect_disposals += 1;
            }
            UiIntentExecutionSlotPhase::AttemptPrepared(prepared) => {
                prepared
                    .reservation
                    .core
                    .lease
                    .mark_cancelled(UiIntentAdmissionCancellationReason::Shutdown);
                self.release_reservation(prepared.reservation);
                counts.execution_entries_disposed += 1;
                counts.reservation_backed_entries_disposed += 1;
                counts.before_effect_disposals += 1;
            }
            UiIntentExecutionSlotPhase::Running(running) => {
                let reason = running
                    .cancellation
                    .unwrap_or(UiIntentExecutionCancellationReason::Shutdown);
                let poll = running
                    .execution
                    .cancel(UiIntentExecutionCancellationContext::new(tick, reason));
                self.release_reservation(running.reservation);
                counts.execution_entries_disposed += 1;
                counts.reservation_backed_entries_disposed += 1;
                counts.provider_cancellation_calls += 1;
                record_poll_disposal(poll, counts);
            }
            UiIntentExecutionSlotPhase::Recovery(recovering) => {
                self.release_reservation(recovering.reservation);
                counts.execution_entries_disposed += 1;
                counts.reservation_backed_entries_disposed += 1;
                counts.recovery_lanes_disposed += 1;
                if recovering.partial.is_some() {
                    counts.partial_effect_disposals += 1;
                } else {
                    counts.indeterminate_effect_disposals += 1;
                }
            }
            UiIntentExecutionSlotPhase::ConsequencePending(_)
            | UiIntentExecutionSlotPhase::ConsequenceReady(_)
            | UiIntentExecutionSlotPhase::ConsequenceHandoff(_) => {
                counts.execution_entries_disposed += 1;
                counts.completed_outcomes_disposed += 1;
                counts.consequence_pending_outcomes_disposed += 1;
            }
        }
    }
}

fn record_poll_disposal(
    poll: UiManagedIntentExecutionPoll,
    counts: &mut UiIntentExecutionShutdownCounts,
) {
    match poll {
        UiManagedIntentExecutionPoll::PendingBeforeEffect(_) => {
            counts.before_effect_disposals += 1;
        }
        UiManagedIntentExecutionPoll::PendingEffectMayHaveBegun(_) => {
            counts.indeterminate_effect_disposals += 1;
        }
        UiManagedIntentExecutionPoll::Settled(settlement) => {
            record_settlement_disposal(settlement, counts);
        }
    }
}

fn record_settlement_disposal(
    settlement: UiManagedIntentSettlement,
    counts: &mut UiIntentExecutionShutdownCounts,
) {
    match settlement {
        UiManagedIntentSettlement::Completed(_) => counts.completed_outcomes_disposed += 1,
        UiManagedIntentSettlement::Partial { .. } => {
            counts.partial_effect_disposals += 1;
            counts.recovery_lanes_disposed += 1;
        }
        UiManagedIntentSettlement::Indeterminate { .. } => {
            counts.indeterminate_effect_disposals += 1;
            counts.recovery_lanes_disposed += 1;
        }
        UiManagedIntentSettlement::RejectedBeforeEffect(_)
        | UiManagedIntentSettlement::FailedBeforeEffect(_)
        | UiManagedIntentSettlement::CancelledBeforeEffect(_)
        | UiManagedIntentSettlement::TimedOutBeforeEffect(_) => {
            counts.before_effect_disposals += 1;
        }
    }
}
