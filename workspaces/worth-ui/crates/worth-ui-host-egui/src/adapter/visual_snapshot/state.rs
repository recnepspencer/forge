use std::collections::BTreeMap;

#[derive(Default)]
pub(in crate::adapter) struct UiEguiVisualCaptureState {
    presentations:
        BTreeMap<worth_ui_host_contract::UiSurfaceBindingGeneration, UiEguiPresentedSurface>,
    pending: BTreeMap<worth_ui_host_contract::UiHostCaptureRequestIdentity, UiEguiPendingCapture>,
}

#[derive(Clone)]
pub(super) struct UiEguiPresentedSurface {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiEguiScreenshotCorrelation {
    request: worth_ui_host_contract::UiHostCaptureRequestIdentity,
}

struct UiEguiPendingCapture {
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
}

pub(super) enum UiEguiPendingAdmission {
    Admitted(UiEguiScreenshotCorrelation),
    AlreadyPending,
    CapacityExceeded,
}

pub(super) enum UiEguiPresentationAffinity {
    Exact(UiEguiPresentedSurface),
    Superseded,
}

impl UiEguiVisualCaptureState {
    pub(super) fn record_presentation(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        presentation: UiEguiPresentedSurface,
    ) {
        self.presentations.insert(binding, presentation);
    }

    pub(super) fn remove_binding(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) {
        self.presentations.remove(&binding);
        self.pending
            .retain(|_, pending| pending.request.binding() != binding);
    }

    pub(super) fn exact_presentation(
        &self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> Option<UiEguiPresentedSurface> {
        self.presentations
            .get(&request.binding())
            .filter(|presentation| presentation.matches(request))
            .cloned()
    }

    pub(super) fn presentation_affinity(
        &mut self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> UiEguiPresentationAffinity {
        match self.exact_presentation(request) {
            Some(presented) => UiEguiPresentationAffinity::Exact(presented),
            None => {
                self.cancel(request);
                UiEguiPresentationAffinity::Superseded
            }
        }
    }

    pub(super) fn admit_pending(
        &mut self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> UiEguiPendingAdmission {
        if self.pending.contains_key(&request.identity()) {
            return UiEguiPendingAdmission::AlreadyPending;
        }
        if self
            .pending
            .values()
            .any(|pending| pending.request.binding() == request.binding())
        {
            return UiEguiPendingAdmission::CapacityExceeded;
        }
        self.pending
            .insert(request.identity(), UiEguiPendingCapture { request });
        UiEguiPendingAdmission::Admitted(UiEguiScreenshotCorrelation {
            request: request.identity(),
        })
    }

    pub(super) fn finish_if_exact(
        &mut self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> bool {
        if self.pending.remove(&request.identity()).is_none() {
            return false;
        }
        self.exact_presentation(request).is_some()
    }

    pub(super) fn cancel(
        &mut self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> bool {
        self.pending.remove(&request.identity()).is_some()
    }
}

impl UiEguiPresentedSurface {
    pub(super) fn from_view(
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        epoch: worth_ui_host_contract::UiHostPresentationEpoch,
        regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
    ) -> Self {
        Self {
            frame: view.projection().frame(),
            attempt: view.attempt(),
            host_surface: view.requirement().host_surface(),
            binding: view.requirement().binding(),
            epoch,
            regions,
        }
    }

    pub(super) fn regions(&self) -> Vec<worth_ui_host_contract::UiHostRealizedRegion> {
        self.regions.clone()
    }

    fn matches(&self, request: worth_ui_host_contract::UiHostVisualCaptureRequest) -> bool {
        self.frame == request.frame()
            && self.attempt == request.presentation_attempt()
            && self.host_surface == request.host_surface()
            && self.binding == request.binding()
            && self.epoch == request.expected_epoch()
    }
}

impl UiEguiScreenshotCorrelation {
    pub(super) const fn request(self) -> worth_ui_host_contract::UiHostCaptureRequestIdentity {
        self.request
    }
}
