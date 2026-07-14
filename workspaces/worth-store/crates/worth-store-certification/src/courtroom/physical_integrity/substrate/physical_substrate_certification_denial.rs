use crate::PhysicalSubstrateCloseoutDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCertificationDenial {
    RunIdentityRejected,
    PhysicalIdentifierRejected,
    PhysicalCellRejected,
    ReadinessRejected,
    FacadeOperationDenied,
    PlatformWitnessRejected,
    PhysicalSubstrateHandoffEvidenceRejected,
    FacadeEvidenceRejected,
    OfflineVerifierEvidenceRejected,
    RuntimeVerifierComparisonDenied,
    RuntimeVerifierMismatchNotDetected,
    PageRecordEvidenceRejected,
    ExtentRecordEvidenceRejected,
    IdentityEvidenceRejected,
    ManifestEvidenceRejected,
    StoryDefinitionRejected,
    StoryPlanRejected,
    StoryEvidenceRejected,
    FoundationEvidenceRejected,
    ComplexityEvidenceRejected,
    HostileScaleFixtureRejected,
    CloseoutDenied(PhysicalSubstrateCloseoutDenial),
}
