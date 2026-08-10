use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::{ControlFlow, EventLoop};
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

use super::{
    terminal_cleanup_complete, UiNativeEventLoopApplication, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport, WorthUiNativeEventLoop,
};
use crate::native::{UiNativeHostState, UiNativeReadinessRegistry, UiNativeResourceClass};

impl WorthUiNativeEventLoop {
    pub fn run<Client: UiNativeEventLoopClient>(
        self,
        client: Client,
    ) -> Result<UiNativeEventLoopRunReport, UiNativeEventLoopStopReport> {
        let mut builder = EventLoop::<()>::builder();
        #[cfg(target_os = "windows")]
        builder.with_any_thread(false);
        let event_loop = match builder.build() {
            Ok(event_loop) => event_loop,
            Err(_) => {
                return Err(stop_before_callbacks(
                    self.state,
                    client,
                    UiNativeEventLoopRunDenial::EventLoopCreation,
                ));
            }
        };
        let loop_resources = {
            let mut state = self.state.borrow_mut();
            state.resources.reserve(&[
                UiNativeResourceClass::ApplicationDriver,
                UiNativeResourceClass::EventWakeRegistration,
            ])
        };
        let Ok(loop_resources) = loop_resources else {
            return Err(stop_before_callbacks(
                self.state,
                client,
                UiNativeEventLoopRunDenial::ApplicationDriver,
            ));
        };
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut readiness = UiNativeReadinessRegistry::new();
        let readiness_owner = match readiness.register() {
            Ok(owner) => owner,
            Err(()) => {
                self.state
                    .borrow_mut()
                    .resources
                    .release_all(loop_resources)
                    .expect("event-loop preflight owners remain exact");
                return Err(stop_before_callbacks(
                    self.state,
                    client,
                    UiNativeEventLoopRunDenial::ApplicationDriver,
                ));
            }
        };
        let mut application = UiNativeEventLoopApplication {
            shared: self.state,
            configuration: self.window,
            client: Some(client),
            first_frame_presented: false,
            readiness,
            readiness_owner,
            readiness_signals: 0,
            redraw_turns: 0,
            idle_wait_turns: 0,
            coalesced_wakes: 0,
            failure: None,
            run_thread: std::thread::current().id(),
            thread_observation: None,
            loop_resources,
            port_crossings: 0,
        };
        if event_loop.run_app(&mut application).is_err() {
            application
                .failure
                .get_or_insert(UiNativeEventLoopRunDenial::EventLoopRun);
        }
        application.finish()
    }
}

pub(super) fn stop_before_callbacks<Client: UiNativeEventLoopClient>(
    state: Rc<RefCell<UiNativeHostState>>,
    client: Client,
    cause: UiNativeEventLoopRunDenial,
) -> UiNativeEventLoopStopReport {
    let peak_census = state.borrow().resources.peak();
    let effect_posture = state.borrow().effect_posture;
    let client_cleanup = client.close().into_cleanup();
    let client_cleanup_complete = client_cleanup.is_none();
    let terminal_census = state.borrow_mut().close();
    let cleanup =
        UiNativeEventLoopCleanup::retain(Rc::clone(&state), terminal_census, client_cleanup);
    UiNativeEventLoopStopReport {
        cause: if terminal_cleanup_complete(client_cleanup_complete, true, &terminal_census) {
            cause
        } else {
            UiNativeEventLoopRunDenial::IncompleteCleanup
        },
        effect_posture,
        peak_census,
        terminal_census,
        client_cleanup_complete,
        cleanup,
    }
}
