use worth_store_security::StoreSecurityScopeDenial;

use crate::IoSchedulerBackendCapabilityRequirement;

use super::SecureIoOperation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureIoPreservationDenial {
    LowerAuthoritySecurityScopeSource(StoreSecurityScopeDenial),
    BackendRequirementMismatch {
        required: IoSchedulerBackendCapabilityRequirement,
        admitted: IoSchedulerBackendCapabilityRequirement,
    },
    SecureIoRequiresSecurityBoundBackend,
    UnsupportedSecureIoPosture {
        operation: SecureIoOperation,
        requirement: IoSchedulerBackendCapabilityRequirement,
    },
    ScopeMismatch {
        operation: SecureIoOperation,
    },
    SpeculativeScopeMismatch {
        operation: SecureIoOperation,
    },
    OperationMismatch {
        expected: SecureIoOperation,
        actual: SecureIoOperation,
    },
}
