use worth_ui::facade::{WorthUiCapabilityReloadRequest, WorthUiRuntimeHost};

fn main() {
    let request = WorthUiCapabilityReloadRequest::batch([]);
    let runtime = forged_runtime();
    runtime.admit_capability_runtime_change(&request);
}

fn forged_runtime() -> WorthUiRuntimeHost {
    panic!("fixture should fail before runtime construction")
}
