use forge_store_security::{reject_non_store_security_scope_source, StoreSecurityAuthoritySource};

use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerSecurityScopeAdmission,
};

use super::{
    SecureIoOperation, SecureIoPosture, SecureIoPostureRequirement,
    SecureIoPreservationCounterSnapshot, SecureIoPreservationDenial, SecureIoPreservationReceipt,
    SecureIoScopeBasis,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecureIoPreservationRequest<'a> {
    operation: SecureIoOperation,
    security_scope: &'a IoSchedulerSecurityScopeAdmission,
    backend: &'a IoSchedulerBackendCapabilityAdmission,
    required_backend: IoSchedulerBackendCapabilityRequirement,
    posture: SecureIoPostureRequirement,
}

impl<'a> SecureIoPreservationRequest<'a> {
    pub const fn new(
        operation: SecureIoOperation,
        security_scope: &'a IoSchedulerSecurityScopeAdmission,
        backend: &'a IoSchedulerBackendCapabilityAdmission,
    ) -> Self {
        Self {
            operation,
            security_scope,
            backend,
            required_backend: backend.requirement(),
            posture: SecureIoPostureRequirement::ScopePreserving,
        }
    }

    pub const fn require_backend(
        mut self,
        required_backend: IoSchedulerBackendCapabilityRequirement,
    ) -> Self {
        self.required_backend = required_backend;
        self
    }

    pub const fn require_posture(mut self, posture: SecureIoPostureRequirement) -> Self {
        self.posture = posture;
        self
    }
}

pub fn admit_secure_io_scope_for_scheduler(
    request: SecureIoPreservationRequest<'_>,
) -> Result<SecureIoPreservationReceipt, SecureIoPreservationDenial> {
    let mut counters = SecureIoPreservationCounterSnapshot::start().checked_scope();
    if request.backend.requirement() != request.required_backend {
        return Err(SecureIoPreservationDenial::BackendRequirementMismatch {
            required: request.required_backend,
            admitted: request.backend.requirement(),
        });
    }
    counters = counters.checked_backend_posture();
    let posture = match request.posture {
        SecureIoPostureRequirement::ScopePreserving => SecureIoPosture::ScopePreserving,
        SecureIoPostureRequirement::SecureFrameCompatible => {
            if request.backend.requirement()
                != IoSchedulerBackendCapabilityRequirement::SecureFrameIo
            {
                return Err(SecureIoPreservationDenial::UnsupportedSecureIoPosture {
                    operation: request.operation,
                    requirement: request.backend.requirement(),
                });
            }
            if !request.backend.security_scope_bound() {
                return Err(SecureIoPreservationDenial::SecureIoRequiresSecurityBoundBackend);
            }
            SecureIoPosture::SecureFrameCompatible
        }
    };
    Ok(SecureIoPreservationReceipt::new(
        request.operation,
        SecureIoScopeBasis::from_s5_1_admission(request.security_scope.receipt()),
        request.backend.requirement(),
        request.backend.profile(),
        request.backend.evidence_class(),
        posture,
        counters,
    ))
}

pub const fn reject_lower_authority_secure_io_scope_source(
    source: StoreSecurityAuthoritySource,
) -> Result<(), SecureIoPreservationDenial> {
    Err(
        SecureIoPreservationDenial::LowerAuthoritySecurityScopeSource(
            reject_non_store_security_scope_source(source),
        ),
    )
}
