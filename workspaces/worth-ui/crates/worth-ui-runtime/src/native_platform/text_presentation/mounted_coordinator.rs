//! Ordinary runtime coordinator for mounted native text pin transactions.

use worth_ui_host_contract::{
    UiGlyphRasterPinRequest, UiGlyphRasterTransactionOutcome, UiGlyphRasterTransactionReceipt,
    UiMountedFrameConsumptionView, UiMountedTextPinReleaseRequest, UiSurfaceBindingGeneration,
};

use crate::facade::UiHostEffectPort;
use crate::mounting::presentation::coordinator::{
    UiMountedTextPinCandidate, UiMountedTextPinState,
};

use super::{
    UiNativeTextAtlasTransaction, UiNativeTextPresentationPrepared, UiNativeTextRasterWorkReport,
};

#[derive(Default)]
pub(crate) struct UiNativeMountedTextCoordinator {
    pins: UiMountedTextPinState,
}

pub(crate) struct UiNativeMountedTextPending {
    binding: UiSurfaceBindingGeneration,
    host_pending: worth_ui_host_contract::UiGlyphRasterTransactionPending,
    candidate: UiMountedTextPinCandidate,
    raster_work: UiNativeTextRasterWorkReport,
}

pub(crate) struct UiNativeMountedTextBeginObservation {
    pub(crate) outcome: UiNativeMountedTextOutcome,
    pub(crate) additions: Box<[UiGlyphRasterPinRequest]>,
}

pub(crate) struct UiNativeMountedSurfaceTextObservation {
    outcome: worth_ui_host_contract::UiHostSurfacePresentationOutcome,
    pending_candidate: Option<UiMountedTextPinCandidate>,
}

pub(crate) enum UiNativeMountedTextOutcome {
    Committed {
        receipt: UiGlyphRasterTransactionReceipt,
        raster_work: UiNativeTextRasterWorkReport,
    },
    Pending(UiNativeMountedTextPending),
    RejectedBeforeEffects {
        denial: worth_ui_host_contract::UiGlyphRasterTransactionDenial,
        raster_work: UiNativeTextRasterWorkReport,
    },
    RejectedAfterRasterization {
        denial: worth_ui_host_contract::UiGlyphRasterTransactionDenial,
        raster_work: UiNativeTextRasterWorkReport,
    },
    EffectsIndeterminate {
        _recovery: worth_ui_host_contract::UiGlyphRasterEffectsIndeterminate,
        raster_work: UiNativeTextRasterWorkReport,
    },
}

pub(crate) enum UiNativeMountedTextReleaseOutcome {
    Local,
    Native(UiGlyphRasterTransactionOutcome),
}

impl UiNativeMountedTextOutcome {
    pub(crate) const fn raster_work(&self) -> UiNativeTextRasterWorkReport {
        match self {
            Self::Committed { raster_work, .. }
            | Self::RejectedBeforeEffects { raster_work, .. }
            | Self::RejectedAfterRasterization { raster_work, .. }
            | Self::EffectsIndeterminate { raster_work, .. } => *raster_work,
            Self::Pending(pending) => pending.raster_work,
        }
    }
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

    pub(crate) fn begin<'layout>(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        prepared: &'layout UiNativeTextPresentationPrepared,
        resolve: impl Fn(
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
        ) -> Option<&'layout worth_ui_text::UiQualifiedTextLayout>,
        host: UiHostEffectPort<'_>,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Option<UiNativeMountedTextBeginObservation> {
        let candidate = self.pins.candidate(binding, prepared);
        let additions = UiMountedTextPinState::transition_view(&candidate)
            .additions()
            .to_vec()
            .into_boxed_slice();
        let transaction = UiNativeTextAtlasTransaction::prepare(prepared, resolve)?;
        let observation = transaction.execute(
            host,
            view,
            UiMountedTextPinState::transition_view(&candidate),
        );
        let outcome = self.settle(
            binding,
            candidate,
            observation.outcome,
            observation.raster_work,
        );
        Some(UiNativeMountedTextBeginObservation { outcome, additions })
    }

    pub(crate) fn complete(
        &mut self,
        host: UiHostEffectPort<'_>,
        pending: UiNativeMountedTextPending,
    ) -> UiNativeMountedTextOutcome {
        let outcome = host
            .adapter()
            .complete_mounted_text_raster(host.authority(), pending.host_pending);
        self.settle(
            pending.binding,
            pending.candidate,
            outcome,
            pending.raster_work,
        )
    }

    pub(crate) fn cancel(
        host: UiHostEffectPort<'_>,
        pending: UiNativeMountedTextPending,
    ) -> UiGlyphRasterTransactionOutcome {
        host.adapter()
            .cancel_mounted_text_raster(host.authority(), pending.host_pending)
    }

    pub(crate) fn release(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        host: UiHostEffectPort<'_>,
        request: UiMountedTextPinReleaseRequest,
    ) -> UiNativeMountedTextReleaseOutcome {
        let candidate = self.pins.deregistration_candidate(binding);
        let transition = UiMountedTextPinState::transition_view(&candidate);
        if transition.releases().is_empty() {
            self.pins.commit_presented(candidate);
            return UiNativeMountedTextReleaseOutcome::Local;
        }
        let outcome =
            host.adapter()
                .release_mounted_text_pins(host.authority(), request, transition);
        if matches!(outcome, UiGlyphRasterTransactionOutcome::Committed(_)) {
            self.pins.commit_presented(candidate);
        }
        UiNativeMountedTextReleaseOutcome::Native(outcome)
    }

    fn settle(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        candidate: UiMountedTextPinCandidate,
        outcome: UiGlyphRasterTransactionOutcome,
        raster_work: UiNativeTextRasterWorkReport,
    ) -> UiNativeMountedTextOutcome {
        match outcome {
            UiGlyphRasterTransactionOutcome::Committed(receipt) => {
                self.pins.commit_presented(candidate);
                UiNativeMountedTextOutcome::Committed {
                    receipt,
                    raster_work,
                }
            }
            UiGlyphRasterTransactionOutcome::Pending(host_pending) => {
                UiNativeMountedTextOutcome::Pending(UiNativeMountedTextPending {
                    binding,
                    host_pending,
                    candidate,
                    raster_work,
                })
            }
            UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(denial) => {
                UiNativeMountedTextOutcome::RejectedBeforeEffects {
                    denial,
                    raster_work,
                }
            }
            UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(denial) => {
                UiNativeMountedTextOutcome::RejectedAfterRasterization {
                    denial,
                    raster_work,
                }
            }
            UiGlyphRasterTransactionOutcome::EffectsIndeterminate(recovery) => {
                UiNativeMountedTextOutcome::EffectsIndeterminate {
                    _recovery: recovery,
                    raster_work,
                }
            }
        }
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
