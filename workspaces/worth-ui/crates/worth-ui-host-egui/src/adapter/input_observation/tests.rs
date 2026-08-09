use worth_ui_host_contract::{
    UiHostImeCompositionPhase, UiHostImePreeditSelection, UiHostKey, UiHostKeyTransition,
    UiHostKeyboardModifiers, UiHostObservationPayload, UiHostObservationRetentionDenial,
    UiHostPointerButton, UiHostPointerButtonTransition, UiHostPresentationEpoch,
    UiHostProtocolContract, UiHostProtocolNegotiation, UiHostSurfaceIdentity,
    UiHostSurfacePositionBasis, UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome,
    UiHostSurfaceRegistrationInput, UiHostSurfaceRegistrationOutcome,
    UiHostSurfaceRegistrationRequest, UiMountedFrameConsumptionInput,
    UiMountedPresentationAttemptIdentity, UiMountedPresentationWorkView,
    UiMountedSurfaceBindingRequirement, WorthUiHostMechanicsAdapter,
    UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT,
};
use worth_ui_test_support::{
    initial_presentation_mechanics_for_certification,
    semantic_text_projection_for_certification_with_capability,
};

use super::{UiEguiRawInputIngressOutcome, UiEguiRawInputIngressStopReason};
use crate::adapter::WorthUiHostEgui;

mod atomic_stops;
mod companion_events;
mod teardown;
mod translator_inventory;

const HOST_SESSION: u64 = 41;

#[test]
fn input_observation_translates_exact_families_against_completed_presentation() {
    let host = initialized_host();
    let presented = present_one(&host, HOST_SESSION);
    let raw = egui::RawInput {
        events: exact_event_family_trace(),
        ..Default::default()
    };

    let retained = match host.observe_native_input(&raw) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("exact translated input must retain, got {other:?}"),
    };
    assert_eq!(retained.report_count(), 8);
    assert_eq!(retained.presentation(), presented.presentation);
    assert_eq!(retained.sequences().first().value(), 1);
    assert_eq!(retained.sequences().last().value(), 8);
    assert_eq!(retained.reachability().event_count(), 9);

    let drain = host
        .drain_mechanical_host_observations(HOST_SESSION)
        .expect("adapter drain remains structurally bounded");
    let batches = drain.into_batches();
    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    assert_eq!(
        batch.canonical_core().presentation(),
        presented.presentation
    );
    assert_pointer_reports(batch.reports());
    assert_keyboard_report(&batch.reports()[2]);
    assert_text_and_ime_reports(batch.reports());
}

#[test]
fn report_overflow_is_atomic_and_does_not_advance_sequence() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    let key = key_event();
    let overflow = egui::RawInput {
        events: vec![key.clone(); UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT + 1],
        ..Default::default()
    };

    assert_stop(
        host.observe_native_input(&overflow),
        UiEguiRawInputIngressStopReason::ReportLimitExceeded,
    );
    assert_eq!(
        host.drain_mechanical_host_observations(HOST_SESSION)
            .unwrap()
            .into_batches()
            .len(),
        0
    );
    let retained = match host.observe_native_input(&egui::RawInput {
        events: vec![key],
        ..Default::default()
    }) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("post-overflow retry must retain, got {other:?}"),
    };
    assert_eq!(retained.sequences().first().value(), 1);
}

#[test]
fn retention_stop_is_atomic_and_retry_resumes_exact_sequence() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    let raw = egui::RawInput {
        events: vec![key_event()],
        ..Default::default()
    };
    for _ in 0..16 {
        assert!(matches!(
            host.observe_native_input(&raw),
            UiEguiRawInputIngressOutcome::Retained(_)
        ));
    }
    assert_stop(
        host.observe_native_input(&raw),
        UiEguiRawInputIngressStopReason::Retention(UiHostObservationRetentionDenial::Capacity(
            worth_ui_host_contract::UiHostObservationDrainDenial::BatchCapacityExceeded,
        )),
    );
    assert_eq!(
        host.drain_mechanical_host_observations(HOST_SESSION)
            .unwrap()
            .into_batches()
            .len(),
        16
    );
    let retry = match host.observe_native_input(&raw) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("drained retry must retain, got {other:?}"),
    };
    assert_eq!(retry.sequences().first().value(), 17);
}

#[test]
fn missing_ambiguous_and_released_presentation_bases_stop_explicitly() {
    let host = initialized_host();
    assert_stop(
        host.observe_native_input(&egui::RawInput::default()),
        UiEguiRawInputIngressStopReason::NoPresentedSurface,
    );
    present_one(&host, HOST_SESSION);
    present_one(&host, HOST_SESSION);
    assert_stop(
        host.observe_native_input(&egui::RawInput::default()),
        UiEguiRawInputIngressStopReason::AmbiguousPresentedSurfaces { count: 2 },
    );
    let _ = host.release_mechanical_host_session(HOST_SESSION);
    assert_stop(
        host.observe_native_input(&egui::RawInput::default()),
        UiEguiRawInputIngressStopReason::NoPresentedSurface,
    );
    assert_eq!(
        host.drain_mechanical_host_observations(HOST_SESSION)
            .unwrap()
            .into_batches()
            .len(),
        0
    );
}

#[derive(Clone, Copy)]
struct PresentedInputWorld {
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
}

fn initialized_host() -> WorthUiHostEgui {
    let context = egui::Context::default();
    let _ = context.run_ui(egui::RawInput::default(), |_| {});
    WorthUiHostEgui::new(context)
}

fn present_one(host: &WorthUiHostEgui, host_session: u64) -> PresentedInputWorld {
    let capabilities = host.mechanical_capability_report();
    let projection = semantic_text_projection_for_certification_with_capability(
        capabilities.observation_generation(),
        capabilities.profile_identity_digest(),
    );
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(denial) => panic!("{denial:?}"),
    };
    let host_surface = UiHostSurfaceIdentity::mint_unbound().unwrap();
    let requirement = UiMountedSurfaceBindingRequirement::new(
        projection.surface(),
        host_surface,
        projection.binding(),
        capabilities.observation_generation(),
        capabilities.profile_identity_digest(),
        UiHostSurfacePresentationMode::NativeDisplay,
    );
    let registration =
        UiHostSurfaceRegistrationRequest::from_runtime(UiHostSurfaceRegistrationInput {
            host_session_identity: host_session,
            semantic_surface_identity: projection.surface(),
            host_surface_identity: host_surface,
            binding_generation: projection.binding(),
            protocol,
            capability_generation: capabilities.observation_generation(),
            capability_profile_digest: capabilities.profile_identity_digest(),
            presentation_mode: UiHostSurfacePresentationMode::NativeDisplay,
        });
    assert!(matches!(
        host.perform_surface_registration(registration),
        UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    ));
    let attempt = UiMountedPresentationAttemptIdentity::mint_unbound().unwrap();
    let presentation_work =
        initial_presentation_mechanics_for_certification(&projection, requirement);
    let view = worth_ui_host_contract::UiMountedFrameConsumptionView::from_inert_mechanics(
        UiMountedFrameConsumptionInput {
            authority: std::rc::Rc::new(()),
            host_session_identity: host_session,
            protocol,
            capability_generation: capabilities.observation_generation(),
            capability_profile_digest: capabilities.profile_identity_digest(),
            attempt,
            deadline: worth_ui_host_contract::UiPresentationDeadline::at_tick(100),
            requirement,
            presentation_work: UiMountedPresentationWorkView::Initial(&presentation_work),
        },
    );
    let epoch = match host.perform_mounted_surface_presentation(&view) {
        UiHostSurfacePresentationOutcome::Presented(completion) => completion.epoch(),
        other => panic!("production egui presentation must complete, got {other:?}"),
    };
    assert_eq!(
        epoch,
        UiHostPresentationEpoch::issued_by_host(attempt.diagnostic_value())
    );
    PresentedInputWorld {
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis::new(
            projection.frame(),
            projection.binding(),
            epoch,
        ),
    }
}

fn exact_event_family_trace() -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(egui::pos2(1.25, -2.5)),
        pointer_button(),
        egui::Event::Touch {
            device_id: egui::TouchDeviceId(7),
            id: egui::TouchId::from(9_u64),
            phase: egui::TouchPhase::Start,
            pos: egui::pos2(2.5, 3.75),
            force: None,
        },
        key_event(),
        egui::Event::Text("é".to_owned()),
        egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "aé🦀z".to_owned(),
            active_range_chars: Some(1..3),
        }),
        egui::Event::Ime(egui::ImeEvent::Commit("done".to_owned())),
        egui::Event::Ime(egui::ImeEvent::Preedit {
            text: String::new(),
            active_range_chars: None,
        }),
        egui::Event::Paste("paste".to_owned()),
    ]
}

fn pointer_button() -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(2.5, 3.75),
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    }
}

fn key_event() -> egui::Event {
    egui::Event::Key {
        key: egui::Key::A,
        physical_key: Some(egui::Key::B),
        pressed: true,
        repeat: true,
        modifiers: egui::Modifiers {
            alt: true,
            ctrl: true,
            shift: true,
            mac_cmd: true,
            command: true,
        },
    }
}

fn assert_pointer_reports(reports: &[worth_ui_host_contract::UiHostObservationReport]) {
    let UiHostObservationPayload::PointerMotion {
        capture_epoch,
        pressed_buttons,
        position,
        ..
    } = reports[0].payload()
    else {
        panic!("first report must be pointer motion");
    };
    assert_eq!(capture_epoch.value(), 1);
    assert_eq!(pressed_buttons.bits(), 0);
    assert_eq!(
        position.basis(),
        UiHostSurfacePositionBasis::viewport_logical()
    );
    assert_eq!(
        [position.x_subpixels(), position.y_subpixels()],
        [1_250, -2_500]
    );
    let UiHostObservationPayload::PointerButton {
        capture_epoch,
        button,
        transition,
        position,
        ..
    } = reports[1].payload()
    else {
        panic!("second report must be pointer button");
    };
    assert_eq!(capture_epoch.value(), 1);
    assert_eq!(*button, UiHostPointerButton::Primary);
    assert_eq!(*transition, UiHostPointerButtonTransition::Pressed);
    assert_eq!(
        position.basis(),
        UiHostSurfacePositionBasis::viewport_logical()
    );
    assert_eq!(
        [position.x_subpixels(), position.y_subpixels()],
        [2_500, 3_750]
    );
}

fn assert_keyboard_report(report: &worth_ui_host_contract::UiHostObservationReport) {
    let UiHostObservationPayload::Keyboard {
        logical_key,
        physical_key,
        modifiers,
        transition,
    } = report.payload()
    else {
        panic!("third report must be keyboard");
    };
    assert_eq!(*logical_key, UiHostKey::A);
    assert_eq!(*physical_key, Some(UiHostKey::B));
    assert_eq!(
        *modifiers,
        UiHostKeyboardModifiers::new(true, true, true, true, true)
    );
    assert_eq!(*transition, UiHostKeyTransition::Pressed { repeat: true });
}

fn assert_text_and_ime_reports(reports: &[worth_ui_host_contract::UiHostObservationReport]) {
    assert!(matches!(
        reports[3].payload(),
        UiHostObservationPayload::TextInput { revision: 1, text } if text.as_ref() == "é"
    ));
    let UiHostObservationPayload::ImeComposition {
        revision: 2,
        phase: UiHostImeCompositionPhase::Preedit(preedit),
    } = reports[4].payload()
    else {
        panic!("fifth report must be IME preedit");
    };
    let UiHostImePreeditSelection::Converted(range) = preedit.selection() else {
        panic!("preedit must carry converted range");
    };
    assert_eq!([range.source().start(), range.source().end()], [1, 3]);
    assert_eq!([range.canonical().start(), range.canonical().end()], [1, 7]);
    assert!(matches!(
        reports[5].payload(),
        UiHostObservationPayload::ImeComposition {
            revision: 3,
            phase: UiHostImeCompositionPhase::Commit(text),
        } if text.as_ref() == "done"
    ));
    assert!(matches!(
        reports[6].payload(),
        UiHostObservationPayload::ImeComposition {
            revision: 4,
            phase: UiHostImeCompositionPhase::Cancel,
        }
    ));
    assert!(matches!(
        reports[7].payload(),
        UiHostObservationPayload::TextInput { revision: 5, text } if text.as_ref() == "paste"
    ));
}

fn assert_stop(outcome: UiEguiRawInputIngressOutcome, expected: UiEguiRawInputIngressStopReason) {
    match outcome {
        UiEguiRawInputIngressOutcome::Stopped(stop) => assert_eq!(stop.reason(), expected),
        other => panic!("input must stop as {expected:?}, got {other:?}"),
    }
}
