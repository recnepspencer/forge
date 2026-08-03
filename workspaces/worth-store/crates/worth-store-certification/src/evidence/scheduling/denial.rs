use super::S6CanonicalMaterializationDenial;
use crate::FoundationalPerformanceEvidenceDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6CertificationMaterializationDenial {
    BackendAdmissionReadinessMismatch,
    StoreEvidenceBackendBindingMismatch,
    StoreEvidenceSecurityScopeBindingMismatch,
    StoreEvidenceReadmissionBindingMismatch,
    ForegroundReservation(S6ForegroundReservationCertificationDenial),
    QueueExecution(S6QueueExecutionCertificationDenial),
    MissingAccessPolicyEvidence,
    MissingPostAdmissionViolationEvidence,
    MissingSecureIoPreservationEvidence,
    MissingFlushDurabilityEvidence,
    EmptyQualificationMatrix,
    MissingHarnessReplayEvidence,
    FoundationalPerformance(FoundationalPerformanceEvidenceDenial),
    Canonical(S6CanonicalMaterializationDenial),
}

use crate::{S6ForegroundReservationCertificationDenial, S6QueueExecutionCertificationDenial};

impl From<S6ForegroundReservationCertificationDenial> for S6CertificationMaterializationDenial {
    fn from(denial: S6ForegroundReservationCertificationDenial) -> Self {
        Self::ForegroundReservation(denial)
    }
}

impl From<S6QueueExecutionCertificationDenial> for S6CertificationMaterializationDenial {
    fn from(denial: S6QueueExecutionCertificationDenial) -> Self {
        Self::QueueExecution(denial)
    }
}

impl From<FoundationalPerformanceEvidenceDenial> for S6CertificationMaterializationDenial {
    fn from(denial: FoundationalPerformanceEvidenceDenial) -> Self {
        Self::FoundationalPerformance(denial)
    }
}

impl From<S6CanonicalMaterializationDenial> for S6CertificationMaterializationDenial {
    fn from(denial: S6CanonicalMaterializationDenial) -> Self {
        Self::Canonical(denial)
    }
}
