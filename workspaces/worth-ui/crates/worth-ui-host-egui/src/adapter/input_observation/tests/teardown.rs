use worth_ui_host_contract::{UiHostObservationPayload, WorthUiHostMechanicsAdapter};

use super::{initialized_host, present_one, HOST_SESSION};
use crate::adapter::UiEguiRawInputIngressOutcome;

#[test]
fn session_release_discards_retained_batches_and_translation_revision_state() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    assert!(matches!(
        host.observe_native_input(&text_input("predecessor")),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));

    let _ = host.release_mechanical_host_session(HOST_SESSION);
    assert!(host
        .drain_mechanical_host_observations(HOST_SESSION)
        .unwrap()
        .into_batches()
        .is_empty());

    let successor_session = HOST_SESSION + 1;
    present_one(&host, successor_session);
    let retained = match host.observe_native_input(&text_input("successor")) {
        UiEguiRawInputIngressOutcome::Retained(retained) => retained,
        other => panic!("fresh session input must retain, got {other:?}"),
    };
    assert_eq!(retained.sequences().first().value(), 1);
    let batches = host
        .drain_mechanical_host_observations(successor_session)
        .unwrap()
        .into_batches();
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::TextInput { revision: 1, text }
            if text.as_ref() == "successor"
    ));
}

fn text_input(text: &str) -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Text(text.to_owned())],
        ..Default::default()
    }
}
