use std::rc::Rc;

use winit::event_loop::ActiveEventLoop;

use super::{UiNativeEventLoopApplication, UiNativeEventLoopClient, UiNativeEventLoopRunDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativePhysicalWakeProgress {
    NoWake,
    ConsumedWithoutReadyWork,
    Progressed,
}

pub(super) fn progress_ready_physical_work(
    readiness: &mut crate::native::UiNativeReadinessRegistry,
    physical_owner: crate::native::UiNativeReadyOwner,
    shared: &Rc<std::cell::RefCell<crate::native::UiNativeHostState>>,
) -> UiNativePhysicalWakeProgress {
    if readiness.take_level(physical_owner).is_err() {
        return UiNativePhysicalWakeProgress::NoWake;
    }
    if shared.borrow_mut().progress_one_physical_signal_ready() {
        UiNativePhysicalWakeProgress::Progressed
    } else {
        UiNativePhysicalWakeProgress::ConsumedWithoutReadyWork
    }
}

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn advance_physical_signal_clock(&mut self, event_loop: &ActiveEventLoop) {
        let tick = self.physical_clock.current_tick();
        if self
            .shared
            .borrow_mut()
            .physical_signal
            .advance_clock_to(tick)
            .is_err()
        {
            self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
            return;
        }
        self.request_physical_signal_redraw();
    }

    pub(super) fn schedule_physical_signal_deadline(&self, event_loop: &ActiveEventLoop) {
        let due_tick = self.shared.borrow().physical_signal.next_due_tick();
        let Some(deadline) = due_tick.and_then(|tick| self.physical_clock.deadline(tick)) else {
            return;
        };
        event_loop.set_control_flow(super::physical_clock::tighten_deadline(
            event_loop.control_flow(),
            deadline,
        ));
    }

    pub(super) fn request_physical_signal_redraw(&mut self) {
        let ready = self
            .shared
            .borrow()
            .physical_signal
            .observation()
            .pending_wakes
            != 0;
        if self.shared.borrow().window.is_none() {
            return;
        }
        let shared = Rc::clone(&self.shared);
        let _ = crate::native::readiness::signal_level_ready(
            &mut self.readiness,
            self.physical_readiness_owner,
            ready,
            || {
                if let Some(window) = shared.borrow().window.as_ref() {
                    window.request_redraw();
                }
            },
        );
    }
}
