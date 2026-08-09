use worth_ui_runtime::facade::mounted::UiMountedSurfaceBaselineReceipt;

fn duplicate(receipt: UiMountedSurfaceBaselineReceipt) {
    let _first = receipt.clone();
    let _second = receipt;
}

fn main() {}
