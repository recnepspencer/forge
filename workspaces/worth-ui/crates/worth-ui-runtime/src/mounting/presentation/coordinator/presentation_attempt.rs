use worth_ui_host_contract::{
    UiHostSurfacePresentationOutcome, UiMountedPresentationAttemptIdentity, UiPresentationDeadline,
    UiSurfaceBindingGeneration,
};

use super::super::consumption_view::{
    UiMountedHostPresentationAuthority, UiRuntimeMountedFrameConsumptionInput,
};
use super::super::outcome::{
    UiMountedSurfacePresentationReceipt, UiMountedSurfacePresentationRejection,
};
use super::super::terminal::{
    aggregate_affected, completion_satisfies, UiIndeterminatePresentationEvidence,
};
use crate::facade::UiHostEffectPort;

pub(super) struct UiMountedPresentationStart<'host, 'authority> {
    pub(super) frame: crate::mounting::UiPreparedMountedFrame,
    pub(super) retention: crate::mounting::retention::UiMountedRetentionReservation,
    pub(super) attempt: UiMountedPresentationAttemptIdentity,
    pub(super) deadline: UiPresentationDeadline,
    pub(super) host: UiHostEffectPort<'host>,
    pub(super) authority: UiMountedHostPresentationAuthority<'authority>,
}

#[derive(Default)]
pub(super) struct UiMountedPresentationProgress {
    pub(super) pending: Vec<super::super::state::UiPendingMountedSurface>,
    pub(super) rejected: Vec<UiMountedSurfacePresentationRejection>,
    pub(super) completed: Vec<UiMountedSurfacePresentationReceipt>,
}

pub(super) fn present_one_surface(
    start: &UiMountedPresentationStart<'_, '_>,
    surface: &crate::mounting::UiMountedSurfaceReceipt,
    presentation_work: &super::super::UiMountedPresentationWork,
    expected_effects: &[worth_ui_host_contract::UiMountedEffectFamily],
    progress: &mut UiMountedPresentationProgress,
) -> Result<(), UiIndeterminatePresentationEvidence> {
    let requirement = surface.requirement();
    let view = start.authority.bind(UiRuntimeMountedFrameConsumptionInput {
        attempt: start.attempt,
        deadline: start.deadline,
        requirement,
        presentation_work,
    });
    match start
        .host
        .adapter()
        .present_mounted_surface(start.host.authority(), &view)
    {
        UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial) => {
            progress
                .rejected
                .push(UiMountedSurfacePresentationRejection::new(
                    requirement.binding(),
                    denial,
                ));
            Ok(())
        }
        UiHostSurfacePresentationOutcome::InFlight(token) => {
            progress
                .pending
                .push(super::super::state::UiPendingMountedSurface {
                    binding: requirement.binding(),
                    token,
                    expected_effects: expected_effects.to_vec().into_boxed_slice(),
                });
            Ok(())
        }
        UiHostSurfacePresentationOutcome::PresentationIndeterminate => Err(
            terminalize_surface_uncertainty(progress, start.host, requirement.binding(), None),
        ),
        UiHostSurfacePresentationOutcome::Presented(completion) => {
            if !completion_satisfies(surface, expected_effects, &completion) {
                return Err(terminalize_surface_uncertainty(
                    progress,
                    start.host,
                    requirement.binding(),
                    Some(completion.cost()),
                ));
            }
            let (epoch, effects, adapter_cost) = completion.into_parts();
            progress
                .completed
                .push(UiMountedSurfacePresentationReceipt::new(
                    requirement,
                    epoch,
                    effects,
                    adapter_cost,
                ));
            Ok(())
        }
    }
}

fn terminalize_surface_uncertainty(
    progress: &mut UiMountedPresentationProgress,
    host: UiHostEffectPort<'_>,
    binding: UiSurfaceBindingGeneration,
    additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
) -> UiIndeterminatePresentationEvidence {
    let mut affected =
        aggregate_affected(&progress.completed, &progress.pending, &progress.rejected);
    affected.push(binding);
    super::settlement::cancel_all(std::mem::take(&mut progress.pending), host);
    let evidence =
        UiIndeterminatePresentationEvidence::new(affected, std::mem::take(&mut progress.completed));
    match additional_cost {
        Some(cost) => evidence.with_additional_adapter_cost(cost),
        None => evidence,
    }
}
