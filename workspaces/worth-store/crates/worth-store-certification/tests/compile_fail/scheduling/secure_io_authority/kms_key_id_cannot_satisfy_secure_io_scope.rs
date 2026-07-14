use worth_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, SecureIoOperation, SecureIoPreservationRequest,
};
use worth_store_security::StoreKmsKeyIdentifier;

fn main() {
    let backend: IoSchedulerBackendCapabilityAdmission = todo!();
    let claim = StoreKmsKeyIdentifier::raw("kms-key-123");
    let _ = SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &claim, &backend);
}
