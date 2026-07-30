use std::sync::Arc;

use worth_ui_host_contract::{UiHostObservationPayload, WorthUiHostMechanicsAdapter};

use super::{initialized_host, pointer_button, present_one, HOST_SESSION};
use crate::adapter::{UiEguiInputTranslatorFamily, UiEguiRawInputIngressOutcome};

#[test]
fn touch_companions_do_not_duplicate_pointer_reports() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    let raw = egui::RawInput {
        events: vec![
            pointer_button(),
            egui::Event::Touch {
                device_id: egui::TouchDeviceId(7),
                id: egui::TouchId::from(9_u64),
                phase: egui::TouchPhase::Start,
                pos: egui::pos2(2.5, 3.75),
                force: Some(0.5),
            },
        ],
        ..Default::default()
    };

    let retained = match host.observe_native_input(&raw) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("canonical pointer companion trace must retain, got {other:?}"),
    };
    assert_eq!(retained.report_count(), 1);
}

#[test]
#[allow(deprecated)]
fn host_screenshot_and_deprecated_ime_lifecycle_events_are_nontranslating_companions() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    let retained = match host.observe_native_input(&egui::RawInput {
        events: vec![
            egui::Event::Screenshot {
                viewport_id: egui::ViewportId::ROOT,
                user_data: egui::UserData::default(),
                image: Arc::new(egui::ColorImage::filled([1, 1], egui::Color32::TRANSPARENT)),
            },
            egui::Event::Ime(egui::ImeEvent::Enabled),
            egui::Event::Ime(egui::ImeEvent::Disabled),
            egui::Event::Text("translated".to_owned()),
        ],
        ..Default::default()
    }) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("companion events must not stop translation, got {other:?}"),
    };
    assert_eq!(retained.reachability().event_count(), 4);
    assert_eq!(retained.report_count(), 1);

    let batches = host
        .drain_mechanical_host_observations(HOST_SESSION)
        .unwrap()
        .into_batches();
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::TextInput { revision: 1, text }
            if text.as_ref() == "translated"
    ));
}

#[test]
#[allow(deprecated)]
fn ime_lifecycle_companions_do_not_require_an_installed_ime_translator() {
    let host =
        initialized_host().without_input_translator_for_testing(UiEguiInputTranslatorFamily::Ime);
    present_one(&host, HOST_SESSION);

    assert!(matches!(
        host.observe_native_input(&egui::RawInput {
            events: vec![
                egui::Event::Ime(egui::ImeEvent::Enabled),
                egui::Event::Ime(egui::ImeEvent::Disabled),
            ],
            ..Default::default()
        }),
        UiEguiRawInputIngressOutcome::NoMechanicalObservations(_)
    ));
}
