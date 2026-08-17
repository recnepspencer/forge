//! Ordinary runtime coordinator for mounted native text pin transactions.

use crate::mounting::presentation::coordinator::{
    UiMountedTextPinCandidate, UiMountedTextPinState,
};
use worth_ui_host_contract::UiSurfaceBindingGeneration;

use super::{UiNativeTextAtlasTransaction, UiNativeTextPresentationPrepared};

#[derive(Default)]
pub(crate) struct UiNativeMountedTextCoordinator {
    pins: UiMountedTextPinState,
}

pub(crate) struct UiNativeMountedSurfaceTextObservation {
    outcome: worth_ui_host_contract::UiHostSurfacePresentationOutcome,
    pending_candidate: Option<UiMountedTextPinCandidate>,
}

impl UiNativeMountedTextCoordinator {
    pub(crate) fn present_with_mounted_work<'layout>(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        prepared: &'layout UiNativeTextPresentationPrepared,
        resolve: impl Fn(
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
        ) -> Option<&'layout worth_ui_text::UiQualifiedTextLayout>,
        present: impl FnOnce(
            &worth_ui_host_contract::UiMountedTextRasterWork<'_>,
        ) -> worth_ui_host_contract::UiHostSurfacePresentationOutcome,
    ) -> Option<UiNativeMountedSurfaceTextObservation> {
        let candidate = self.pins.candidate(binding, prepared);
        let transition = UiMountedTextPinState::transition_view(&candidate);
        let mut transaction = UiNativeTextAtlasTransaction::prepare(prepared, resolve)?;
        let outcome = transaction.with_mounted_work(
            transition,
            UiMountedTextPinState::binding_pins(&candidate),
            present,
        );
        let pending_candidate = match outcome {
            worth_ui_host_contract::UiHostSurfacePresentationOutcome::Presented(_) => {
                self.pins.commit_presented(candidate);
                None
            }
            worth_ui_host_contract::UiHostSurfacePresentationOutcome::InFlight(_) => {
                Some(candidate)
            }
            worth_ui_host_contract::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(_)
            | worth_ui_host_contract::UiHostSurfacePresentationOutcome::PresentationIndeterminate => {
                None
            }
        };
        Some(UiNativeMountedSurfaceTextObservation {
            outcome,
            pending_candidate,
        })
    }

    pub(crate) fn commit_surface_candidate(&mut self, candidate: UiMountedTextPinCandidate) {
        self.pins.commit_presented(candidate);
    }

    pub(crate) fn deregistration_candidate(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> UiMountedTextPinCandidate {
        self.pins.deregistration_candidate(binding)
    }
}

impl UiNativeMountedSurfaceTextObservation {
    pub(crate) fn into_parts(
        self,
    ) -> (
        worth_ui_host_contract::UiHostSurfacePresentationOutcome,
        Option<UiMountedTextPinCandidate>,
    ) {
        (self.outcome, self.pending_candidate)
    }
}
