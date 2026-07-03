use forge_foundational::{
    CanonicalBasisConstructionDenial, FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
};
use forge_store_physical_certification::{
    S51SecurityScopeFailureKind, S51SecurityScopeHarnessSchedule,
    S51SecurityScopeReplayMutationKind,
};
use forge_store_readiness::{
    FoundationalAdoptionDenial, FoundationalAdoptionFamily, PhysicalFoundationEvidenceField,
    S51SecurityFoundationNonClaim,
};

use crate::FoundationalBoundaryEvidenceDenial;

#[derive(Debug, PartialEq, Eq)]
pub enum S51CertificationCloseoutDenial {
    MissingScenarioEvidence(S51SecurityScopeFailureKind),
    MissingReplayTranscript {
        schedule: S51SecurityScopeHarnessSchedule,
        mutation: S51SecurityScopeReplayMutationKind,
    },
    ReplayTranscriptNotSamePhysicalSchedule,
    ReplayTranscriptDidNotDenyBeforeLogicalDecode,
    CounterMismatch {
        counter: &'static str,
        expected: u64,
        observed: u64,
    },
    MissingSecurityFoundationNonClaim(S51SecurityFoundationNonClaim),
    FoundationalAdoptionDenied(FoundationalAdoptionDenial),
    MissingFoundationalAdoptionFamily(FoundationalAdoptionFamily),
    MissingFoundationalEvidenceField(PhysicalFoundationEvidenceField),
    BoundaryPerformanceDenied(FoundationalBoundaryEvidenceDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
    FoundationalBoundaryProvenanceDenied(FoundationalBoundaryEvidenceProvenanceConstructionDenial),
    FoundationalCanonicalBasisDenied(CanonicalBasisConstructionDenial),
}

impl From<FoundationalAdoptionDenial> for S51CertificationCloseoutDenial {
    fn from(denial: FoundationalAdoptionDenial) -> Self {
        Self::FoundationalAdoptionDenied(denial)
    }
}

impl From<FoundationalCounterBackedPerformanceReceiptConstructionDenial>
    for S51CertificationCloseoutDenial
{
    fn from(denial: FoundationalCounterBackedPerformanceReceiptConstructionDenial) -> Self {
        Self::PerformanceReceiptDenied(denial)
    }
}

impl From<FoundationalBoundaryEvidenceDenial> for S51CertificationCloseoutDenial {
    fn from(denial: FoundationalBoundaryEvidenceDenial) -> Self {
        Self::BoundaryPerformanceDenied(denial)
    }
}

impl From<FoundationalBoundaryEvidenceProvenanceConstructionDenial>
    for S51CertificationCloseoutDenial
{
    fn from(denial: FoundationalBoundaryEvidenceProvenanceConstructionDenial) -> Self {
        Self::FoundationalBoundaryProvenanceDenied(denial)
    }
}

impl From<CanonicalBasisConstructionDenial> for S51CertificationCloseoutDenial {
    fn from(denial: CanonicalBasisConstructionDenial) -> Self {
        Self::FoundationalCanonicalBasisDenied(denial)
    }
}
