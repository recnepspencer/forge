use std::cell::RefCell;
use std::rc::Rc;

use crate::native::{UiNativeHostState, WorthUiNativeEventLoop, WorthUiNativeMechanicsAdapter};

/// Effect-free qualified native mechanics preparation.
///
/// Preparing this value allocates no event loop, window, surface, adapter,
/// device, queue, or presentation target. `into_parts` consumes it into the
/// one adapter/driver pair that shares the later live native state.
pub struct WorthUiPreparedNativeHost {
    state: Rc<RefCell<UiNativeHostState>>,
    profile: super::UiNativePlatformProfileIdentity,
}

impl WorthUiPreparedNativeHost {
    pub fn prepare_qualified() -> Self {
        Self {
            state: Rc::new(RefCell::new(UiNativeHostState::new())),
            profile: super::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1,
        }
    }

    /// Consume the effect-free preparation into its exact mechanics adapter
    /// and event-loop owner. Neither part is independently constructible.
    pub fn into_parts(
        self,
        window: UiNativeWindowConfiguration,
    ) -> (WorthUiNativeMechanicsAdapter, WorthUiNativeEventLoop) {
        debug_assert_eq!(
            self.profile,
            super::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1
        );
        (
            WorthUiNativeMechanicsAdapter::from_preparation(Rc::clone(&self.state), self.profile),
            WorthUiNativeEventLoop::from_preparation(self.state, window),
        )
    }
}

/// Vendor-free window configuration consumed by the native event-loop owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeWindowConfiguration {
    title: Box<str>,
    initial_logical_size: [u32; 2],
}

impl UiNativeWindowConfiguration {
    pub fn qualified(title: impl Into<Box<str>>, initial_logical_size: [u32; 2]) -> Self {
        Self {
            title: title.into(),
            initial_logical_size,
        }
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) const fn initial_logical_size(&self) -> [u32; 2] {
        self.initial_logical_size
    }
}
