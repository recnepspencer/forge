use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::UiNativeWindowConfiguration;

/// Contractual window-opening boundary. It returns only the owned OS window;
/// runtime composition remains responsible for lifecycle settlement.
pub(super) trait UiNativeWindowPort {
    fn open(
        event_loop: &ActiveEventLoop,
        configuration: &UiNativeWindowConfiguration,
    ) -> Result<Arc<Window>, ()>;
}

pub(super) struct UiWinitNativeWindowPort;

impl UiNativeWindowPort for UiWinitNativeWindowPort {
    fn open(
        event_loop: &ActiveEventLoop,
        configuration: &UiNativeWindowConfiguration,
    ) -> Result<Arc<Window>, ()> {
        let [width, height] = configuration.initial_logical_size();
        let attributes = WindowAttributes::default()
            .with_title(configuration.title())
            .with_transparent(true)
            .with_inner_size(LogicalSize::new(f64::from(width), f64::from(height)));
        event_loop
            .create_window(attributes)
            .map(Arc::new)
            .map_err(|_| ())
    }
}
