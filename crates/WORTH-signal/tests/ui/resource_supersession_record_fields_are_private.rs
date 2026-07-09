use worth_signal::facade::{
    ResourceLifecycleTransition, ResourceRequestHandle, ResourceSupersessionOrdinal,
    ResourceSupersessionRecord,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn main() {
    let _ = ResourceSupersessionRecord {
        supersession_ordinal: ResourceSupersessionOrdinal::new(0),
        previous: WORTHd_handle(),
        replacing: WORTHd_handle(),
        lifecycle_transition: WORTHd_transition(),
    };
}
