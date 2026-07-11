use super::requirement_set::SimulationHarnessRoadmapRequirement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationHarnessBoundaryDenial {
    CopiedS4ReportCannotAdmitEntry,
    LogOutputCannotAdmitEntry,
    OldSemanticHarnessContextCannotAdmitEntry,
    SameRunSelfComparisonCannotAdmitEntry,
    TerminalProjectionCannotAdmitEntry,
    MissingRoadmapHarnessRequirement(SimulationHarnessRoadmapRequirement),
    IncompleteS4Closeout,
    S4CloseoutDoesNotRejectSyntheticShortcuts,
    RecoveryCloseoutMissingPhysicalIsolationReadiness,
    PhysicalIsolationAuthorityCannotBeMintedByHarnessEntry,
    TestSupportMechanicsCannotOwnCertificationMeaning,
    FoundationalProjectionCannotReplaceStoreAuthority,
    ProofProgressionSkipped,
    MissingReusableMechanicsInventory,
    MissingMilestoneLocalMechanicsInventory,
    MissingCertificationMeaningInventory,
    MissingObsoleteSemanticContextInventory,
}
