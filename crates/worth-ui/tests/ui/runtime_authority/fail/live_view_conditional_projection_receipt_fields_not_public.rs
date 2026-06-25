use worth_ui::facade::WorthUiLiveViewConditionalProjectionReceipt;

fn main() {
    let _forged = WorthUiLiveViewConditionalProjectionReceipt {
        live_view_id: "validation.live_view.primitive_proof".to_owned(),
        control: panic!("fixture only checks receipt field privacy"),
        condition: panic!("fixture only checks receipt field privacy"),
        consumed_binding: panic!("fixture only checks receipt field privacy"),
        participation: panic!("fixture only checks receipt field privacy"),
        graph_execution: panic!("fixture only checks receipt field privacy"),
        conditional_projection_digest: 1,
    };
}
