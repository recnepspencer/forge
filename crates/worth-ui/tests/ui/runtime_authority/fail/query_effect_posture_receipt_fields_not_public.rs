use worth_ui::facade::WorthUiQueryEffectPostureReceipt;

fn main() {
    let _ = WorthUiQueryEffectPostureReceipt {
        receipt_identity: "validation.query.effects.save_product".to_owned(),
        receipt_digest: 42,
    };
}
