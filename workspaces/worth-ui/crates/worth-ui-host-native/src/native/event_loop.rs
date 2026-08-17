use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::WindowId;

use crate::UiNativeWindowConfiguration;

use super::{UiNativeGraphicsPort, UiNativeHostState, UiWgpuNativeGraphicsPort};

mod cleanup;
mod contract;
mod finish;
mod physical_clock;
mod physical_progression;
mod run;
mod terminal_cleanup;
#[cfg(test)]
mod tests;
mod window_port;

#[cfg(test)]
use run::stop_before_callbacks;

pub use cleanup::UiNativeEventLoopCleanup;
pub use contract::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopClient, UiNativeEventLoopClientCleanup,
    UiNativeEventLoopClientClose, UiNativeEventLoopDirective, UiNativeEventLoopRunDenial,
    UiNativeEventLoopRunReport, UiNativeEventLoopStopReport, UiNativePhysicalProgressGrant,
    UiNativeReadinessGrant,
};
use physical_clock::UiNativePhysicalEventClock;
use terminal_cleanup::terminal_cleanup_complete;
pub(crate) use window_port::UiNativeOwnedWindow;
use window_port::{UiNativeWindowPort, UiWinitNativeWindowPort};

pub struct WorthUiNativeEventLoop {
    state: Rc<RefCell<UiNativeHostState>>,
    window: UiNativeWindowConfiguration,
}

struct UiNativeEventLoopApplication<Client> {
    shared: Rc<RefCell<UiNativeHostState>>,
    configuration: UiNativeWindowConfiguration,
    client: Option<Client>,
    first_frame_presented: bool,
    readiness: super::UiNativeReadinessRegistry,
    readiness_owner: super::UiNativeReadyOwner,
    physical_readiness_owner: super::UiNativeReadyOwner,
    readiness_signals: u64,
    redraw_turns: u64,
    idle_wait_turns: u64,
    coalesced_wakes: u64,
    failure: Option<UiNativeEventLoopRunDenial>,
    run_thread: std::thread::ThreadId,
    thread_observation: Option<UiNativeEventLoopThreadObservation>,
    loop_resources: Vec<super::UiNativeResourceOwner>,
    port_crossings: u8,
    physical_clock: UiNativePhysicalEventClock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiNativeEventLoopThreadObservation {
    thread: std::thread::ThreadId,
    matches_launch: bool,
}

impl WorthUiNativeEventLoop {
    pub(crate) fn from_preparation(
        state: Rc<RefCell<UiNativeHostState>>,
        window: UiNativeWindowConfiguration,
    ) -> Self {
        Self { state, window }
    }
}

impl<Client: UiNativeEventLoopClient> ApplicationHandler for UiNativeEventLoopApplication<Client> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        self.advance_physical_signal_clock(event_loop);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let callback_thread = std::thread::current().id();
        let admission = transition_callback_thread(
            &mut self.thread_observation,
            self.run_thread,
            callback_thread,
        );
        let Ok(admission) = admission else {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
        };
        self.resume_admitted(event_loop, admission);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self
            .shared
            .borrow()
            .window
            .as_ref()
            .is_none_or(|window| window.id() != window_id)
        {
            return;
        }
        match event {
            WindowEvent::RedrawRequested => self.redraw(event_loop),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                let replacement = {
                    let mut shared = self.shared.borrow_mut();
                    let UiNativeHostState {
                        graphics,
                        resources,
                        ..
                    } = &mut *shared;
                    graphics.as_mut().map_or(Ok(false), |graphics| {
                        graphics.resize([size.width, size.height], resources)
                    })
                };
                match replacement {
                    Ok(true) => self.commit_readiness(event_loop),
                    Ok(false) => {}
                    Err(()) => {
                        self.fail(event_loop, UiNativeEventLoopRunDenial::GraphicsPreparation)
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let mut shared = self.shared.borrow_mut();
                let size = shared.window.as_ref().map(|window| window.inner_size());
                let UiNativeHostState {
                    graphics,
                    resources,
                    ..
                } = &mut *shared;
                let replacement =
                    size.zip(graphics.as_mut())
                        .map_or(Ok(false), |(size, graphics)| {
                            graphics.rebind_scale(
                                scale_factor,
                                [size.width, size.height],
                                resources,
                            )
                        });
                drop(shared);
                match replacement {
                    Ok(true) => self.commit_readiness(event_loop),
                    Ok(false) => {}
                    Err(()) => {
                        self.fail(event_loop, UiNativeEventLoopRunDenial::GraphicsPreparation)
                    }
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.request_physical_signal_redraw();
        if self
            .shared
            .borrow()
            .physical_signal
            .observation()
            .pending_wakes
            != 0
        {
            event_loop.set_control_flow(ControlFlow::Poll);
        }
        self.schedule_physical_signal_deadline(event_loop);
        if !self.first_frame_presented {
            return;
        }
        self.idle_wait_turns += 1;
        self.first_frame_presented = false;
    }
}

fn transition_callback_thread(
    slot: &mut Option<UiNativeEventLoopThreadObservation>,
    run_thread: std::thread::ThreadId,
    callback_thread: std::thread::ThreadId,
) -> Result<UiNativeEventLoopThreadObservation, UiNativeEventLoopRunDenial> {
    let observation = UiNativeEventLoopThreadObservation {
        thread: callback_thread,
        matches_launch: run_thread == callback_thread,
    };
    *slot = Some(observation);
    observation
        .matches_launch
        .then_some(observation)
        .ok_or(UiNativeEventLoopRunDenial::ApplicationDriver)
}

fn apply_directive(event_loop: &ActiveEventLoop, directive: UiNativeEventLoopDirective) -> bool {
    match directive {
        UiNativeEventLoopDirective::Continue => {
            event_loop.set_control_flow(ControlFlow::Wait);
            false
        }
        UiNativeEventLoopDirective::WaitUntil(deadline) => {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
            false
        }
        UiNativeEventLoopDirective::Close => true,
    }
}

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    fn resume_admitted(
        &mut self,
        event_loop: &ActiveEventLoop,
        _admission: UiNativeEventLoopThreadObservation,
    ) {
        if self.shared.borrow().window.is_some() {
            return;
        }
        if !self.shared.borrow().resources.admits(6) {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::IncompleteCleanup);
        }
        let opened_window = match UiWinitNativeWindowPort::open(event_loop, &self.configuration) {
            Ok(window) => window,
            Err(_) => return self.fail(event_loop, UiNativeEventLoopRunDenial::WindowCreation),
        };
        let registered_window = {
            let mut shared = self.shared.borrow_mut();
            opened_window.register(&mut shared.resources)
        };
        let (window, window_crossings) = match registered_window {
            Ok(window) => window,
            Err(()) => {
                return self.fail(event_loop, UiNativeEventLoopRunDenial::IncompleteCleanup);
            }
        };
        self.port_crossings = self.port_crossings.saturating_add(window_crossings);
        let prepared_graphics = match UiWgpuNativeGraphicsPort::prepare(Arc::clone(&window)) {
            Ok(graphics) => graphics,
            Err(_) => {
                window.close(&mut self.shared.borrow_mut().resources);
                return self.fail(event_loop, UiNativeEventLoopRunDenial::GraphicsPreparation);
            }
        };
        let (graphics, graphics_crossings) = prepared_graphics.into_parts();
        self.port_crossings = self.port_crossings.saturating_add(graphics_crossings);
        let registered_graphics = {
            let mut shared = self.shared.borrow_mut();
            super::UiNativeOwnedGraphics::register(graphics, &mut shared.resources)
        };
        let graphics = match registered_graphics {
            Ok(graphics) => graphics,
            Err(graphics) => {
                drop(graphics);
                window.close(&mut self.shared.borrow_mut().resources);
                return self.fail(event_loop, UiNativeEventLoopRunDenial::IncompleteCleanup);
            }
        };
        let mut state = self.shared.borrow_mut();
        state.graphics = Some(graphics);
        state.window = Some(window);
        drop(state);
        let preparation = self.shared.borrow().graphics.as_ref().map(|graphics| {
            UiNativeReadinessGrant::issued(
                0,
                (graphics.scale_factor * 1_000.0).round() as u32,
                graphics.extent(),
            )
        });
        let directive = self.client.as_mut().and_then(|client| {
            client
                .native_surface_ready(preparation.expect("graphics was just installed"))
                .ok()
        });
        if directive.is_none() {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
        }
        if apply_directive(event_loop, directive.expect("checked directive")) {
            return event_loop.exit();
        }
        self.commit_readiness(event_loop);
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) {
        let physical = physical_progression::progress_ready_physical_work(
            &mut self.readiness,
            self.physical_readiness_owner,
            &self.shared,
        );
        if physical == physical_progression::UiNativePhysicalWakeProgress::Progressed {
            let directive = self.client.as_mut().and_then(|client| {
                client
                    .physical_work_progressed(UiNativePhysicalProgressGrant::issued())
                    .ok()
            });
            if directive.is_none() {
                return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
            }
            if apply_directive(event_loop, directive.expect("checked directive")) {
                return event_loop.exit();
            }
        }
        self.request_physical_signal_redraw();
        let Ok(work) = self.readiness.take(self.readiness_owner) else {
            return;
        };
        self.redraw_turns += 1;
        let readiness = UiNativeReadinessGrant::issued(
            work.generation,
            work.scale_factor_milli,
            work.client_physical_size,
        );
        let directive = self
            .client
            .as_mut()
            .and_then(|client| client.redraw_ready(readiness).ok());
        if directive.is_none() {
            self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
        } else if apply_directive(event_loop, directive.expect("checked directive")) {
            event_loop.exit();
        } else {
            self.first_frame_presented = true;
        }
    }

    fn commit_readiness(&mut self, event_loop: &ActiveEventLoop) {
        let basis = self.shared.borrow().graphics.as_ref().map(|graphics| {
            (
                (graphics.scale_factor * 1_000.0).round() as u32,
                graphics.extent(),
            )
        });
        let Some((scale_factor_milli, client_physical_size)) = basis else {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::GraphicsPreparation);
        };
        if self
            .readiness
            .commit_latest(
                self.readiness_owner,
                scale_factor_milli,
                client_physical_size,
            )
            .is_err()
        {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
        }
        let window = self
            .shared
            .borrow()
            .window
            .as_ref()
            .map(|window| Arc::clone(window));
        match super::readiness::signal_committed(&mut self.readiness, self.readiness_owner, || {
            if let Some(window) = &window {
                window.request_redraw();
            }
        }) {
            Ok(super::readiness::UiNativeReadinessSignalDisposition::RedrawRequested) => {
                self.readiness_signals += 1;
            }
            Ok(super::readiness::UiNativeReadinessSignalDisposition::Coalesced) => {
                self.coalesced_wakes += 1;
            }
            Ok(super::readiness::UiNativeReadinessSignalDisposition::NoWork) => {
                unreachable!("committed application readiness always carries work")
            }
            Err(()) => self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver),
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, denial: UiNativeEventLoopRunDenial) {
        self.failure = Some(denial);
        event_loop.exit();
    }
}
