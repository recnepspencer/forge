use worth_ui_host_contract::{
    UiHostSurfaceCancellationOutcome, UiHostSurfaceInFlightCompletion,
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationOutcome,
    UiMountedPresentationAttemptIdentity, UiMountedSurfaceBindingRequirement,
    UiPresentationDeadline,
};

use super::super::consumption_view::{
    UiMountedHostPresentationAuthority, UiRuntimeMountedFrameConsumptionInput,
};
use super::UiMountedPresentationCoordinator;
use crate::facade::UiHostEffectPort;

const SAMPLE_DEADLINE_TICKS: u64 = 32;

pub(super) struct UiPendingMotionSamplePresentation {
    prepared: super::super::motion_sampling::UiPreparedMotionSampling,
    token: Option<worth_ui_host_contract::UiHostPresentationCompletionToken>,
    attempt: UiMountedPresentationAttemptIdentity,
    requirement: UiMountedSurfaceBindingRequirement,
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    expected_effects: Box<[worth_ui_host_contract::UiMountedEffectFamily]>,
}

pub(crate) enum UiMotionSamplePresentationOutcome {
    Presented {
        prepared: super::super::motion_sampling::UiPreparedMotionSampling,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    },
    RejectedBeforeEffects(UiHostSurfacePresentationDenial),
    InFlight(UiMountedPresentationAttemptIdentity),
    Superseded,
    PresentationIndeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionSampleCancellationOutcome {
    NoPendingSample,
    CancelledBeforeEffects,
    PresentationIndeterminate,
}

impl UiMountedPresentationCoordinator {
    pub(crate) fn motion_sample_presentation_pending(&self) -> bool {
        self.motion_sample_in_flight.is_some()
    }

    pub(crate) fn pending_motion_sample_matches(
        &self,
        presentation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> bool {
        self.motion_sample_in_flight
            .as_ref()
            .is_some_and(|pending| {
                pending.attempt == presentation.attempt()
                    && pending.requirement.semantic_surface() == presentation.surface()
                    && pending.requirement.binding() == presentation.binding()
            })
    }

    pub(crate) fn present_motion_sample(
        &mut self,
        prepared: super::super::motion_sampling::UiPreparedMotionSampling,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        host: UiHostEffectPort<'_>,
        authority: UiMountedHostPresentationAuthority<'_>,
    ) -> UiMotionSamplePresentationOutcome {
        let Some(state) = self.presentation_states.get(&presentation.binding()) else {
            return UiMotionSamplePresentationOutcome::RejectedBeforeEffects(
                UiHostSurfacePresentationDenial::StalePredecessor,
            );
        };
        let requirement = state.motion_sample_requirement();
        if self.shutting_down
            || self.motion_sample_in_flight.is_some()
            || self.active.borrow().len() >= self.in_flight_limit
            || authority.capability_report().observation_generation()
                != requirement.capability_generation()
            || authority.capability_report().profile_identity_digest()
                != requirement.capability_profile_digest()
            || self
                .reconstruction_bindings
                .contains(&requirement.binding())
            || self
                .host_truth
                .surface_requires_reconciliation(requirement.semantic_surface())
        {
            return UiMotionSamplePresentationOutcome::RejectedBeforeEffects(
                UiHostSurfacePresentationDenial::AdapterDeclined,
            );
        }
        let work = match state.issue_motion_sample(
            prepared.receipt(),
            presentation,
            authority.presentation(),
        ) {
            Ok(work) => work,
            Err(_) => {
                return UiMotionSamplePresentationOutcome::RejectedBeforeEffects(
                    UiHostSurfacePresentationDenial::MalformedProjection,
                )
            }
        };
        let expected_effects = state
            .expected_completion_effects(Some(state), &work, requirement.presentation_mode())
            .into_boxed_slice();
        let Ok(attempt) = UiMountedPresentationAttemptIdentity::mint_unbound() else {
            return UiMotionSamplePresentationOutcome::RejectedBeforeEffects(
                UiHostSurfacePresentationDenial::CapacityExceeded,
            );
        };
        self.active.borrow_mut().insert(attempt);
        let deadline = UiPresentationDeadline::at_tick(
            prepared
                .receipt()
                .samples()
                .first()
                .map(|sample| sample.tick())
                .unwrap_or_default()
                .saturating_add(SAMPLE_DEADLINE_TICKS),
        );
        let view = authority.bind(UiRuntimeMountedFrameConsumptionInput {
            attempt,
            deadline,
            requirement,
            presentation_work: &work,
            text_raster_work: None,
        });
        let outcome = host
            .adapter()
            .present_mounted_surface(host.authority(), &view);
        self.settle_initial_motion_sample(
            prepared,
            attempt,
            requirement,
            presentation,
            expected_effects,
            outcome,
        )
    }

    pub(crate) fn complete_motion_sample(
        &mut self,
        host: UiHostEffectPort<'_>,
    ) -> Option<UiMotionSamplePresentationOutcome> {
        let mut pending = self.motion_sample_in_flight.take()?;
        let token = pending.token.take().expect("pending sample owns its token");
        let completion = host
            .adapter()
            .complete_mounted_surface(host.authority(), token);
        Some(self.settle_pending_motion_sample(pending, completion))
    }

    pub(crate) fn cancel_motion_sample(
        &mut self,
        host: UiHostEffectPort<'_>,
    ) -> UiMotionSampleCancellationOutcome {
        let Some(mut pending) = self.motion_sample_in_flight.take() else {
            return UiMotionSampleCancellationOutcome::NoPendingSample;
        };
        self.active.borrow_mut().remove(&pending.attempt);
        match host.adapter().cancel_mounted_surface(
            host.authority(),
            pending.token.take().expect("pending sample owns its token"),
            worth_ui_host_contract::UiHostSurfaceStopReason::Cancelled,
        ) {
            UiHostSurfaceCancellationOutcome::CancelledBeforeEffects => {
                UiMotionSampleCancellationOutcome::CancelledBeforeEffects
            }
            UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun => {
                self.reconstruction_bindings
                    .insert(pending.requirement.binding());
                UiMotionSampleCancellationOutcome::PresentationIndeterminate
            }
        }
    }

    fn settle_initial_motion_sample(
        &mut self,
        prepared: super::super::motion_sampling::UiPreparedMotionSampling,
        attempt: UiMountedPresentationAttemptIdentity,
        requirement: UiMountedSurfaceBindingRequirement,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        expected_effects: Box<[worth_ui_host_contract::UiMountedEffectFamily]>,
        outcome: UiHostSurfacePresentationOutcome,
    ) -> UiMotionSamplePresentationOutcome {
        match outcome {
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial) => {
                self.active.borrow_mut().remove(&attempt);
                UiMotionSamplePresentationOutcome::RejectedBeforeEffects(denial)
            }
            UiHostSurfacePresentationOutcome::Presented(completion) => {
                self.active.borrow_mut().remove(&attempt);
                let settled = settle_presented(
                    prepared,
                    requirement,
                    presentation,
                    &expected_effects,
                    completion,
                );
                if matches!(
                    settled,
                    UiMotionSamplePresentationOutcome::PresentationIndeterminate
                ) {
                    self.reconstruction_bindings.insert(requirement.binding());
                }
                settled
            }
            UiHostSurfacePresentationOutcome::InFlight(token) => {
                self.motion_sample_in_flight = Some(UiPendingMotionSamplePresentation {
                    prepared,
                    token: Some(token),
                    attempt,
                    requirement,
                    presentation,
                    expected_effects,
                });
                UiMotionSamplePresentationOutcome::InFlight(attempt)
            }
            UiHostSurfacePresentationOutcome::PresentationIndeterminate => {
                self.active.borrow_mut().remove(&attempt);
                self.reconstruction_bindings.insert(requirement.binding());
                UiMotionSamplePresentationOutcome::PresentationIndeterminate
            }
        }
    }

    fn settle_pending_motion_sample(
        &mut self,
        mut pending: UiPendingMotionSamplePresentation,
        completion: UiHostSurfaceInFlightCompletion,
    ) -> UiMotionSamplePresentationOutcome {
        match completion {
            UiHostSurfaceInFlightCompletion::Pending(token) => {
                pending.token = Some(token);
                let attempt = pending.attempt;
                self.motion_sample_in_flight = Some(pending);
                UiMotionSamplePresentationOutcome::InFlight(attempt)
            }
            UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(denial) => {
                self.active.borrow_mut().remove(&pending.attempt);
                UiMotionSamplePresentationOutcome::RejectedBeforeEffects(denial)
            }
            UiHostSurfaceInFlightCompletion::Presented(completion) => {
                self.active.borrow_mut().remove(&pending.attempt);
                let settled = settle_presented(
                    pending.prepared,
                    pending.requirement,
                    pending.presentation,
                    &pending.expected_effects,
                    completion,
                );
                if matches!(
                    settled,
                    UiMotionSamplePresentationOutcome::PresentationIndeterminate
                ) {
                    self.reconstruction_bindings
                        .insert(pending.requirement.binding());
                }
                settled
            }
            UiHostSurfaceInFlightCompletion::Superseded(_) => {
                self.active.borrow_mut().remove(&pending.attempt);
                UiMotionSamplePresentationOutcome::Superseded
            }
            UiHostSurfaceInFlightCompletion::PresentationIndeterminate => {
                self.active.borrow_mut().remove(&pending.attempt);
                self.reconstruction_bindings
                    .insert(pending.requirement.binding());
                UiMotionSamplePresentationOutcome::PresentationIndeterminate
            }
        }
    }

    pub(crate) fn mark_motion_sample_indeterminate(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) {
        self.reconstruction_bindings.insert(binding);
    }
}

fn settle_presented(
    prepared: super::super::motion_sampling::UiPreparedMotionSampling,
    requirement: UiMountedSurfaceBindingRequirement,
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    expected_effects: &[worth_ui_host_contract::UiMountedEffectFamily],
    completion: worth_ui_host_contract::UiMountedSurfacePresentationCompletion,
) -> UiMotionSamplePresentationOutcome {
    if completion.mode() != requirement.presentation_mode()
        || !super::super::terminal::completion_effects_satisfy(
            expected_effects,
            completion.effects().families(),
            completion.cost(),
        )
    {
        return UiMotionSamplePresentationOutcome::PresentationIndeterminate;
    }
    UiMotionSamplePresentationOutcome::Presented {
        prepared,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis::new(
            requirement.host_surface(),
            presentation.frame(),
            requirement.binding(),
            completion.epoch(),
        ),
    }
}
