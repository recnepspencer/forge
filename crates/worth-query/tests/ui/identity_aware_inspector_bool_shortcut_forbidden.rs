use worth_query::facade::ViewShapeDescriptor;

fn main() {
    let _ = ViewShapeDescriptor::identity_aware_inspector_detail_focused(worth_foundational::facade::AspectKey::new("profile").unwrap(), true);
}
