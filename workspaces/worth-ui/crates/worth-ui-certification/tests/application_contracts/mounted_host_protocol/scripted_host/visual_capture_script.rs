use super::ScriptedPresentationHost;

pub(super) enum ScriptedVisualAffinity {
    Exact,
    WrongRequest,
    WrongEpoch,
}

pub(super) enum ScriptedVisualTerminal {
    SupersededBeforeReadback,
    CaptureAffinityIndeterminate,
    Unsupported,
    CapacityExceeded,
}

pub(super) enum ScriptedVisualCapture {
    Pending,
    Captured {
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
        affinity: ScriptedVisualAffinity,
    },
    Terminal(ScriptedVisualTerminal),
}

pub(super) fn observe(
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    script: ScriptedVisualCapture,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    match script {
        ScriptedVisualCapture::Pending => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        }
        ScriptedVisualCapture::Captured {
            transform,
            regions,
            pixels,
            affinity,
        } => worth_ui_host_contract::UiHostCaptureObservationOutcome::Captured(
            worth_ui_host_contract::UiHostCaptureObservation::observed_by_host(
                capture_affinity(request, affinity),
                transform,
                regions,
                pixels,
            ),
        ),
        ScriptedVisualCapture::Terminal(terminal) => terminal_outcome(terminal),
    }
}

impl ScriptedPresentationHost {
    pub(crate) fn set_visual_capture_capability(
        &self,
        capability: worth_ui_host_contract::UiHostCaptureCapability,
    ) {
        self.state.lock().unwrap().visual_capture_capability = capability;
    }

    pub(crate) fn push_visual_capture_pending(&self) {
        self.push_visual_script(ScriptedVisualCapture::Pending);
    }

    pub(crate) fn push_visual_capture(
        &self,
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
    ) {
        self.push_visual_capture_observation(
            transform,
            Vec::new(),
            pixels,
            ScriptedVisualAffinity::Exact,
        );
    }

    pub(crate) fn push_visual_capture_with_regions(
        &self,
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
    ) {
        self.push_visual_capture_observation(
            transform,
            regions,
            pixels,
            ScriptedVisualAffinity::Exact,
        );
    }

    pub(crate) fn push_visual_capture_with_wrong_request(
        &self,
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
    ) {
        self.push_visual_capture_observation(
            transform,
            Vec::new(),
            pixels,
            ScriptedVisualAffinity::WrongRequest,
        );
    }

    pub(crate) fn push_visual_capture_with_wrong_epoch(
        &self,
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
    ) {
        self.push_visual_capture_observation(
            transform,
            Vec::new(),
            pixels,
            ScriptedVisualAffinity::WrongEpoch,
        );
    }

    pub(crate) fn push_visual_capture_superseded(&self) {
        self.push_visual_terminal(ScriptedVisualTerminal::SupersededBeforeReadback);
    }

    pub(crate) fn push_visual_capture_affinity_indeterminate(&self) {
        self.push_visual_terminal(ScriptedVisualTerminal::CaptureAffinityIndeterminate);
    }

    pub(crate) fn push_visual_capture_unsupported(&self) {
        self.push_visual_terminal(ScriptedVisualTerminal::Unsupported);
    }

    pub(crate) fn push_visual_capture_capacity_exceeded(&self) {
        self.push_visual_terminal(ScriptedVisualTerminal::CapacityExceeded);
    }

    pub(crate) fn visual_capture_calls(
        &self,
    ) -> Vec<worth_ui_host_contract::UiHostVisualCaptureRequest> {
        self.state.lock().unwrap().visual_capture_calls.clone()
    }

    pub(crate) fn set_visual_cancellation_outcome(
        &self,
        outcome: worth_ui_host_contract::UiHostCaptureCancellationOutcome,
    ) {
        self.state.lock().unwrap().visual_cancellation_outcome = outcome;
    }

    pub(crate) fn visual_cancellation_calls(
        &self,
    ) -> Vec<worth_ui_host_contract::UiHostVisualCaptureRequest> {
        self.state.lock().unwrap().visual_cancellation_calls.clone()
    }

    fn push_visual_capture_observation(
        &self,
        transform: worth_ui_host_contract::UiHostCoordinateTransform,
        regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
        pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
        affinity: ScriptedVisualAffinity,
    ) {
        self.push_visual_script(ScriptedVisualCapture::Captured {
            transform,
            regions,
            pixels,
            affinity,
        });
    }

    fn push_visual_terminal(&self, terminal: ScriptedVisualTerminal) {
        self.push_visual_script(ScriptedVisualCapture::Terminal(terminal));
    }

    fn push_visual_script(&self, script: ScriptedVisualCapture) {
        self.state.lock().unwrap().visual_captures.push_back(script);
    }
}

fn terminal_outcome(
    terminal: ScriptedVisualTerminal,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    match terminal {
        ScriptedVisualTerminal::SupersededBeforeReadback => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback
        }
        ScriptedVisualTerminal::CaptureAffinityIndeterminate => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate
        }
        ScriptedVisualTerminal::Unsupported => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported
        }
        ScriptedVisualTerminal::CapacityExceeded => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded
        }
    }
}

fn capture_affinity(
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    affinity: ScriptedVisualAffinity,
) -> worth_ui_host_contract::UiHostCaptureAffinity {
    let request_identity = match affinity {
        ScriptedVisualAffinity::Exact | ScriptedVisualAffinity::WrongEpoch => request.identity(),
        ScriptedVisualAffinity::WrongRequest => {
            worth_ui_host_contract::UiHostCaptureRequestIdentity::issued_by_runtime(
                request.identity().diagnostic_value().wrapping_add(1),
            )
        }
    };
    let epoch = match affinity {
        ScriptedVisualAffinity::Exact | ScriptedVisualAffinity::WrongRequest => {
            request.expected_epoch()
        }
        ScriptedVisualAffinity::WrongEpoch => {
            worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
                request.expected_epoch().diagnostic_value().wrapping_add(1),
            )
        }
    };
    worth_ui_host_contract::UiHostCaptureAffinity::observed_by_host(request_identity, epoch)
}
