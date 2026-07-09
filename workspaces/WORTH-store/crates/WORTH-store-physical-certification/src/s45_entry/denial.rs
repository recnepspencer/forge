use super::requirement_set::S45RoadmapHarnessRequirement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S45HarnessBoundaryDenial {
    CopiedS4ReportCannotAdmitEntry,
    LogOutputCannotAdmitEntry,
    OldSemanticHarnessContextCannotAdmitEntry,
    SameRunSelfComparisonCannotAdmitEntry,
    TerminalProjectionCannotAdmitEntry,
    MissingRoadmapHarnessRequirement(S45RoadmapHarnessRequirement),
    IncompleteS4Closeout,
    S4CloseoutDoesNotRejectSyntheticShortcuts,
    S4CloseoutMissingS5RecoveryReadiness,
    S5IsolationAuthorityCannotBeMintedByHarnessEntry,
    TestSupportMechanicsCannotOwnCertificationMeaning,
    FoundationalProjectionCannotReplaceStoreAuthority,
    ProofProgressionSkipped,
    MissingReusableMechanicsInventory,
    MissingMilestoneLocalMechanicsInventory,
    MissingCertificationMeaningInventory,
    MissingObsoleteSemanticContextInventory,
}
