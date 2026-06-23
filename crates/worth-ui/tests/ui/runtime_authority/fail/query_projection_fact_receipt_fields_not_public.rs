use worth_ui::facade::WorthUiQueryProjectionFactReceipt;

fn main() {
    let _ = WorthUiQueryProjectionFactReceipt {
        receipt_identity: "validation.query.products.rows".to_owned(),
        receipt_digest: 42,
    };
}
