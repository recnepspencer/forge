use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::{ActiveEventLoop, ControlFlow};

use super::{UiNativeEventLoopDirective, UiNativeHostState};

pub(super) fn apply(
    shared: &Rc<RefCell<UiNativeHostState>>,
    event_loop: &ActiveEventLoop,
    directive: UiNativeEventLoopDirective,
) -> bool {
    match directive {
        UiNativeEventLoopDirective::Continue => set_wait(event_loop),
        UiNativeEventLoopDirective::ExternalObservationReady => {
            if let Some(window) = shared.borrow().window.as_ref() {
                window.publish_external_observation_readiness();
            }
            set_wait(event_loop)
        }
        UiNativeEventLoopDirective::WaitUntil(deadline) => {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
            false
        }
        UiNativeEventLoopDirective::Close => true,
    }
}

fn set_wait(event_loop: &ActiveEventLoop) -> bool {
    event_loop.set_control_flow(ControlFlow::Wait);
    false
}
