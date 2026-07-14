use worth_foundational::{
    CanonicalBasisConstructionDenial, FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
};
use worth_store_physical_certification::{
    SecurityScopeFailureKind, SecurityScopeHarnessSchedule, SecurityScopeReplayMutationKind,
};
use worth_store_readiness::{
    FoundationalAdoptionDenial, FoundationalAdoptionFamily, PhysicalFoundationEvidenceField,
};

use crate::FoundationalBoundaryEvidenceDenial;

#[derive(Debug, PartialEq, Eq)]
pub enum S51CertificationCloseoutDenial {
    MissingScenarioEvidence(SecurityScopeFailureKind),
    MissingReplayTranscript {
        schedule: SecurityScopeHarnessSchedule,
        mutation: SecurityScopeReplayMutationKind,
    },
    ReplayTranscriptNotSamePhysicalSchedule,
    ReplayTranscriptDidNotDenyBeforeLogicalDecode,
    CounterMismatch {
        counter: &'static str,
        expected: u64,
        observed: u64,
    },
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
