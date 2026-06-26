use forge_query::facade::{DeclarativeLiveViewShape, ViewShapeDescriptor};

fn main() {
    let _ = ViewShapeDescriptor::inspector_detail_focused("profile");
    let _ = DeclarativeLiveViewShape::inspector_focused("profile");
}
