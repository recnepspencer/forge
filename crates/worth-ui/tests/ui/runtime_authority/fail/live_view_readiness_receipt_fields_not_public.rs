use worth_ui::facade::WorthUiLiveViewReadinessProjectionReceipt;

fn main() {
    let _forged = WorthUiLiveViewReadinessProjectionReceipt {
        live_view_id: String::new(),
        target_binding: panic!("fixture only checks receipt field privacy"),
        readiness_id: String::new(),
        required_bindings: Vec::new(),
        posture: panic!("fixture only checks receipt field privacy"),
        graph_execution: panic!("fixture only checks receipt field privacy"),
        readiness_digest: 1,
    };
}
