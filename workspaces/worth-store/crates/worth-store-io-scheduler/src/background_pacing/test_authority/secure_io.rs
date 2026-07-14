use crate::{
    admit_secure_io_scope_for_scheduler, BackgroundIoPressureClass, BackgroundIoPressureShape,
    IoSchedulerBackendCapabilityAdmission, IoSchedulerSecurityScopeAdmission, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationRequest,
};

pub(super) fn secure_io_for_pressure(
    pressure: BackgroundIoPressureShape,
    backend: &IoSchedulerBackendCapabilityAdmission,
    security: &IoSchedulerSecurityScopeAdmission,
) -> crate::SecureIoPreservationReceipt {
    let operation = match pressure.class() {
        BackgroundIoPressureClass::VerificationPressure => SecureIoOperation::VerificationPressure,
        BackgroundIoPressureClass::RepairScan => SecureIoOperation::RepairScan,
        _ => SecureIoOperation::BackgroundLease,
    };
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(operation, security, backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("background pressure secure I/O should admit")
}
