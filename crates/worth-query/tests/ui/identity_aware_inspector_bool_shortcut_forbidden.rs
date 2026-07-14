use worth_query::facade::runtime::ViewShapeDescriptor;

fn main() {
    let _ = ViewShapeDescriptor::identity_aware_inspector_detail_focused(worth_foundational::facade::AspectKey::new("profile").unwrap(), true);
}
