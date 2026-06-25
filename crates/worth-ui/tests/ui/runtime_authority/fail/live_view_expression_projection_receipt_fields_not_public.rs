use worth_ui::facade::WorthUiLiveViewExpressionProjectionReceipt;

fn main() {
    let _forged = WorthUiLiveViewExpressionProjectionReceipt {
        live_view_id: String::new(),
        expression_id: String::new(),
        operator: panic!("fixture only checks receipt field privacy"),
        consumed_facts: Vec::new(),
        output: panic!("fixture only checks receipt field privacy"),
        graph_execution: panic!("fixture only checks receipt field privacy"),
        expression_digest: 1,
    };
}
