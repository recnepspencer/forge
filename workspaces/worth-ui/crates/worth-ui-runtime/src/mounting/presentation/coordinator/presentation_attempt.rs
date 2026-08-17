use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationOutcome,
    UiMountedPresentationAttemptIdentity, UiPresentationDeadline, UiSurfaceBindingGeneration,
    WorthUiHostKind,
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
use crate::native_platform::text_presentation::UiNativeTextPresentationReadiness;

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
    pub(super) pending_text: Vec<super::super::state::UiPendingMountedTextRaster>,
    pub(super) rejected: Vec<UiMountedSurfacePresentationRejection>,
    pub(super) completed: Vec<UiMountedSurfacePresentationReceipt>,
}

pub(super) fn present_one_surface(
    start: &UiMountedPresentationStart<'_, '_>,
    surface: &crate::mounting::UiMountedSurfaceReceipt,
    presentation_work: &super::super::UiMountedPresentationWork,
    expected_effects: &[worth_ui_host_contract::UiMountedEffectFamily],
    progress: &mut UiMountedPresentationProgress,
    text: &mut crate::native_platform::text_presentation::UiNativeMountedTextCoordinator,
) -> Result<(), UiIndeterminatePresentationEvidence> {
    let requirement = surface.requirement();
    if native_host_owns_semantic_text_boundary(
        start.host.adapter().operational_host_contract().kind(),
    ) {
        if let Some(outcome) = super::semantic_text_raster::present(
            start,
            requirement,
            presentation_work,
            progress,
            text,
        ) {
            return record_presentation_outcome(
                start,
                surface,
                expected_effects,
                progress,
                outcome,
            );
        }
        if progress
            .rejected
            .iter()
            .any(|rejection| rejection.binding() == requirement.binding())
        {
            return Ok(());
        }
    }
    let view = start.authority.bind(UiRuntimeMountedFrameConsumptionInput {
        attempt: start.attempt,
        deadline: start.deadline,
        requirement,
        presentation_work,
        text_raster_work: None,
    });
    let outcome = start
        .host
        .adapter()
        .present_mounted_surface(start.host.authority(), &view);
    record_presentation_outcome(start, surface, expected_effects, progress, outcome)
}

fn record_presentation_outcome(
    start: &UiMountedPresentationStart<'_, '_>,
    surface: &crate::mounting::UiMountedSurfaceReceipt,
    expected_effects: &[worth_ui_host_contract::UiMountedEffectFamily],
    progress: &mut UiMountedPresentationProgress,
    outcome: UiHostSurfacePresentationOutcome,
) -> Result<(), UiIndeterminatePresentationEvidence> {
    let requirement = surface.requirement();
    match outcome {
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

pub(super) fn terminalize_surface_uncertainty(
    progress: &mut UiMountedPresentationProgress,
    host: UiHostEffectPort<'_>,
    binding: UiSurfaceBindingGeneration,
    additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
) -> UiIndeterminatePresentationEvidence {
    let mut affected =
        aggregate_affected(&progress.completed, &progress.pending, &progress.rejected);
    for binding in progress.pending_text.iter().map(|pending| pending.binding) {
        if !affected.contains(&binding) {
            affected.push(binding);
        }
    }
    affected.push(binding);
    super::settlement::cancel_all(std::mem::take(&mut progress.pending), host);
    super::text_raster_settlement::cancel_all_text(
        std::mem::take(&mut progress.pending_text),
        host,
    );
    let evidence =
        UiIndeterminatePresentationEvidence::new(affected, std::mem::take(&mut progress.completed));
    match additional_cost {
        Some(cost) => evidence.with_additional_adapter_cost(cost),
        None => evidence,
    }
}

fn native_host_owns_semantic_text_boundary(kind: WorthUiHostKind) -> bool {
    kind == WorthUiHostKind::Native
}

pub(super) fn map_text_readiness(
    readiness: UiNativeTextPresentationReadiness,
) -> UiHostSurfacePresentationDenial {
    match readiness {
        UiNativeTextPresentationReadiness::SemanticTextDemandDenied(_)
        | UiNativeTextPresentationReadiness::SemanticTextLayoutMismatch
        | UiNativeTextPresentationReadiness::SemanticTextSourceMismatch => {
            UiHostSurfacePresentationDenial::AdapterDeclined
        }
    }
}

#[cfg(test)]
mod tests {
    use super::native_host_owns_semantic_text_boundary;
    use worth_ui_host_contract::WorthUiHostKind;

    #[test]
    fn semantic_text_boundary_is_native_only() {
        assert!(native_host_owns_semantic_text_boundary(
            WorthUiHostKind::Native
        ));
        assert!(!native_host_owns_semantic_text_boundary(
            WorthUiHostKind::Headless
        ));
        assert!(!native_host_owns_semantic_text_boundary(
            WorthUiHostKind::Egui
        ));
        assert!(!native_host_owns_semantic_text_boundary(
            WorthUiHostKind::CapabilityProbeInconclusive
        ));
        assert!(!native_host_owns_semantic_text_boundary(
            WorthUiHostKind::DiagnosticsOnly
        ));
    }
}
