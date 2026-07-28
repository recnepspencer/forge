use worth_ui::facade::inspection::{
    UiClientPhysicalRect, UiGeometryOnly, UiVisualSnapshotReceipt,
};

fn invalid(first: &UiVisualSnapshotReceipt<UiGeometryOnly>) {
    let _escaped = first.with_coordinate_scope(|scope| {
        scope.client_region(UiClientPhysicalRect::new(0, 0, 8, 8).unwrap())
    });
}

fn main() {}
