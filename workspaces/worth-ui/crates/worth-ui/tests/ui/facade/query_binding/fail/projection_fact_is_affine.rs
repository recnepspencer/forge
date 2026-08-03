use worth_ui::facade::query_binding::UiProjectionFactReceipt;

fn invalid(receipt: &UiProjectionFactReceipt) {
    let _copy: UiProjectionFactReceipt = receipt.clone();
}

fn main() {}
