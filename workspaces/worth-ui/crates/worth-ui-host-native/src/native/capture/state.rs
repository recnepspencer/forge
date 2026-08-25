use std::collections::BTreeMap;

use super::port::{
    UiNativeCaptureReadbackPort, UiNativePendingCaptureReadback, UiWgpuNativeCaptureReadbackPort,
};
use super::readback::UiNativeReadbackLayout;
use super::source::UiNativeCaptureSource;

pub(crate) struct UiNativeCaptureState {
    pub(super) sources: BTreeMap<u64, UiNativeCaptureSource>,
    pub(super) pending:
        BTreeMap<worth_ui_host_contract::UiHostCaptureRequestIdentity, UiNativePendingCapture>,
    pub(super) recovering: Vec<UiNativeRecoveringCapture>,
    pub(super) reserved_bytes: u64,
    pub(super) port: Box<dyn UiNativeCaptureReadbackPort>,
}

pub(super) struct UiNativeRecoveringCapture {
    pub(super) layout: UiNativeReadbackLayout,
    pub(super) readback: Box<dyn UiNativePendingCaptureReadback>,
    pub(super) owners: Vec<crate::native::UiNativeResourceOwner>,
}

pub(super) enum UiNativePendingCapture {
    Admitted {
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
        source: UiNativeCaptureSource,
        layout: UiNativeReadbackLayout,
    },
    Readback {
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
        source: UiNativeCaptureSource,
        layout: UiNativeReadbackLayout,
        readback: Box<dyn UiNativePendingCaptureReadback>,
        owners: Vec<crate::native::UiNativeResourceOwner>,
    },
}

impl Default for UiNativeCaptureState {
    fn default() -> Self {
        Self {
            sources: BTreeMap::new(),
            pending: BTreeMap::new(),
            recovering: Vec::new(),
            reserved_bytes: 0,
            port: Box::new(UiWgpuNativeCaptureReadbackPort),
        }
    }
}

impl UiNativeCaptureState {
    #[cfg(any(test, feature = "certification-support"))]
    pub(super) fn with_port(port: Box<dyn UiNativeCaptureReadbackPort>) -> Self {
        Self {
            sources: BTreeMap::new(),
            pending: BTreeMap::new(),
            recovering: Vec::new(),
            reserved_bytes: 0,
            port,
        }
    }

    pub(super) fn occupied_slots(&self) -> usize {
        self.pending.len() + self.recovering.len()
    }

    pub(crate) fn is_settled(&self) -> bool {
        self.pending.is_empty() && self.recovering.is_empty()
    }

    pub(super) fn record_source(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        source: UiNativeCaptureSource,
    ) {
        self.sources.insert(binding.diagnostic_value(), source);
    }

    pub(super) fn release_bytes(&mut self, layout: UiNativeReadbackLayout) {
        self.reserved_bytes = self
            .reserved_bytes
            .checked_sub(layout.allocation_bytes())
            .expect("one pending capture retains its exact byte reservation");
    }
}

impl UiNativePendingCapture {
    pub(super) const fn request(&self) -> worth_ui_host_contract::UiHostVisualCaptureRequest {
        match self {
            Self::Admitted { request, .. } | Self::Readback { request, .. } => *request,
        }
    }

    pub(super) const fn layout(&self) -> UiNativeReadbackLayout {
        match self {
            Self::Admitted { layout, .. } | Self::Readback { layout, .. } => *layout,
        }
    }
}

pub(super) fn release_owners(
    resources: &mut crate::native::UiNativeResourceRegistry,
    owners: Vec<crate::native::UiNativeResourceOwner>,
) {
    resources
        .release_all(owners)
        .expect("capture readback owners remain exact until terminal settlement");
}
