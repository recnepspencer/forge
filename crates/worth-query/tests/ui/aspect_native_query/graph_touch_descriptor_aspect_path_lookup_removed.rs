use worth_query::facade::WorthQueryGraphTouchDescriptor;

fn main() {
    let descriptor = descriptor_fixture();
    let _ = descriptor.touches_aspect_path("identity.id");
}

fn descriptor_fixture() -> WorthQueryGraphTouchDescriptor {
    panic!("fixture only")
}
