use worth_signal::facade::{
    CancelledResourceRequest, ResourceCancellationOrdinal, ResourceCancellationReason,
    ResourceCancellationGraceWindow, ResourceHostCancellationAdvisory,
    ResourceLifecycleTransition, ResourcePolicyDigest, ResourceRequestHandle,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn WORTHd_host_advisory() -> ResourceHostCancellationAdvisory {
    loop {}
}

fn WORTHd_grace_window() -> ResourceCancellationGraceWindow {
    loop {}
}

fn WORTHd_digest() -> ResourcePolicyDigest {
    loop {}
}

fn main() {
    let _ = CancelledResourceRequest {
        handle: WORTHd_handle(),
        cancellation_ordinal: ResourceCancellationOrdinal::new(0),
        reason: ResourceCancellationReason::HostRequested,
        policy_decision_digest: WORTHd_digest(),
        host_advisory: Some(WORTHd_host_advisory()),
        grace_window: Some(WORTHd_grace_window()),
        lifecycle_transition: WORTHd_transition(),
    };
}
