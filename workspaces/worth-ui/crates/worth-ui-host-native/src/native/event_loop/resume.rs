use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;

use super::{
    callback_thread, directive, pointer_position, window_port, UiNativeEventLoopApplication,
    UiNativeEventLoopClient, UiNativeEventLoopRunDenial, UiNativeOwnedWindow,
    UiNativeReadinessGrant,
};
use crate::native::{UiNativeGraphicsPort, UiNativeOwnedGraphics, UiWgpuNativeGraphicsPort};
use window_port::UiNativeWindowPort;

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn resume_admitted(
        &mut self,
        event_loop: &ActiveEventLoop,
        _admission: callback_thread::UiNativeEventLoopThreadObservation,
    ) {
        if self.shared.borrow().window.is_some() {
            return;
        }
        let (window, pointer_input) = match self.open_registered_window(event_loop) {
            Ok(opened) => opened,
            Err(denial) => return self.fail(event_loop, denial),
        };
        let graphics = match self.prepare_registered_graphics(&window) {
            Ok(graphics) => graphics,
            Err(denial) => {
                window.close(&mut self.shared.borrow_mut().resources);
                return self.fail(event_loop, denial);
            }
        };
        self.install_surface(window, graphics, pointer_input);
        let Some(directive) = self.surface_ready_directive() else {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
        };
        if directive::apply(&self.shared, event_loop, directive) {
            return event_loop.exit();
        }
        self.commit_readiness(event_loop);
    }

    fn open_registered_window(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<
        (
            UiNativeOwnedWindow,
            Option<Box<pointer_position::UiNativePointerInputPort>>,
        ),
        UiNativeEventLoopRunDenial,
    > {
        if !self.shared.borrow().resources.admits(6) {
            return Err(UiNativeEventLoopRunDenial::IncompleteCleanup);
        }
        let opened = window_port::UiWinitNativeWindowPort::open(event_loop, &self.configuration)
            .map_err(|_| UiNativeEventLoopRunDenial::WindowCreation)?;
        let (window, crossings) = opened
            .register(&mut self.shared.borrow_mut().resources)
            .map_err(|()| UiNativeEventLoopRunDenial::IncompleteCleanup)?;
        self.port_crossings = self.port_crossings.saturating_add(crossings);
        let pointer = install_pointer(&window).ok_or(UiNativeEventLoopRunDenial::WindowCreation)?;
        Ok((window, pointer))
    }

    fn prepare_registered_graphics(
        &mut self,
        window: &UiNativeOwnedWindow,
    ) -> Result<UiNativeOwnedGraphics, UiNativeEventLoopRunDenial> {
        let prepared = UiWgpuNativeGraphicsPort::prepare(Arc::clone(window))
            .map_err(|_| UiNativeEventLoopRunDenial::GraphicsPreparation)?;
        let (graphics, crossings) = prepared.into_parts();
        self.port_crossings = self.port_crossings.saturating_add(crossings);
        UiNativeOwnedGraphics::register(graphics, &mut self.shared.borrow_mut().resources).map_err(
            |graphics| {
                drop(graphics);
                UiNativeEventLoopRunDenial::IncompleteCleanup
            },
        )
    }

    fn install_surface(
        &mut self,
        window: UiNativeOwnedWindow,
        graphics: UiNativeOwnedGraphics,
        pointer_input: Option<Box<pointer_position::UiNativePointerInputPort>>,
    ) {
        let profile = (graphics.scale_factor, graphics.extent());
        let mut state = self.shared.borrow_mut();
        state.graphics = Some(graphics);
        state.window = Some(window);
        state
            .lifecycle_protocol
            .install_initial_profile(profile.0, profile.1);
        self.pointer_input = pointer_input;
    }

    fn surface_ready_directive(&mut self) -> Option<super::UiNativeEventLoopDirective> {
        let preparation = self.shared.borrow().graphics.as_ref().map(|graphics| {
            UiNativeReadinessGrant::issued(
                0,
                (graphics.scale_factor * 1_000.0).round() as u32,
                graphics.extent(),
            )
        })?;
        self.client.as_mut()?.native_surface_ready(preparation).ok()
    }
}

#[cfg(target_os = "windows")]
fn install_pointer(
    window: &UiNativeOwnedWindow,
) -> Option<Option<Box<pointer_position::UiNativePointerInputPort>>> {
    pointer_position::install_pointer_input(window).map(Some)
}

#[cfg(not(target_os = "windows"))]
fn install_pointer(
    _window: &UiNativeOwnedWindow,
) -> Option<Option<Box<pointer_position::UiNativePointerInputPort>>> {
    Some(None)
}
