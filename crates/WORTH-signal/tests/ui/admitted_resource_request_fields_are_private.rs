use worth_signal::facade::{
    AdmittedResourceRequest, ResourceAttemptId, ResourceRequestHandle,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn main() {
    let _ = AdmittedResourceRequest {
        handle: WORTHd_handle(),
        attempt: ResourceAttemptId::new(0),
    };
}
