use worth_ui::facade::WorthUiLiveViewDeclarationReceipt;

fn main() {
    let _forged = WorthUiLiveViewDeclarationReceipt {
        live_view_id: "validation.live_view.primitive_proof".to_owned(),
        target_binding: panic!("fixture only checks receipt field privacy"),
        bindings: Vec::new(),
        graph_execution: panic!("fixture only checks receipt field privacy"),
        counters: panic!("fixture only checks receipt field privacy"),
        declaration_digest: 1,
    };
}
