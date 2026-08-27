use super::readback::UiNativeReadbackLayout;
use super::source::UiNativeCaptureSource;
use super::state::{release_owners, UiNativeCaptureState, UiNativePendingCapture};

const SLOT_CAPACITY: usize = crate::UiNativeMechanicsCapacities::QUALIFIED.readback_slots as usize;
const BYTE_CAPACITY: u64 = crate::UiNativeMechanicsCapacities::QUALIFIED.readback_bytes as u64;

impl UiNativeCaptureState {
    pub(super) fn observe(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        self.progress_recovery(graphics, resources);
        if let Some(pending) = self.pending.remove(&request.identity()) {
            return self.progress_pending(graphics, resources, request, pending);
        }
        let Some(source) = self
            .sources
            .get(&request.binding().diagnostic_value())
            .filter(|source| source.matches(request))
            .cloned()
        else {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback;
        };
        let Some(transform) = source.transform() else {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
        };
        if !request.pixels_requested() {
            return super::completion::captured(request, source, None);
        }
        let Some(layout) = UiNativeReadbackLayout::bounded(
            transform.client_physical_dimensions(),
            request.maximum_pixel_bytes(),
        ) else {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded;
        };
        if self.occupied_slots() >= SLOT_CAPACITY
            || self
                .reserved_bytes
                .checked_add(layout.allocation_bytes())
                .is_none_or(|bytes| bytes > BYTE_CAPACITY)
        {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded;
        }
        self.reserved_bytes += layout.allocation_bytes();
        self.pending.insert(
            request.identity(),
            UiNativePendingCapture::Admitted {
                request,
                source,
                layout,
            },
        );
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    }

    pub(super) fn begin_readback(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
        source: UiNativeCaptureSource,
        layout: UiNativeReadbackLayout,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        let still_exact = self
            .sources
            .get(&request.binding().diagnostic_value())
            .is_some_and(|current| current.matches(request));
        if !still_exact {
            self.release_bytes(layout);
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback;
        }
        let owners = match resources.reserve(&[
            crate::native::UiNativeResourceClass::ReadbackBuffer,
            crate::native::UiNativeResourceClass::PendingSubmission,
        ]) {
            Ok(owners) => owners,
            Err(()) => {
                self.release_bytes(layout);
                return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded;
            }
        };
        let readback = match self.port.begin(graphics, layout) {
            Ok(readback) => readback,
            Err(()) => {
                release_owners(resources, owners);
                self.release_bytes(layout);
                return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
            }
        };
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
}
