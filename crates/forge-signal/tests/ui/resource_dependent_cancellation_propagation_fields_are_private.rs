use forge_signal::facade::{CancelledResourceRequest, ResourceDependentCancellationPropagation};

fn forged_cancelled() -> CancelledResourceRequest {
    loop {}
}

fn forged_parent() -> forge_signal::facade::ResourceRequestHandle {
    loop {}
}

fn main() {
    let _ = ResourceDependentCancellationPropagation {
        parent: forged_parent(),
        cancelled_dependents: vec![forged_cancelled()],
    };
}
