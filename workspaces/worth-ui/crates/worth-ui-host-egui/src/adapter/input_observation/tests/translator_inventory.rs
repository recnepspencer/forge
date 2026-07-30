use worth_ui_host_contract::{WorthUiHostCapability, WorthUiHostMechanicsAdapter};

use super::{assert_stop, initialized_host, key_event, pointer_button, present_one, HOST_SESSION};
use crate::adapter::{UiEguiInputTranslatorFamily, UiEguiRawInputIngressStopReason};

#[test]
fn removing_each_translator_removes_its_capability_and_stops_its_event_family() {
    let production = initialized_host().mechanical_capability_report();
    for capability in [
        WorthUiHostCapability::PointerInput,
        WorthUiHostCapability::KeyboardInput,
        WorthUiHostCapability::TextInput,
        WorthUiHostCapability::Ime,
    ] {
        assert!(production.supports(capability));
    }
    let cases = [
        (
            UiEguiInputTranslatorFamily::Pointer,
            WorthUiHostCapability::PointerInput,
            pointer_button(),
        ),
        (
            UiEguiInputTranslatorFamily::Keyboard,
            WorthUiHostCapability::KeyboardInput,
            key_event(),
        ),
        (
            UiEguiInputTranslatorFamily::Text,
            WorthUiHostCapability::TextInput,
            egui::Event::Text("text".to_owned()),
        ),
        (
            UiEguiInputTranslatorFamily::Ime,
            WorthUiHostCapability::Ime,
            egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "draft".to_owned(),
                active_range_chars: Some(0..3),
            }),
        ),
    ];

    for (family, capability, event) in cases {
        let host = initialized_host().without_input_translator_for_testing(family);
        let reduced = host.mechanical_capability_report();
        assert!(!reduced.supports(capability));
        assert_eq!(
            [
                WorthUiHostCapability::PointerInput,
                WorthUiHostCapability::KeyboardInput,
                WorthUiHostCapability::TextInput,
                WorthUiHostCapability::Ime,
            ]
            .into_iter()
            .filter(|candidate| reduced.supports(*candidate))
            .count(),
            3
        );
        present_one(&host, HOST_SESSION);

        assert_stop(
            host.observe_native_input(&egui::RawInput {
                events: vec![event],
                ..Default::default()
            }),
            UiEguiRawInputIngressStopReason::TranslatorUnavailable { index: 0, family },
        );
        assert!(host
            .drain_mechanical_host_observations(HOST_SESSION)
            .unwrap()
            .into_batches()
            .is_empty());
    }
}
