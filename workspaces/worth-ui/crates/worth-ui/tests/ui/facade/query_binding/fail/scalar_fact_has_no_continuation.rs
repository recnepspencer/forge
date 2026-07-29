use worth_ui::facade::query_binding::UiScalarProjectionFactReceipt;

fn invalid(receipt: &UiScalarProjectionFactReceipt) {
    let _ = receipt.continuation();
}

fn main() {}
