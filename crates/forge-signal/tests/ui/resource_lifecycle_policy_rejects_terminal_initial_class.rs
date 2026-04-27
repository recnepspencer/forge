use forge_signal::facade::{ResourceLifecycleClass, ResourceLifecyclePolicyDeclaration};

fn main() {
    let _ = ResourceLifecyclePolicyDeclaration::new(ResourceLifecycleClass::Fulfilled);
}
