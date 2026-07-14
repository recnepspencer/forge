use worth_query::facade::runtime::{ViewShapeDescriptor, ViewShapeIdentityConsumption};

fn main() {
    let mut descriptor = ViewShapeDescriptor::inspector_detail_focused(worth_foundational::facade::AspectKey::new("profile").unwrap());
    descriptor.identity_consumption =
        ViewShapeIdentityConsumption::inspector_identity_summary();
}
