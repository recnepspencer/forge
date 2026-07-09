use worth_store_aspect_native::StoreTerminalProjectionText;
use worth_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, SecureIoOperation, SecureIoPreservationRequest,
};

fn main() {
    let backend: IoSchedulerBackendCapabilityAdmission = todo!();
    let projection = StoreTerminalProjectionText::new_terminal_projection_text("terminal");
    let _ = SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &projection, &backend);
}
