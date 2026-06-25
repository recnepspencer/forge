use worth_ui::facade::{WorthUiQueryGraphExecutionReceipt, WorthUiQueryGraphObligationSemantic};

pub fn support_status_for(
    receipt: &WorthUiQueryGraphExecutionReceipt,
    semantic: WorthUiQueryGraphObligationSemantic,
) -> &str {
    receipt
        .rows()
        .iter()
        .find(|row| row.semantic() == semantic)
        .expect("query graph row exists")
        .support_status()
}
