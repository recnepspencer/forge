use forge_signal::facade::{ResourceInitialLifecycleClass, ResourceLifecycleClass};

fn main() {
    let _ = ResourceInitialLifecycleClass {
        lifecycle: ResourceLifecycleClass::Pending,
    };
}
