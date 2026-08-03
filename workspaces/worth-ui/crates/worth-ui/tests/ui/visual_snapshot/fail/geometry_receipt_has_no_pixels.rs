use worth_ui::facade::inspection::{UiGeometryOnly, UiVisualSnapshotReceipt};

fn invalid(receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>) {
    let _ = receipt.pixel_artifact();
}

fn main() {}
