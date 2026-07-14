use worth_query::facade::foundation::DeclarativeLiveViewShape;
use worth_query::facade::runtime::ViewShapeDescriptor;

fn main() {
    let _ = ViewShapeDescriptor::inspector_detail_focused("profile");
    let _ = DeclarativeLiveViewShape::inspector_focused("profile");
}
