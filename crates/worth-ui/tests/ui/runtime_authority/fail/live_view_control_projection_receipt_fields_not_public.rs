use worth_ui::facade::WorthUiLiveViewControlProjectionReceipt;

fn main() {
    let _forged = WorthUiLiveViewControlProjectionReceipt {
        live_view_id: "validation.live_view.primitive_proof".to_owned(),
        control_id: "contact_mode_input".to_owned(),
        component_id: panic!("fixture only checks receipt field privacy"),
        binding: panic!("fixture only checks receipt field privacy"),
        kind: panic!("fixture only checks receipt field privacy"),
        label: "Contact mode".to_owned(),
        options: None,
        graph_execution: panic!("fixture only checks receipt field privacy"),
        control_projection_digest: 1,
    };
}
