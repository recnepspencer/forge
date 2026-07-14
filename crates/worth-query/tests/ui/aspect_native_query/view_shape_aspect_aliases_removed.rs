use worth_query::facade::runtime::{FocusedInspectorAspectPatchArtifact, ViewShapeDeliveryMetadata, ViewShapeDescriptor};

fn main() {
    let descriptor = ViewShapeDescriptor::terminal_inspector_detail_focused("profile");
    let _ = descriptor.focused_aspect();
    let _ = descriptor.grouping_aspect();

    let delivery = delivery_fixture();
    let _ = delivery.focus_aspect();
    let _ = delivery.grouping_aspect();

    let patch = focused_patch_fixture();
    let _ = patch.focus_aspect();
}

fn delivery_fixture() -> ViewShapeDeliveryMetadata {
    panic!("fixture only")
}

fn focused_patch_fixture() -> FocusedInspectorAspectPatchArtifact {
    panic!("fixture only")
}
