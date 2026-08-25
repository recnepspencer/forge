use super::state::{UiNativeCaptureState, UiNativePendingCapture};

impl UiNativeCaptureState {
    pub(super) fn cancel(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        let Some(pending) = self.pending.remove(&request.identity()) else {
            return worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate;
        };
        if pending.request() != request {
            self.pending.insert(pending.request().identity(), pending);
            return worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate;
        }
        let layout = pending.layout();
        let outcome = match pending {
            UiNativePendingCapture::Admitted { .. } => {
                worth_ui_host_contract::UiHostCaptureCancellationOutcome::CancelledBeforeReadback
            }
            UiNativePendingCapture::Readback {
                layout,
                readback,
                owners,
                ..
            } => {
                self.retain_recovery(layout, readback, owners);
                self.progress_recovery(graphics, resources);
                worth_ui_host_contract::UiHostCaptureCancellationOutcome::ReadbackMayHaveBegun
            }
        };
        if matches!(
            outcome,
            worth_ui_host_contract::UiHostCaptureCancellationOutcome::CancelledBeforeReadback
        ) {
            self.release_bytes(layout);
        }
        outcome
    }

    pub(crate) fn remove_binding(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) {
        self.sources.remove(&binding.diagnostic_value());
        let requests = self
            .pending
            .iter()
            .filter_map(|(identity, pending)| {
                (pending.request().binding() == binding).then_some(*identity)
            })
            .collect::<Vec<_>>();
        for identity in requests {
            if let Some(pending) = self.pending.remove(&identity) {
                match pending {
                    UiNativePendingCapture::Admitted { layout, .. } => self.release_bytes(layout),
                    UiNativePendingCapture::Readback {
                        layout,
                        readback,
                        owners,
                        ..
                    } => self.retain_recovery(layout, readback, owners),
                }
            }
        }
    }

    pub(crate) fn invalidate_source(&mut self, binding: u64) {
        self.sources.remove(&binding);
        let admitted = self
            .pending
            .iter()
            .filter_map(|(identity, pending)| match pending {
                UiNativePendingCapture::Admitted { request, .. }
                    if request.binding().diagnostic_value() == binding =>
                {
                    Some(*identity)
                }
                UiNativePendingCapture::Admitted { .. }
                | UiNativePendingCapture::Readback { .. } => None,
            })
            .collect::<Vec<_>>();
        for identity in admitted {
            if let Some(pending) = self.pending.remove(&identity) {
                self.release_bytes(pending.layout());
            }
        }
    }

    pub(crate) fn invalidate_all_sources(&mut self) {
        self.sources.clear();
        let admitted = self
            .pending
            .iter()
            .filter_map(|(identity, pending)| {
                matches!(pending, UiNativePendingCapture::Admitted { .. }).then_some(*identity)
            })
            .collect::<Vec<_>>();
        for identity in admitted {
            if let Some(pending) = self.pending.remove(&identity) {
                self.release_bytes(pending.layout());
            }
        }
    }

    pub(crate) fn close(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
    ) {
        let pending = std::mem::take(&mut self.pending);
        for (_, capture) in pending {
            match capture {
                UiNativePendingCapture::Admitted { layout, .. } => self.release_bytes(layout),
                UiNativePendingCapture::Readback {
                    layout,
                    readback,
                    owners,
                    ..
                } => self.retain_recovery(layout, readback, owners),
            }
        }
        self.sources.clear();
        self.progress_recovery(graphics, resources);
    }
}
