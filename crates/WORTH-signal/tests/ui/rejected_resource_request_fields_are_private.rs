use worth_signal::facade::{
    RejectedResourceRequest, ResourceLifecycleTransition, ResourceNodeId, ResourceRejectionOrdinal,
    ResourceRejectionReason, ResourceRequestHandle,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn WORTHd_node() -> ResourceNodeId {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn main() {
    let _ = RejectedResourceRequest {
        handle: WORTHd_handle(),
        node: WORTHd_node(),
        rejection_ordinal: ResourceRejectionOrdinal::new(0),
        reason: ResourceRejectionReason::SemanticFailure,
        lifecycle_transition: WORTHd_transition(),
    };
}
