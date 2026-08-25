use std::collections::VecDeque;

use super::port::{
    UiNativeCaptureReadbackPoll, UiNativeCaptureReadbackPort, UiNativePendingCaptureReadback,
};
use super::readback::UiNativeReadbackLayout;
use super::source::{UiNativeCaptureSource, UiNativeCaptureSourceInput};
use super::UiNativeCaptureState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeCaptureExternalObservation {
    Pending,
    CapturedRgba8([u8; 4]),
    ArtifactIndeterminate,
    PhysicalCompletionIndeterminate,
}

pub struct UiNativeCaptureProtocolWorld {
    captures: UiNativeCaptureState,
    resources: crate::native::UiNativeResourceRegistry,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
}

struct UiScriptedNativeCapturePort {
    observations: VecDeque<UiNativeCaptureExternalObservation>,
}

struct UiScriptedNativePendingCapture {
    observations: VecDeque<UiNativeCaptureExternalObservation>,
    dimensions: [u32; 2],
}

impl UiNativeCaptureProtocolWorld {
    pub fn new(observations: impl IntoIterator<Item = UiNativeCaptureExternalObservation>) -> Self {
        let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound()
            .expect("certification frame identity");
        let attempt = worth_ui_host_contract::UiMountedPresentationAttemptIdentity::mint_unbound()
            .expect("certification attempt identity");
        let host_surface = worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound()
            .expect("certification host surface");
        let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound()
            .expect("certification binding");
        let epoch = worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
            attempt.diagnostic_value(),
        );
        let transform = worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
            worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host([11, 13], [2, 1]),
            worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
                [2.0, 1.0],
                [1.0, 1.0],
                [0.0, 0.0],
            ),
            worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
                worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
                worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
            ),
        );
        let port = UiScriptedNativeCapturePort {
            observations: observations.into_iter().collect(),
        };
        let mut captures = UiNativeCaptureState::with_port(Box::new(port));
        captures.record_source(
            binding,
            UiNativeCaptureSource::completed(UiNativeCaptureSourceInput {
                host_session: 7,
                frame,
                attempt,
                host_surface,
                binding,
                epoch,
                transform: Some(transform),
                regions: Vec::new(),
            }),
        );
        let request = worth_ui_host_contract::UiHostVisualCaptureRequest::admitted_by_runtime(
            worth_ui_host_contract::UiHostCaptureRequestIdentity::issued_by_runtime(1),
            worth_ui_host_contract::UiHostCaptureFrameAffinity::observed_by_runtime(frame, attempt),
            worth_ui_host_contract::UiHostCaptureSurfaceAffinity::observed_by_runtime(
                7,
                host_surface,
                binding,
                epoch,
            ),
            worth_ui_host_contract::UiHostCaptureArtifactBudget::admitted_by_runtime(true, 8),
        );
        Self {
            captures,
            resources: crate::native::UiNativeResourceRegistry::new(),
            request,
        }
    }

    pub fn observe(&mut self) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        self.captures
            .observe(None, &mut self.resources, self.request)
    }

    pub fn cancel(&mut self) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        self.captures
            .cancel(None, &mut self.resources, self.request)
    }

    pub fn current_census(&self) -> crate::UiNativeResourceCensus {
        self.resources.current()
    }

    pub fn close(mut self) -> crate::UiNativeResourceCensus {
        self.captures.close(None, &mut self.resources);
        self.resources.current()
    }
}

impl UiNativeCaptureReadbackPort for UiScriptedNativeCapturePort {
    fn begin(
        &mut self,
        _graphics: Option<&crate::native::UiNativePresentationAccess>,
        layout: UiNativeReadbackLayout,
    ) -> Result<Box<dyn UiNativePendingCaptureReadback>, ()> {
        Ok(Box::new(UiScriptedNativePendingCapture {
            observations: std::mem::take(&mut self.observations),
            dimensions: layout.dimensions(),
        }))
    }
}

impl UiNativePendingCaptureReadback for UiScriptedNativePendingCapture {
    fn poll(
        mut self: Box<Self>,
        _graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll {
        match self.observations.pop_front() {
            Some(UiNativeCaptureExternalObservation::Pending) => {
                UiNativeCaptureReadbackPoll::Pending(self)
            }
            Some(UiNativeCaptureExternalObservation::CapturedRgba8(rgba)) => {
                let pixels = usize::try_from(self.dimensions[0])
                    .ok()
                    .and_then(|width| {
                        usize::try_from(self.dimensions[1])
                            .ok()
                            .and_then(|height| width.checked_mul(height))
                    })
                    .unwrap_or(0);
                UiNativeCaptureReadbackPoll::Captured(
                    std::iter::repeat_n(rgba, pixels)
                        .flatten()
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                )
            }
            Some(UiNativeCaptureExternalObservation::ArtifactIndeterminate) => {
                UiNativeCaptureReadbackPoll::ArtifactIndeterminate
            }
            Some(UiNativeCaptureExternalObservation::PhysicalCompletionIndeterminate) | None => {
                UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(self)
            }
        }
    }

    fn poll_recovery(
        self: Box<Self>,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll {
        self.poll(graphics)
    }
}
