use worth_ui_host_contract::{UiHostSurfacePresentationDenial, UiHostSurfacePresentationOutcome};

use crate::native::{presentation::UiNativePresentationFailure, UiNativeHostState};

pub(in crate::native::mechanics_adapter) fn mark_presentation_indeterminate(
    state: &mut UiNativeHostState,
) -> UiHostSurfacePresentationOutcome {
    state.lifecycle.record_presentation_indeterminate();
    UiHostSurfacePresentationOutcome::PresentationIndeterminate
}

pub(super) fn malformed() -> UiHostSurfacePresentationOutcome {
    UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
        UiHostSurfacePresentationDenial::MalformedProjection,
    )
}

pub(in crate::native::mechanics_adapter) fn adapter_declined() -> UiHostSurfacePresentationOutcome {
    UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
        UiHostSurfacePresentationDenial::AdapterDeclined,
    )
}

pub(super) fn before_effects_malformed() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(UiHostSurfacePresentationDenial::MalformedProjection)
}

pub(super) fn before_effects_declined() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(UiHostSurfacePresentationDenial::AdapterDeclined)
}
