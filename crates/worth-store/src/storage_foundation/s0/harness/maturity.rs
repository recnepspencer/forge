use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HarnessMaturityLevel {
    Missing,
    Exists,
    SmokeWorks,
    CiCertifiable,
    ReleaseCertifiable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HarnessSubsystemMaturity {
    TerminologyClaimGate,
    BackendTierFenceEnforcement,
    DeferredGuaranteeValidation,
    MilestoneStatusCompleteness,
    CompileTimeBoundaryFixtures,
    StaleHandoffRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvidenceBundleReadiness {
    Insufficient,
    ReadyForS1Planning,
    ReadyForS1Closeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForbiddenShortcutDetectionStatus {
    Missing,
    Exists,
    CiEnforced,
}
