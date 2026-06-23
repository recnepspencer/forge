use worth_ui::facade::WorthUiQueryStateSnapshotReceipt;

fn main() {
    let _ = WorthUiQueryStateSnapshotReceipt {
        receipt_identity: "validation.query.state.product_filters".to_owned(),
        receipt_digest: 42,
    };
}
