use worth_query::facade::runtime::WorthQueryGraphTouchDescriptor;

fn main() {
    let descriptor = descriptor_fixture();
    let _ = descriptor.touches_collection("Task");
}

fn descriptor_fixture() -> WorthQueryGraphTouchDescriptor {
    panic!("fixture only")
}
