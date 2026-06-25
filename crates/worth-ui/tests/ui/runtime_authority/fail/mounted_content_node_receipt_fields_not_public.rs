use worth_ui::facade::{WorthUiMountedContentNodeReceipt, WorthUiPrimitiveContentReceipt};

fn main() {
    let _forged = WorthUiMountedContentNodeReceipt {
        node_id: "content".to_owned(),
        content: forged_content(),
        semantic_slice: "PrimitiveContent",
        receipt_digest: 1,
    };
}

fn forged_content() -> WorthUiPrimitiveContentReceipt {
    panic!("not executed by compile-fail test")
}
