use worth_signal::facade::{
    ResourceAttemptId, ResourceRequestHandle, ValidatedCompletionEnvelope,
};

fn WORTHd_handle() -> ResourceRequestHandle {
    loop {}
}

fn main() {
    let _ = ValidatedCompletionEnvelope {
        handle: WORTHd_handle(),
        attempt: ResourceAttemptId::new(0),
        payload_byte_len: 0,
    };
}
