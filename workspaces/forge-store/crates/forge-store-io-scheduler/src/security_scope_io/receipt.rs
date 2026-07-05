use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
};

use crate::IoSchedulerBackendCapabilityRequirement;

use super::{SecureIoOperation, SecureIoPosture, SecureIoPreservationCounterSnapshot};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecureIoScopeBasis {
    admission_receipt: StoreSecurityScopeAdmissionReceipt,
    identity: StoreSecurityScopeIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecureIoPreservationReceipt {
    operation: SecureIoOperation,
    basis: SecureIoScopeBasis,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    posture: SecureIoPosture,
    counters: SecureIoPreservationCounterSnapshot,
}

impl SecureIoScopeBasis {
    pub const fn from_s5_1_admission(receipt: StoreSecurityScopeAdmissionReceipt) -> Self {
        Self {
            admission_receipt: receipt,
            identity: receipt.identity(),
        }
    }

    pub const fn admission_receipt(self) -> StoreSecurityScopeAdmissionReceipt {
        self.admission_receipt
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.identity.authenticity_requirement()
    }
}

impl SecureIoPreservationReceipt {
    pub(crate) const fn new(
        operation: SecureIoOperation,
        basis: SecureIoScopeBasis,
        backend_requirement: IoSchedulerBackendCapabilityRequirement,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
        posture: SecureIoPosture,
        counters: SecureIoPreservationCounterSnapshot,
    ) -> Self {
        Self {
            operation,
            basis,
            backend_requirement,
            backend_profile,
            backend_evidence_class,
            posture,
            counters,
        }
    }

    pub const fn operation(self) -> SecureIoOperation {
        self.operation
    }

    pub const fn basis(self) -> SecureIoScopeBasis {
        self.basis
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.basis.identity()
    }

    pub const fn backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn posture(self) -> SecureIoPosture {
        self.posture
    }

    pub const fn counters(self) -> SecureIoPreservationCounterSnapshot {
        self.counters
    }
}
