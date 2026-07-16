use worth_foundational::{
    FoundationalBoundaryEvidenceLineageConstructionDenial,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEvidenceDenial {
    PlannedRecoveryCannotMaterializeEvidence,
    CopiedReceiptFieldsCannotMaterializeEvidence,
    LogExcerptCannotMaterializeEvidence,
    SameRunSelfComparisonCannotMaterializeEvidence,
    JsonPayloadCannotMaterializeEvidence,
    RawBytesCannotMaterializeEvidence,
    DebugStringCannotMaterializeEvidence,
    DisplayNameCannotMaterializeEvidence,
    ProducerPrivateNameCannotMaterializeEvidence,
    MissingRecoveredState,
    MissingCounterSnapshot,
    OfflineVerifierDidNotVerify,
    OfflineVerifierStateDisagreesWithExecutedRecovery,
    OfflineVerifierCountersDisagreeWithExecutedRecovery,
    ProfileReductionChangedRecoveryTruth,
    CanonicalBasisMaterializationDenied,
    DiagnosticCertificationDenied,
    DiagnosticReadmissionDenied,
    PerformanceCertificationDenied,
    PerformanceReadmissionDenied,
    MissingStoreRecoveryAuthority,
    RawDigestCannotSatisfyCurrentBasis,
    BoundaryBridgedStaleFormRequiresReadmission,
    CurrentBasisAdmissionDenied,
    NonApplicableFoundationalSurface,
    EmptyProofCollection,
    NonCanonicalWalReplayOrder,
    DuplicateRecoverySourceFamily,
    FoundationalProvenanceConstructionDenied(
        FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    ),
    FoundationalLineageConstructionDenied(FoundationalBoundaryEvidenceLineageConstructionDenial),
}

impl From<FoundationalBoundaryEvidenceProvenanceConstructionDenial> for RecoveryEvidenceDenial {
    fn from(denial: FoundationalBoundaryEvidenceProvenanceConstructionDenial) -> Self {
        Self::FoundationalProvenanceConstructionDenied(denial)
    }
}

impl From<FoundationalBoundaryEvidenceLineageConstructionDenial> for RecoveryEvidenceDenial {
    fn from(denial: FoundationalBoundaryEvidenceLineageConstructionDenial) -> Self {
        Self::FoundationalLineageConstructionDenied(denial)
    }
}
