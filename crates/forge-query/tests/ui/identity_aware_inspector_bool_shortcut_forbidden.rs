use forge_query::facade::ViewShapeDescriptor;

fn main() {
    let _ = ViewShapeDescriptor::identity_aware_inspector_detail_focused(forge_foundational::facade::AspectKey::new("profile").unwrap(), true);
}
