use std::cell::RefCell;
use std::rc::Rc;

use super::{
    stop_before_callbacks, transition_callback_thread, UiNativeEventLoopClient,
    UiNativeEventLoopDirective, UiNativeEventLoopRunDenial, UiNativeReadinessGrant,
};
use crate::native::{UiNativeEffectPosture, UiNativeHostState, UiNativeResourceClass};

struct CleanupClient {
    completes: bool,
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

    fn close(self) -> Result<(), ()> {
        self.completes.then_some(()).ok_or(())
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
