use worth_ui_host_contract::{UiHostSurfaceCancellationOutcome, UiHostSurfaceInFlightCompletion};

use super::super::outcome::{
    UiMountedPresentationOutcome, UiMountedSurfacePresentationReceipt,
    UiMountedSurfacePresentationRejection,
};
use super::super::state::{
    UiMountedPresentationCompletionDenial, UiMountedPresentationInFlight,
    UiMountedPresentationInFlightState, UiPendingMountedSurface,
};
use super::super::terminal::UiIndeterminatePresentationEvidence;
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
        if state.deadline.expired_at(now) {
            return Ok(self.finish_expired_completion(state, host));
        }
        Ok(self.poll_pending_completion(state, host))
    }

    fn finish_expired_completion(
        &mut self,
        state: UiMountedPresentationInFlightState,
        host: UiHostEffectPort<'_>,
    ) -> UiMountedPresentationOutcome {
        let UiMountedPresentationInFlightState {
            frame,
            retention,
            attempt,
            pending,
            rejected,
            completed,
            ..
        } = state;
        let affected = aggregate_affected(&completed, &pending, &rejected);
        let timeout_rejections = pending
            .iter()
            .map(|pending| pending.binding)
            .map(|binding| {
                UiMountedSurfacePresentationRejection::new(
                    binding,
                    worth_ui_host_contract::UiHostSurfacePresentationDenial::DeadlineExpired,
                )
            })
            .collect::<Vec<_>>();
        if cancel_all(pending, host) || !completed.is_empty() {
            return self.indeterminate(
                frame,
                retention,
                attempt,
                UiIndeterminatePresentationEvidence::new(affected, completed),
            );
        }
        self.active.borrow_mut().remove(&attempt);
        let mut rejections = rejected;
        rejections.extend(timeout_rejections);
        rejected_outcome(attempt, frame, retention, rejections)
    }

    fn poll_pending_completion(
        &mut self,
        state: UiMountedPresentationInFlightState,
        host: UiHostEffectPort<'_>,
    ) -> UiMountedPresentationOutcome {
        let UiMountedPresentationInFlightState {
            frame,
            retention,
            attempt,
            deadline,
            pending,
            rejected,
            completed,
            candidates,
        } = state;
        let mut progress = super::UiMountedPresentationProgress {
            pending: Vec::new(),
            rejected,
            completed,
        };
        let mut remaining = Vec::new();
        let mut pending_iter = pending.into_iter();
        while let Some(pending_surface) = pending_iter.next() {
            if let Some((binding, additional_cost)) = observe_pending_surface(
                &frame,
                host,
                pending_surface,
                &mut progress,
                &mut self.text,
            ) {
                remaining.extend(pending_iter);
                progress.pending.extend(remaining);
                let evidence =
                    terminalize_pending_uncertainty(&mut progress, host, binding, additional_cost);
                return self.indeterminate(frame, retention, attempt, evidence);
            }
        }
        progress.pending.extend(remaining);
        self.finish_or_wait(super::UiMountedPresentationSettlement {
            frame,
            retention,
            attempt,
            deadline,
            pending: progress.pending,
            rejected: progress.rejected,
            completed: progress.completed,
            candidates,
            host,
        })
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
            retention,
            attempt,
            pending,
            rejected,
            completed,
            ..
        } = state;
        let affected = aggregate_affected(&completed, &pending, &rejected);
        let cancellation_rejections = pending
            .iter()
            .map(|pending| pending.binding)
            .map(|binding| {
                UiMountedSurfacePresentationRejection::new(
                    binding,
                    worth_ui_host_contract::UiHostSurfacePresentationDenial::CancelledBeforeEffects,
                )
            })
            .collect::<Vec<_>>();
        let effects_may_have_begun = cancel_all(pending, host);
        if effects_may_have_begun || !completed.is_empty() {
            return self.indeterminate(
                frame,
                retention,
                attempt,
                UiIndeterminatePresentationEvidence::new(affected, completed),
            );
        }
        self.active.borrow_mut().remove(&attempt);
        let mut rejections = rejected;
        rejections.extend(cancellation_rejections);
        rejected_outcome(attempt, frame, retention, rejections)
    }
}

fn observe_pending_surface(
    frame: &super::super::super::UiPreparedMountedFrame,
    host: UiHostEffectPort<'_>,
    pending: UiPendingMountedSurface,
    progress: &mut super::UiMountedPresentationProgress,
    text: &mut crate::native_platform::text_presentation::UiNativeMountedTextCoordinator,
) -> Option<(
    worth_ui_host_contract::UiSurfaceBindingGeneration,
    Option<worth_ui_host_contract::UiHostPresentationCostReport>,
)> {
    let UiPendingMountedSurface {
        binding,
        token,
        expected_effects,
        text_candidate,
    } = pending;
    match host
        .adapter()
        .complete_mounted_surface(host.authority(), token)
    {
        UiHostSurfaceInFlightCompletion::Pending(token) => {
            progress.pending.push(UiPendingMountedSurface {
                binding,
                token,
                expected_effects,
                text_candidate,
            });
            None
        }
        UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(denial) => {
            progress
                .rejected
                .push(UiMountedSurfacePresentationRejection::new(binding, denial));
            None
        }
        UiHostSurfaceInFlightCompletion::PresentationIndeterminate => Some((binding, None)),
        UiHostSurfaceInFlightCompletion::Presented(completion) => {
            let surface = frame
                .surfaces()
                .iter()
                .find(|surface| surface.requirement().binding() == binding)
                .expect("pending binding belongs to retained prepared frame");
            if !completion_satisfies(surface, &expected_effects, &completion) {
                return Some((binding, Some(completion.cost())));
            }
            let (epoch, effects, adapter_cost) = completion.into_parts();
            progress
                .completed
                .push(UiMountedSurfacePresentationReceipt::new(
                    surface.requirement(),
                    epoch,
                    effects,
                    adapter_cost,
                ));
            if let Some(candidate) = text_candidate {
                text.commit_surface_candidate(candidate);
            }
            None
        }
    }
}

fn terminalize_pending_uncertainty(
    progress: &mut super::UiMountedPresentationProgress,
    host: UiHostEffectPort<'_>,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
) -> UiIndeterminatePresentationEvidence {
    let mut affected =
        aggregate_affected(&progress.completed, &progress.pending, &progress.rejected);
    affected.push(binding);
    cancel_all(std::mem::take(&mut progress.pending), host);
    let evidence =
        UiIndeterminatePresentationEvidence::new(affected, std::mem::take(&mut progress.completed));
    match additional_cost {
        Some(cost) => evidence.with_additional_adapter_cost(cost),
        None => evidence,
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
