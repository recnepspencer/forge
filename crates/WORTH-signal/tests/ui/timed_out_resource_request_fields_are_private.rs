use worth_signal::facade::{
    ReadyTemporalWake, ResourceLifecycleTransition, ResourceRequestHandle, ResourceTimeoutOrdinal,
    TimedOutResourceRequest,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn WORTHd_ready_wake() -> ReadyTemporalWake {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn main() {
    let _ = TimedOutResourceRequest {
        handle: WORTHd_handle(),
        timeout_ordinal: ResourceTimeoutOrdinal::new(0),
        ready_wake: WORTHd_ready_wake(),
        lifecycle_transition: WORTHd_transition(),
    };
}
