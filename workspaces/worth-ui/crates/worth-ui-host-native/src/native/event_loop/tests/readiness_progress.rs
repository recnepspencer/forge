use std::cell::RefCell;
use std::rc::Rc;

use super::super::physical_progression;
use super::PendingProbe;
use crate::native::presentation::{
    reserve_presentation_owners, settle_port_result, UiNativePresentationFailure,
    UiNativePresentationPortFailure,
};
use crate::native::UiNativeHostState;

#[test]
fn delayed_physical_wake_settles_without_an_ordinary_redraw_grant() {
    let (state, dropped) = delayed_pending_presentation();
    let mut readiness = crate::native::UiNativeReadinessRegistry::new();
    let ordinary = readiness.register().unwrap();
    let physical = readiness.register_level().unwrap();
    let mut redraw_requests = 0;
    assert_eq!(
        crate::native::readiness::signal_level_ready(&mut readiness, physical, true, || {
            redraw_requests += 1
        }),
        Ok(crate::native::readiness::UiNativeReadinessSignalDisposition::RedrawRequested)
    );
    assert!(matches!(
        physical_progression::progress_ready_physical_work(&mut readiness, physical, &state),
        physical_progression::UiNativePhysicalWakeProgress::PresentationProgressed { .. }
    ));
    assert_eq!(redraw_requests, 1);
    assert!(readiness.take(ordinary).is_err());
    assert_eq!(
        state.borrow().physical_signal.observation().active_requests,
        0
    );
    assert!(state.borrow().pending_presentations.is_empty());
    assert!(dropped.get());
    assert!(state.borrow_mut().close().is_zero());
}

fn delayed_pending_presentation() -> (Rc<RefCell<UiNativeHostState>>, Rc<std::cell::Cell<bool>>) {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let dropped = Rc::new(std::cell::Cell::new(false));
    let settles = Rc::new(std::cell::Cell::new(false));
    let mut host = state.borrow_mut();
    let UiNativeHostState {
        resources,
        physical_signal,
        pending_presentations,
        ..
    } = &mut *host;
    let owners = reserve_presentation_owners(
        resources,
        physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
    )
    .unwrap_or_else(|_| panic!("the physical request must admit"));
    let pending = settle_port_result(
        resources,
        physical_signal,
        owners,
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(
            Box::new(PendingProbe {
                dropped: Rc::clone(&dropped),
                settles: Rc::clone(&settles),
            }),
        )),
    );
    let Err(UiNativePresentationFailure::Pending(pending)) = pending else {
        panic!("the external readback must remain physically pending");
    };
    pending_presentations.push(pending);
    let due = physical_signal
        .next_due_tick()
        .expect("delayed physical wake");
    settles.set(true);
    physical_signal.advance_clock_to(due).unwrap();
    drop(host);
    (state, dropped)
}
