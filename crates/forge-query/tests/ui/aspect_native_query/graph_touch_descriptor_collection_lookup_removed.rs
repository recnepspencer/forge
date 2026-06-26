use forge_query::facade::ForgeQueryGraphTouchDescriptor;

fn main() {
    let descriptor = descriptor_fixture();
    let _ = descriptor.touches_collection("Task");
}

fn descriptor_fixture() -> ForgeQueryGraphTouchDescriptor {
    panic!("fixture only")
}
