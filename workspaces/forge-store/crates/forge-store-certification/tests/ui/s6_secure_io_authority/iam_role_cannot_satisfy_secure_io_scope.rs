use forge_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, SecureIoOperation, SecureIoPreservationRequest,
};
use forge_store_security::StoreIamRoleClaim;

fn main() {
    let backend: IoSchedulerBackendCapabilityAdmission = todo!();
    let claim = StoreIamRoleClaim::raw("arn:aws:iam::123:role/store");
    let _ = SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &claim, &backend);
}
