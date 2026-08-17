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
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        _device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        basis.observe(if self.settles.get() {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
        } else {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
        })
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
    let peak = report.peak_census();
    assert_eq!(peak.physical_signal_runtimes, 1);
    assert_eq!(peak.physical_signal_workers, 1);
    assert!(peak.entries().all(|(name, count)| {
        matches!(name, "physical_signal_runtimes" | "physical_signal_workers") || count == 0
    }));
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
        let state = &mut *state;
        let owners = reserve_presentation_owners(
            &mut state.resources,
            &mut state.physical_signal,
            crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        )
        .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
        let pending = settle_port_result(
            &mut state.resources,
            &mut state.physical_signal,
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
    std::thread::sleep(std::time::Duration::from_millis(2));
    assert!(cleanup.retry().expect("settled cleanup").is_zero());
    assert!(dropped.get());
}

#[test]
fn delayed_physical_wake_settles_without_an_ordinary_redraw_grant() {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let dropped = Rc::new(std::cell::Cell::new(false));
    let settles = Rc::new(std::cell::Cell::new(false));
    {
        let mut state = state.borrow_mut();
        let UiNativeHostState {
            resources,
            physical_signal,
            pending_presentations,
            ..
        } = &mut *state;
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
        let Err(UiNativePresentationFailure::Indeterminate(pending)) = pending else {
            panic!("the external readback must remain physically pending");
        };
        pending_presentations.push(pending);
        let due = physical_signal
            .next_due_tick()
            .expect("the pending readback owns a delayed physical wake");
        settles.set(true);
        physical_signal
            .advance_clock_to(due)
            .expect("the delayed Signal wake must become ready");
    }

    let mut readiness = crate::native::UiNativeReadinessRegistry::new();
    let ordinary = readiness.register().unwrap();
    let physical = readiness.register_level().unwrap();
    let mut redraw_requests = 0;
    assert_eq!(
        crate::native::readiness::signal_level_ready(&mut readiness, physical, true, || {
            redraw_requests += 1
        },),
        Ok(crate::native::readiness::UiNativeReadinessSignalDisposition::RedrawRequested)
    );

    assert_eq!(
        super::physical_progression::progress_ready_physical_work(&mut readiness, physical, &state,),
        super::physical_progression::UiNativePhysicalWakeProgress::Progressed
    );
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

pub(crate) fn production_event_loop_progresses_ready_atlas_work() {
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let pending = crate::native::mechanics_adapter::seed_pending_atlas_for_event_loop(
        &mut state.borrow_mut(),
    );
    {
        let mut state = state.borrow_mut();
        let due = state
            .physical_signal
            .next_due_tick()
            .expect("the pending atlas upload retains a physical deadline");
        state
            .physical_signal
            .advance_clock_to(due)
            .expect("the event clock admits the exact pending atlas wake");
    }

    let mut readiness = crate::native::UiNativeReadinessRegistry::new();
    let physical = readiness.register_level().unwrap();
    assert_eq!(
        crate::native::readiness::signal_level_ready(&mut readiness, physical, true, || {}),
        Ok(crate::native::readiness::UiNativeReadinessSignalDisposition::RedrawRequested)
    );
    assert_eq!(
        super::physical_progression::progress_ready_physical_work(&mut readiness, physical, &state),
        super::physical_progression::UiNativePhysicalWakeProgress::Progressed
    );
    assert!(matches!(
        state.borrow_mut().complete_pending_text_atlas(pending),
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Committed(_)
    ));
    assert_eq!(
        state.borrow().physical_signal.observation().active_requests,
        0
    );
    assert!(state.borrow_mut().close().is_zero());
}
