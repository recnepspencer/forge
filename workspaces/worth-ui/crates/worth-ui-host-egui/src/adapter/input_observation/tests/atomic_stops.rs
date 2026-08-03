use worth_ui_host_contract::{UiHostObservationPayload, WorthUiHostMechanicsAdapter};

use super::{initialized_host, present_one, HOST_SESSION};
use crate::adapter::{
    UiEguiRawInputIngressOutcome, UiEguiRawInputIngressStopReason, UiEguiUnsupportedEventFamily,
};

#[test]
fn unsupported_event_stops_at_its_source_index_and_rolls_back_the_whole_batch() {
    let host = initialized_host();
    present_one(&host, HOST_SESSION);
    let stopped = host.observe_native_input(&egui::RawInput {
        events: vec![
            egui::Event::Text("before".to_owned()),
            egui::Event::Copy,
            egui::Event::Text("after".to_owned()),
        ],
        ..Default::default()
    });
    let UiEguiRawInputIngressOutcome::Stopped(stop) = stopped else {
        panic!("unsupported event must stop the batch");
    };
    assert_eq!(
        stop.reason(),
        UiEguiRawInputIngressStopReason::UnsupportedEvent {
            index: 1,
            family: UiEguiUnsupportedEventFamily::Copy,
        }
    );
    assert_eq!(stop.reachability().event_count(), 3);
    assert_eq!(stop.reachability().text_events(), 2);
    assert!(host
        .drain_mechanical_host_observations(HOST_SESSION)
        .unwrap()
        .into_batches()
        .is_empty());

    assert!(matches!(
        host.observe_native_input(&egui::RawInput {
            events: vec![egui::Event::Text("retry".to_owned())],
            ..Default::default()
        }),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));
    let batches = host
        .drain_mechanical_host_observations(HOST_SESSION)
        .unwrap()
        .into_batches();
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::TextInput { revision: 1, text }
            if text.as_ref() == "retry"
    ));
}
