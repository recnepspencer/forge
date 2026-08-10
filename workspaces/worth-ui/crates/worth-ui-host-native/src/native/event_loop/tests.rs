use std::cell::RefCell;
use std::rc::Rc;

use super::{
    stop_before_callbacks, transition_callback_thread, UiNativeEventLoopClient,
    UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose, UiNativeEventLoopDirective,
    UiNativeEventLoopRunDenial, UiNativeReadinessGrant,
};
use crate::native::presentation::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationFailure, UiNativePresentationPortFailure,
};
use crate::native::{UiNativeEffectPosture, UiNativeHostState, UiNativeResourceClass};

struct CleanupClient {
    completes: bool,
}

struct PendingProbe {
    dropped: Rc<std::cell::Cell<bool>>,
    settles: Rc<std::cell::Cell<bool>>,
}

impl UiNativePendingExternalObligation for PendingProbe {
    fn try_settle(&mut self, _device: Option<&wgpu::Device>) -> bool {
        self.settles.get()
    }
}

impl Drop for PendingProbe {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

#[test]
fn callback_thread_transition_rejects_off_thread_run() {
    let run_owner = include_str!("run.rs");
    assert!(run_owner.contains("builder.with_any_thread(false);"));
    assert!(!run_owner.contains("builder.with_any_thread(true);"));
    let run_thread = std::thread::current().id();
    let mut observation = None;
    let lawful = transition_callback_thread(&mut observation, run_thread, run_thread).unwrap();
    assert_eq!(lawful.thread, run_thread);
    assert!(lawful.matches_launch);
    assert!(observation.is_some_and(|observed| observed.matches_launch));
    let other = std::thread::spawn(|| std::thread::current().id())
        .join()
        .unwrap();
    assert_eq!(
        transition_callback_thread(&mut observation, run_thread, other),
        Err(UiNativeEventLoopRunDenial::ApplicationDriver)
    );
    let hostile = observation.expect("hostile callback remains observed");
    assert_eq!(hostile.thread, other);
    assert!(!hostile.matches_launch);
}

impl UiNativeEventLoopClient for CleanupClient {
    fn native_surface_ready(
        &mut self,
        _grant: super::UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        unreachable!("cleanup proof never enters callbacks")
    }

    fn redraw_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        unreachable!("cleanup proof never enters callbacks")
    }

    fn presentation_attribution(&self) -> Option<super::UiNativeClientPresentationAttribution> {
        None
    }

    fn close(self) -> UiNativeEventLoopClientClose {
        if self.completes {
            UiNativeEventLoopClientClose::Complete
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(self))
        }
    }
}

impl UiNativeEventLoopClientCleanup for CleanupClient {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose {
        UiNativeEventLoopClientClose::Incomplete(self)
    }
}

#[test]
fn stop_report_retains_effect_posture_and_exact_cleanup_census() {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    {
        let mut state = state.borrow_mut();
        state.effect_posture = UiNativeEffectPosture::PresentationIndeterminate;
    }
    let report = stop_before_callbacks(
        state,
        CleanupClient { completes: true },
        UiNativeEventLoopRunDenial::EventLoopRun,
    );
    assert_eq!(report.cause(), UiNativeEventLoopRunDenial::EventLoopRun);
    assert_eq!(
        report.effect_posture(),
        UiNativeEffectPosture::PresentationIndeterminate
    );
    assert!(report.peak_census().is_zero());
    assert!(report.terminal_census().is_zero());
    assert!(report.client_cleanup_complete());
}

#[test]
fn held_resource_with_clean_client_cannot_report_a_clean_stop() {
    for class in UiNativeResourceClass::all() {
        let state = Rc::new(RefCell::new(UiNativeHostState::new()));
        let _held = state.borrow_mut().resources.register(*class).unwrap();
        let report = stop_before_callbacks(
            state,
            CleanupClient { completes: true },
            UiNativeEventLoopRunDenial::EventLoopCreation,
        );
        assert_eq!(
            report.cause(),
            UiNativeEventLoopRunDenial::IncompleteCleanup,
            "resource class {class:?} was omitted from terminal admission"
        );
        assert!(!report.terminal_census().is_zero());
        assert!(report.client_cleanup_complete());
    }
}

#[test]
fn incomplete_client_without_held_resources_cannot_report_a_clean_stop() {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let report = stop_before_callbacks(
        state,
        CleanupClient { completes: false },
        UiNativeEventLoopRunDenial::EventLoopCreation,
    );
    assert_eq!(
        report.cause(),
        UiNativeEventLoopRunDenial::IncompleteCleanup
    );
    assert!(report.terminal_census().is_zero());
    assert!(!report.client_cleanup_complete());
}

#[test]
fn indeterminate_external_work_moves_into_retryable_cleanup_authority() {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let dropped = Rc::new(std::cell::Cell::new(false));
    let settles = Rc::new(std::cell::Cell::new(false));
    {
        let mut state = state.borrow_mut();
        let owners = reserve_presentation_owners(&mut state.resources)
            .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
        let pending = settle_port_result(
            &mut state.resources,
            owners,
            Err(UiNativePresentationPortFailure::ReadbackUnsettled(
                Box::new(PendingProbe {
                    dropped: Rc::clone(&dropped),
                    settles: Rc::clone(&settles),
                }),
            )),
        );
        let Err(UiNativePresentationFailure::Indeterminate(pending)) = pending else {
            panic!("unsettled external work must enter retryable cleanup");
        };
        state.pending_presentations.push(pending);
    }
    let report = stop_before_callbacks(
        state,
        CleanupClient { completes: true },
        UiNativeEventLoopRunDenial::EventLoopRun,
    );
    assert_eq!(
        report.cause(),
        UiNativeEventLoopRunDenial::IncompleteCleanup
    );
    assert!(!dropped.get());
    let cleanup = report.into_cleanup().expect("pending cleanup authority");
    let cleanup = match cleanup.retry() {
        Err(cleanup) => cleanup,
        Ok(_) => panic!("unsettled external work must retain cleanup authority"),
    };
    assert!(!dropped.get());
    settles.set(true);
    assert!(cleanup.retry().expect("settled cleanup").is_zero());
    assert!(dropped.get());
}
