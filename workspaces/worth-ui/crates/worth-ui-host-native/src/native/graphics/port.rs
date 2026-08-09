use std::sync::Arc;

use winit::window::Window;

use super::UiNativeGraphics;

/// Contractual device/surface preparation boundary. The wgpu implementation
/// returns observations and owned mechanics, never a framework lifecycle
/// verdict.
pub(crate) trait UiNativeGraphicsPort {
    fn prepare(window: Arc<Window>) -> Result<UiNativeGraphics, ()>;
}

pub(crate) struct UiWgpuNativeGraphicsPort;

impl UiNativeGraphicsPort for UiWgpuNativeGraphicsPort {
    fn prepare(window: Arc<Window>) -> Result<UiNativeGraphics, ()> {
        UiNativeGraphics::prepare(window).map_err(|_| ())
    }
}
