use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationOutcome,
    UiMountedSurfaceBindingRequirement,
};

use super::presentation_attempt::{UiMountedPresentationProgress, UiMountedPresentationStart};
use crate::native_platform::text_presentation::{
    prepare_mounted_semantic_text, UiMountedEventTimeDpiAuthority, UiNativeMountedTextCoordinator,
    UiNativeTextPresentationPreparation,
};

pub(super) fn present(
    start: &UiMountedPresentationStart<'_, '_>,
    requirement: UiMountedSurfaceBindingRequirement,
    presentation_work: &super::super::UiMountedPresentationWork,
    progress: &mut UiMountedPresentationProgress,
    text: &mut UiNativeMountedTextCoordinator,
) -> Option<UiHostSurfacePresentationOutcome> {
    let Some(dpi) = UiMountedEventTimeDpiAuthority::from_requirement(requirement) else {
        record_rejection(progress, requirement, UiHostSurfacePresentationDenial::AdapterDeclined);
        return None;
    };
    let preparation = prepare_mounted_semantic_text(presentation_work.view(), dpi, |identity| {
        presentation_work.resolve_layout(identity)
    })?;
    let prepared = match preparation {
        UiNativeTextPresentationPreparation::Prepared(prepared) => prepared,
        UiNativeTextPresentationPreparation::Denied(denial) => {
            record_rejection(
                progress,
                requirement,
                super::presentation_attempt::map_text_readiness(denial.readiness()),
            );
            return None;
        }
    };
    text.present_with_mounted_work(
        requirement.binding(),
        &prepared,
        |identity| presentation_work.resolve_layout(identity),
        |text_raster_work| {
            let view = start.authority.bind(
                super::super::consumption_view::UiRuntimeMountedFrameConsumptionInput {
                    attempt: start.attempt,
                    deadline: start.deadline,
                    requirement,
                    presentation_work,
                    text_raster_work: Some(text_raster_work),
                },
            );
            start
                .host
                .adapter()
                .present_mounted_surface(start.host.authority(), &view)
        },
    )
}

fn record_rejection(
    progress: &mut UiMountedPresentationProgress,
    requirement: UiMountedSurfaceBindingRequirement,
    denial: UiHostSurfacePresentationDenial,
) {
    progress
        .rejected
        .push(super::super::UiMountedSurfacePresentationRejection::new(
            requirement.binding(),
            denial,
        ));
}
