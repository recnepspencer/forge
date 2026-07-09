use worth_signal::facade::{CancelledResourceRequest, ResourceDependentCancellationPropagation};

fn WORTHd_cancelled() -> CancelledResourceRequest {
    loop {}
}

fn WORTHd_parent() -> worth_signal::facade::ResourceRequestHandle {
    loop {}
}

fn main() {
    let _ = ResourceDependentCancellationPropagation {
        parent: WORTHd_parent(),
        cancelled_dependents: vec![WORTHd_cancelled()],
    };
}
