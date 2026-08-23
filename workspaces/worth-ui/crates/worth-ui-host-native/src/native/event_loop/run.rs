use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::ControlFlow;

use super::{
    run_preflight, UiNativeEventLoopApplication, UiNativeEventLoopCleanup, UiNativeEventLoopClient,
    UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport, UiNativeEventLoopStopReport,
    WorthUiNativeEventLoop,
};
use crate::native::UiNativeHostState;

impl WorthUiNativeEventLoop {
    pub fn run<Client: UiNativeEventLoopClient>(
        self,
        client: Client,
    ) -> Result<UiNativeEventLoopRunReport, UiNativeEventLoopStopReport> {
        let preflight = match run_preflight::prepare(&self.state, self.thread_posture) {
            Ok(preflight) => preflight,
            Err(cause) => return Err(stop_before_callbacks(self.state, client, cause)),
        };
        let run_preflight::UiNativeEventLoopRunPreflight {
            event_loop,
            readiness,
            readiness_owner,
            physical_readiness_owner,
            input_readiness_owner,
            loop_resources,
        } = preflight;
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut application = UiNativeEventLoopApplication {
            shared: self.state,
            configuration: self.window,
            client: Some(client),
            first_frame_presented: false,
            readiness,
            readiness_owner,
            physical_readiness_owner,
            input_readiness_owner,
            readiness_signals: 0,
            redraw_turns: 0,
            idle_wait_turns: 0,
            coalesced_wakes: 0,
            failure: None,
            run_thread: std::thread::current().id(),
            thread_observation: None,
            loop_resources,
            port_crossings: 0,
            physical_clock: super::physical_clock::UiNativePhysicalEventClock::new(),
            pointer_input: None,
            thread_posture: self.thread_posture,
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
    let peak_census = state.borrow().compiler_total_peak();
    let peak_text_pins = state.borrow().peak_text_pins.clone();
    let input_observations = state.borrow().lifecycle_protocol.report();
    let effect_posture = state.borrow().effect_posture;
    let (client_cleanup, _) = client.close().into_parts();
    let client_cleanup_complete = client_cleanup.is_none();
    let terminal_census = state.borrow_mut().close();
    let cleanup = UiNativeEventLoopCleanup::retain(
        Rc::clone(&state),
        terminal_census,
        client_cleanup,
        super::physical_clock::UiNativePhysicalEventClock::new(),
    );
    UiNativeEventLoopStopReport {
        cause: if super::terminal_cleanup::terminal_cleanup_complete(
            client_cleanup_complete,
            true,
            &terminal_census,
        ) {
            cause
        } else {
            UiNativeEventLoopRunDenial::IncompleteCleanup
        },
        effect_posture,
        peak_census,
        terminal_census,
        client_cleanup_complete,
        cleanup,
        peak_text_pins,
        input_observations,
    }
}
