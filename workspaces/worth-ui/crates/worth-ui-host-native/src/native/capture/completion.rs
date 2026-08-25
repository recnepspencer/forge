use super::port::UiNativeCaptureReadbackPoll;
use super::readback::UiNativeReadbackLayout;
use super::source::UiNativeCaptureSource;
use super::state::{release_owners, UiNativeCaptureState, UiNativePendingCapture};

impl UiNativeCaptureState {
    pub(super) fn progress_pending(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
        pending: UiNativePendingCapture,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        if pending.request() != request {
            self.pending.insert(pending.request().identity(), pending);
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
        }
        match pending {
            UiNativePendingCapture::Admitted {
                request,
                source,
                layout,
            } => self.begin_readback(graphics, resources, request, source, layout),
            UiNativePendingCapture::Readback {
                request,
                source,
                layout,
                readback,
                owners,
            } => self.poll_readback(
                graphics, resources, request, source, layout, readback, owners,
            ),
        }
    }

    fn poll_readback(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
        source: UiNativeCaptureSource,
        layout: UiNativeReadbackLayout,
        readback: Box<dyn super::port::UiNativePendingCaptureReadback>,
        owners: Vec<crate::native::UiNativeResourceOwner>,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        match readback.poll(graphics) {
            UiNativeCaptureReadbackPoll::Pending(readback) => {
                self.pending.insert(
                    request.identity(),
                    UiNativePendingCapture::Readback {
                        request,
                        source,
                        layout,
                        readback,
                        owners,
                    },
                );
                worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
            }
            UiNativeCaptureReadbackPoll::Captured(bytes) => {
                release_owners(resources, owners);
                self.release_bytes(layout);
                if bytes.len() != layout.canonical_byte_len() {
                    return worth_ui_host_contract::UiHostCaptureObservationOutcome::ReadbackCompletionIndeterminate;
                }
                let pixels = worth_ui_host_contract::UiHostPixelArtifact::copied_by_host(
                    layout.dimensions(),
                    layout.tight_row_bytes(),
                    bytes,
                    worth_ui_host_contract::UiHostPixelColorSpace::Srgb,
                );
                captured(request, source, Some(pixels))
            }
            UiNativeCaptureReadbackPoll::ArtifactIndeterminate => {
                release_owners(resources, owners);
                self.release_bytes(layout);
                worth_ui_host_contract::UiHostCaptureObservationOutcome::ReadbackCompletionIndeterminate
            }
            UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(readback) => {
                self.retain_recovery(layout, readback, owners);
                worth_ui_host_contract::UiHostCaptureObservationOutcome::ReadbackCompletionIndeterminate
            }
        }
    }
}

pub(super) fn captured(
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    source: UiNativeCaptureSource,
    pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    let Some(transform) = source.transform() else {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
    };
    worth_ui_host_contract::UiHostCaptureObservationOutcome::Captured(
        worth_ui_host_contract::UiHostCaptureObservation::observed_by_host(
            worth_ui_host_contract::UiHostCaptureAffinity::observed_by_host(
                request.identity(),
                source.epoch(),
            ),
            transform,
            source.regions(),
            pixels,
        ),
    )
}
