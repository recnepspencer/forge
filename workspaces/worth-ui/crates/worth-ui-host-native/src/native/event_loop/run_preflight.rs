use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::EventLoop;

use super::{UiNativeEventLoopRunDenial, UiNativeEventLoopThreadPosture};
use crate::native::{
    UiNativeHostState, UiNativeReadinessRegistry, UiNativeReadyOwner, UiNativeResourceClass,
    UiNativeResourceOwner,
};

pub(super) struct UiNativeEventLoopRunPreflight {
    pub event_loop: EventLoop<()>,
    pub readiness: UiNativeReadinessRegistry,
    pub readiness_owner: UiNativeReadyOwner,
    pub physical_readiness_owner: UiNativeReadyOwner,
    pub input_readiness_owner: UiNativeReadyOwner,
    pub loop_resources: Vec<UiNativeResourceOwner>,
}

pub(super) fn prepare(
    state: &Rc<RefCell<UiNativeHostState>>,
    thread_posture: UiNativeEventLoopThreadPosture,
) -> Result<UiNativeEventLoopRunPreflight, UiNativeEventLoopRunDenial> {
    let mut builder = EventLoop::<()>::builder();
    thread_posture.configure(&mut builder);
    let event_loop = builder
        .build()
        .map_err(|_| UiNativeEventLoopRunDenial::EventLoopCreation)?;
    let loop_resources = state
        .borrow_mut()
        .resources
        .reserve(&[
            UiNativeResourceClass::ApplicationDriver,
            UiNativeResourceClass::EventWakeRegistration,
        ])
        .map_err(|_| UiNativeEventLoopRunDenial::ApplicationDriver)?;
    let mut readiness = UiNativeReadinessRegistry::new();
    let readiness_owner = match readiness.register() {
        Ok(owner) => owner,
        Err(()) => {
            release_resources(state, loop_resources);
            return Err(UiNativeEventLoopRunDenial::ApplicationDriver);
        }
    };
    let physical_readiness_owner = match readiness.register_level() {
        Ok(owner) => owner,
        Err(()) => {
            readiness.close();
            release_resources(state, loop_resources);
            return Err(UiNativeEventLoopRunDenial::IncompleteCleanup);
        }
    };
    let input_readiness_owner = match readiness.register_level() {
        Ok(owner) => owner,
        Err(()) => {
            readiness.close();
            release_resources(state, loop_resources);
            return Err(UiNativeEventLoopRunDenial::IncompleteCleanup);
        }
    };
    Ok(UiNativeEventLoopRunPreflight {
        event_loop,
        readiness,
        readiness_owner,
        physical_readiness_owner,
        input_readiness_owner,
        loop_resources,
    })
}

fn release_resources(
    state: &Rc<RefCell<UiNativeHostState>>,
    resources: Vec<UiNativeResourceOwner>,
) {
    state
        .borrow_mut()
        .resources
        .release_all(resources)
        .expect("event-loop preflight owners remain exact");
}
