use forge_query::facade::{ViewShapeDescriptor, ViewShapeIdentityConsumption};

fn main() {
    let mut descriptor = ViewShapeDescriptor::inspector_detail_focused("profile");
    descriptor.identity_consumption =
        ViewShapeIdentityConsumption::inspector_identity_summary();
}
