use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};

use crate::UiNativeWindowConfiguration;

use super::{
    UiNativeGraphicsObservation, UiNativeGraphicsPort, UiNativeHostState, UiWgpuNativeGraphicsPort,
};

mod contract;
mod run;
mod terminal_cleanup;
#[cfg(test)]
mod tests;
mod window_port;

#[cfg(test)]
use run::stop_before_callbacks;

pub use contract::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopClient, UiNativeEventLoopDirective,
    UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport, UiNativeEventLoopStopReport,
    UiNativeReadinessGrant,
};
use terminal_cleanup::terminal_cleanup_complete;
use window_port::{UiNativeWindowPort, UiWinitNativeWindowPort};

pub struct WorthUiNativeEventLoop {
    state: Rc<RefCell<UiNativeHostState>>,
    window: UiNativeWindowConfiguration,
}

struct UiNativeEventLoopApplication<Client> {
    shared: Rc<RefCell<UiNativeHostState>>,
    configuration: UiNativeWindowConfiguration,
    window: Option<Arc<Window>>,
    client: Option<Client>,
    first_frame_presented: bool,
    readiness: super::UiNativeReadinessRegistry,
    readiness_owner: super::UiNativeReadyOwner,
    readiness_signals: u64,
    redraw_turns: u64,
    idle_wait_turns: u64,
    coalesced_wakes: u64,
    failure: Option<UiNativeEventLoopRunDenial>,
    run_thread: std::thread::ThreadId,
    thread_observation: Option<UiNativeEventLoopThreadObservation>,
    loop_resources: Vec<super::UiNativeResourceOwner>,
    window_resource: Option<super::UiNativeResourceOwner>,
    port_crossings: u8,
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
            .window
            .as_ref()
            .is_none_or(|window| window.id() != window_id)
        {
            return;
        }
        match event {
            WindowEvent::RedrawRequested => self.redraw(event_loop),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                let changed = self
                    .shared
                    .borrow_mut()
                    .graphics
                    .as_mut()
                    .is_some_and(|graphics| graphics.resize([size.width, size.height]));
                if changed {
                    self.commit_readiness(event_loop);
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let changed = if let (Some(window), Some(graphics)) = (
                    self.window.as_ref(),
                    self.shared.borrow_mut().graphics.as_mut(),
                ) {
                    let size = window.inner_size();
                    graphics.rebind_scale(scale_factor, [size.width, size.height])
                } else {
                    false
                };
                if changed {
                    self.commit_readiness(event_loop);
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
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
        if self.window.is_some() {
            return;
        }
        let native_resources = self.shared.borrow_mut().resources.reserve(&[
            super::UiNativeResourceClass::Window,
            super::UiNativeResourceClass::Surface,
            super::UiNativeResourceClass::Adapter,
            super::UiNativeResourceClass::Device,
            super::UiNativeResourceClass::Queue,
            super::UiNativeResourceClass::RetainedTarget,
        ]);
        let Ok(mut native_resources) = native_resources else {
            return self.fail(event_loop, UiNativeEventLoopRunDenial::IncompleteCleanup);
        };
        let window = match UiWinitNativeWindowPort::open(event_loop, &self.configuration) {
            Ok(window) => window,
            Err(_) => {
                self.shared
                    .borrow_mut()
                    .resources
                    .release_all(native_resources)
                    .expect("preflight owners remain exact");
                return self.fail(event_loop, UiNativeEventLoopRunDenial::WindowCreation);
            }
        };
        self.port_crossings = self.port_crossings.saturating_add(1);
        let graphics = match UiWgpuNativeGraphicsPort::prepare(Arc::clone(&window)) {
            Ok(graphics) => graphics,
            Err(_) => {
                self.shared
                    .borrow_mut()
                    .resources
                    .release_all(native_resources)
                    .expect("preflight owners remain exact");
                return self.fail(event_loop, UiNativeEventLoopRunDenial::GraphicsPreparation);
            }
        };
        self.port_crossings = self.port_crossings.saturating_add(1);
        self.window_resource = Some(native_resources.remove(0));
        let mut state = self.shared.borrow_mut();
        state.graphics_resources = native_resources;
        state.graphics = Some(graphics);
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
        self.window = Some(window);
        self.commit_readiness(event_loop);
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) {
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
        let window = self.window.as_ref();
        match super::readiness::signal_committed(&mut self.readiness, self.readiness_owner, || {
            if let Some(window) = window {
                window.request_redraw();
            }
        }) {
            Ok(super::readiness::UiNativeReadinessSignalDisposition::RedrawRequested) => {
                self.readiness_signals += 1;
            }
            Ok(super::readiness::UiNativeReadinessSignalDisposition::Coalesced) => {
                self.coalesced_wakes += 1;
            }
            Err(()) => self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver),
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, denial: UiNativeEventLoopRunDenial) {
        self.failure = Some(denial);
        event_loop.exit();
    }

    fn finish(mut self) -> Result<UiNativeEventLoopRunReport, UiNativeEventLoopStopReport> {
        let presentation = self.shared.borrow().last_presentation.clone();
        let client_attribution = self
            .client
            .as_ref()
            .and_then(UiNativeEventLoopClient::presentation_attribution);
        let peak_census = self.shared.borrow().resources.peak();
        let effect_posture = self.shared.borrow().effect_posture;
        let graphics = self
            .shared
            .borrow()
            .graphics
            .as_ref()
            .map(UiNativeGraphicsObservation::from_graphics);
        let client_closed = self
            .client
            .take()
            .is_some_and(|client| client.close().is_ok());
        let readiness_owner_count = self.readiness.close();
        self.window = None;
        let mut shared = self.shared.borrow_mut();
        if let Some(owner) = self.window_resource.take() {
            shared
                .resources
                .release(owner)
                .expect("window owner must remain exact");
        }
        shared
            .resources
            .release_all(self.loop_resources.drain(..))
            .expect("event-loop owners must remain exact");
        let host_census = shared.close();
        drop(shared);
        let cleanup_complete =
            terminal_cleanup_complete(client_closed, readiness_owner_count == 1, &host_census);
        let failure = if !cleanup_complete {
            Some(UiNativeEventLoopRunDenial::IncompleteCleanup)
        } else {
            self.failure
                .or_else(|| {
                    presentation
                        .is_none()
                        .then_some(UiNativeEventLoopRunDenial::ApplicationDriver)
                })
                .or_else(|| {
                    graphics
                        .is_none()
                        .then_some(UiNativeEventLoopRunDenial::GraphicsPreparation)
                })
                .or_else(|| {
                    self.thread_observation
                        .is_none()
                        .then_some(UiNativeEventLoopRunDenial::EventLoopRun)
                })
                .or_else(|| {
                    self.thread_observation
                        .is_some_and(|observation| !observation.matches_launch)
                        .then_some(UiNativeEventLoopRunDenial::EventLoopRun)
                })
                .or_else(|| {
                    client_attribution
                        .zip(presentation.as_ref())
                        .is_none_or(|(attribution, observed)| !attribution.matches(observed))
                        .then_some(UiNativeEventLoopRunDenial::ApplicationDriver)
                })
        };
        if let Some(cause) = failure {
            return Err(UiNativeEventLoopStopReport {
                cause,
                effect_posture,
                peak_census,
                terminal_census: host_census,
                client_cleanup_complete: client_closed,
            });
        }
        let presentation = presentation.expect("validated presentation");
        let port_crossings = self
            .port_crossings
            .saturating_add(presentation.port_crossings());
        let graphics = graphics.expect("validated graphics");
        let client_attribution = client_attribution.expect("validated client attribution");
        let thread = self
            .thread_observation
            .expect("validated event-loop thread");
        Ok(UiNativeEventLoopRunReport {
            presentation,
            graphics,
            event_loop_thread: format!("{:?}", thread.thread).into_boxed_str(),
            event_loop_thread_matches_launch: thread.matches_launch,
            client_attribution,
            readiness_signals: self.readiness_signals,
            redraw_turns: self.redraw_turns,
            idle_wait_turns: self.idle_wait_turns,
            coalesced_wakes: self.coalesced_wakes,
            peak_census,
            terminal_census: host_census,
            port_crossings,
        })
    }
}
