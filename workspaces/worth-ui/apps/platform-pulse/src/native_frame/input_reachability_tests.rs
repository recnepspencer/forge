use eframe::egui;

use super::input::observe_egui_input;

#[test]
fn production_native_frame_route_reaches_exact_egui_input_families() {
    let host = worth_ui_host_egui::WorthUiHostEgui::default();
    let raw_input = egui::RawInput {
        events: vec![
            egui::Event::PointerButton {
                pos: egui::pos2(12.5, 34.25),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::Text("committed text".to_owned()),
            egui::Event::Ime(egui::ImeEvent::Preedit("preedit".to_owned())),
            egui::Event::Ime(egui::ImeEvent::Commit("commit".to_owned())),
            egui::Event::Ime(egui::ImeEvent::Disabled),
        ],
        ..Default::default()
    };

    let reachability =
        observe_egui_input(Some(&host), &raw_input).expect("the production host is installed");

    assert_eq!(reachability.event_count(), 6);
    assert_eq!(reachability.pointer_button_events(), 1);
    assert_eq!(reachability.keyboard_events(), 1);
    assert_eq!(reachability.text_events(), 1);
    assert_eq!(reachability.ime_preedit_events(), 1);
    assert_eq!(reachability.ime_commit_events(), 1);
    assert_eq!(reachability.ime_cancel_events(), 1);
}

#[test]
fn absent_host_cannot_turn_raw_input_into_reachability_evidence() {
    let raw_input = egui::RawInput::default();

    assert_eq!(observe_egui_input(None, &raw_input), None);
}
