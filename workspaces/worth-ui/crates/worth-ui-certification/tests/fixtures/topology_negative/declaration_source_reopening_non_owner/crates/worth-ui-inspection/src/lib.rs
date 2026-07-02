use worth_ui_dsl::UiDslLoweringReceipt;

pub fn forbidden_declaration_source_reopening(receipt: &UiDslLoweringReceipt) -> usize {
    receipt.semantic_artifact().published_aspects().len()
}
