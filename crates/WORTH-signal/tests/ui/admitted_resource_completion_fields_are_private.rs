use worth_signal::facade::{
    AdmittedResourceCompletion, ResourceCompletionOrdinal, ResourceDescriptorId,
    ResourceLifecycleTransition, ResourceNodeId, ResourceRequestHandle,
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
    let _ = AdmittedResourceCompletion {
        handle: WORTHd_handle(),
        node: WORTHd_node(),
        descriptor_id: ResourceDescriptorId::new(0),
        completion_ordinal: ResourceCompletionOrdinal::new(0),
        payload_byte_len: 0,
        lifecycle_transition: WORTHd_transition(),
    };
}
