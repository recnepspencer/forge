use crate::PhysicalSubstrateCloseoutDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCertificationDenial {
    RunIdentityRejected,
    PhysicalIdentifierRejected,
    PhysicalCellRejected,
    ReadinessRejected,
    FacadeOperationDenied,
    PlatformWitnessRejected,
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
