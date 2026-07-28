use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiGeometryOnly, UiVisualSnapshotReceipt,
};

fn invalid(first: &UiVisualSnapshotReceipt<UiGeometryOnly>) {
    let _escaped = first.with_coordinate_scope(|scope| {
        scope.client_pixel(UiClientPhysicalPixel::new(80, 48).unwrap())
    });
}

fn main() {}
