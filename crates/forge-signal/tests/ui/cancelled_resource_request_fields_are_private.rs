use forge_signal::facade::{
    CancelledResourceRequest, ResourceCancellationOrdinal, ResourceCancellationReason,
    ResourceLifecycleTransition, ResourceRequestHandle,
};

fn forged_handle() -> ResourceRequestHandle {
    loop {}
}

fn forged_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn main() {
    let _ = CancelledResourceRequest {
        handle: forged_handle(),
        cancellation_ordinal: ResourceCancellationOrdinal::new(0),
        reason: ResourceCancellationReason::HostRequested,
        lifecycle_transition: forged_transition(),
    };
}
