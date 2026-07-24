use worth_ui_host_contract::{UiHostSurfaceCancellationOutcome, UiHostSurfaceInFlightCompletion};

use super::super::outcome::{
    UiMountedPresentationOutcome, UiMountedSurfacePresentationReceipt,
    UiMountedSurfacePresentationRejection,
};
use super::super::state::{
    UiMountedPresentationCompletionDenial, UiMountedPresentationInFlight,
    UiMountedPresentationInFlightState, UiPendingMountedSurface,
};
use super::super::terminal::{aggregate_affected, completion_satisfies, rejected_outcome};
use super::UiMountedPresentationCoordinator;
use crate::facade::UiHostEffectPort;

impl UiMountedPresentationCoordinator {
    pub fn complete(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
        host: UiHostEffectPort<'_>,
        now: u64,
    ) -> Result<UiMountedPresentationOutcome, UiMountedPresentationCompletionDenial> {
        let attempt_identity = in_flight.attempt();
        let Some(state) = self.in_flight.remove(&attempt_identity) else {
            return Err(UiMountedPresentationCompletionDenial::UnknownAttempt);
        };
        let UiMountedPresentationInFlightState {
            frame,
            attempt,
            deadline,
            pending,
            mut rejected,
            mut completed,
        } = state;
        if deadline.expired_at(now) {
            let affected = aggregate_affected(&completed, &pending, &rejected);
            let timeout_rejections = pending
                .iter()
                .map(|pending| {
                    UiMountedSurfacePresentationRejection::new(
                        pending.binding,
                        worth_ui_host_contract::UiHostSurfacePresentationDenial::DeadlineExpired,
                    )
                })
                .collect::<Vec<_>>();
            let effects_may_have_begun = cancel_all(pending, host);
            if effects_may_have_begun || !completed.is_empty() {
                return Ok(self.indeterminate(frame, attempt, affected));
            }
            self.active.borrow_mut().remove(&attempt);
            let mut rejections = rejected;
            rejections.extend(timeout_rejections);
            return Ok(rejected_outcome(attempt, frame, rejections));
        }
        let mut remaining = Vec::new();
        let mut pending_iter = pending.into_iter();
        while let Some(pending_surface) = pending_iter.next() {
            let UiPendingMountedSurface { binding, token } = pending_surface;
            match host
                .adapter()
                .complete_mounted_surface(host.authority(), token)
            {
                UiHostSurfaceInFlightCompletion::Pending(token) => {
                    remaining.push(UiPendingMountedSurface { binding, token });
                }
                UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(denial) => {
                    rejected.push(UiMountedSurfacePresentationRejection::new(binding, denial));
                }
                UiHostSurfaceInFlightCompletion::PresentationIndeterminate => {
                    remaining.extend(pending_iter);
                    let mut affected = aggregate_affected(&completed, &remaining, &rejected);
                    affected.push(binding);
                    cancel_all(remaining, host);
                    return Ok(self.indeterminate(frame, attempt, affected));
                }
                UiHostSurfaceInFlightCompletion::Presented(completion) => {
                    let surface = frame
                        .surfaces()
                        .iter()
                        .find(|surface| surface.requirement().binding() == binding)
                        .expect("pending binding belongs to retained prepared frame");
                    if !completion_satisfies(surface, &completion) {
                        remaining.extend(pending_iter);
                        let mut affected = aggregate_affected(&completed, &remaining, &rejected);
                        affected.push(binding);
                        cancel_all(remaining, host);
                        return Ok(self.indeterminate(frame, attempt, affected));
                    }
                    let (effects, adapter_cost) = completion.into_parts();
                    completed.push(UiMountedSurfacePresentationReceipt::new(
                        binding,
                        effects,
                        adapter_cost,
                    ));
                }
            }
        }
        Ok(self.finish_or_wait(
            frame, attempt, deadline, remaining, rejected, completed, host,
        ))
    }

    pub fn cancel(
        &mut self,
        in_flight: UiMountedPresentationInFlight,
        host: UiHostEffectPort<'_>,
    ) -> Result<UiMountedPresentationOutcome, UiMountedPresentationCompletionDenial> {
        let Some(state) = self.in_flight.remove(&in_flight.attempt()) else {
            return Err(UiMountedPresentationCompletionDenial::UnknownAttempt);
        };
        Ok(self.cancel_state(state, host))
    }

    pub(crate) fn shutdown(
        &mut self,
        host: UiHostEffectPort<'_>,
    ) -> (
        super::super::UiMountedPresentationShutdownReport,
        Vec<UiMountedPresentationOutcome>,
    ) {
        self.shutting_down = true;
        let attempts = self.in_flight.keys().copied().collect::<Vec<_>>();
        let mut records = Vec::with_capacity(attempts.len());
        let mut outcomes = Vec::with_capacity(attempts.len());
        for attempt in attempts {
            let state = self
                .in_flight
                .remove(&attempt)
                .expect("shutdown attempt was retained by the coordinator");
            let outcome = self.cancel_state(state, host);
            let (disposition, affected) = match &outcome {
                UiMountedPresentationOutcome::RejectedBeforeEffects(_) => (
                    super::super::UiMountedPresentationShutdownDisposition::CancelledBeforeEffects,
                    Vec::new(),
                ),
                UiMountedPresentationOutcome::PresentationIndeterminate(frame) => (
                    super::super::UiMountedPresentationShutdownDisposition::PresentationIndeterminate,
                    frame.report().affected_bindings().to_vec(),
                ),
                UiMountedPresentationOutcome::Presented(_)
                | UiMountedPresentationOutcome::InFlight(_) => {
                    unreachable!("shutdown cancellation always reaches a terminal state")
                }
            };
            records.push(super::super::UiMountedPresentationShutdownAttempt::new(
                attempt,
                disposition,
                affected,
            ));
            outcomes.push(outcome);
        }
        (
            super::super::UiMountedPresentationShutdownReport::new(records),
            outcomes,
        )
    }

    fn cancel_state(
        &mut self,
        state: UiMountedPresentationInFlightState,
        host: UiHostEffectPort<'_>,
    ) -> UiMountedPresentationOutcome {
        let UiMountedPresentationInFlightState {
            frame,
            attempt,
            pending,
            rejected,
            completed,
            ..
        } = state;
        let affected = aggregate_affected(&completed, &pending, &rejected);
        let cancellation_rejections = pending
            .iter()
            .map(|pending| {
                UiMountedSurfacePresentationRejection::new(
                    pending.binding,
                    worth_ui_host_contract::UiHostSurfacePresentationDenial::CancelledBeforeEffects,
                )
            })
            .collect::<Vec<_>>();
        let effects_may_have_begun = cancel_all(pending, host);
        if effects_may_have_begun || !completed.is_empty() {
            return self.indeterminate(frame, attempt, affected);
        }
        self.active.borrow_mut().remove(&attempt);
        let mut rejections = rejected;
        rejections.extend(cancellation_rejections);
        rejected_outcome(attempt, frame, rejections)
    }
}

pub(super) fn cancel_all(
    pending: Vec<UiPendingMountedSurface>,
    host: UiHostEffectPort<'_>,
) -> bool {
    let mut effects_may_have_begun = false;
    for pending_surface in pending {
        effects_may_have_begun |= host
            .adapter()
            .cancel_mounted_surface(host.authority(), pending_surface.token)
            == UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun;
    }
    effects_may_have_begun
}
