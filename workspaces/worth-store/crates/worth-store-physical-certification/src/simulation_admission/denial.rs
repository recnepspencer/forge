use super::requirement_set::SimulationHarnessRoadmapRequirement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationHarnessBoundaryDenial {
    CopiedRecoveryReportCannotAdmitEntry,
    LogOutputCannotAdmitEntry,
    OldSemanticHarnessContextCannotAdmitEntry,
    SameRunSelfComparisonCannotAdmitEntry,
    TerminalProjectionCannotAdmitEntry,
    MissingRoadmapHarnessRequirement(SimulationHarnessRoadmapRequirement),
    IncompleteRecoveryCloseout,
    RecoveryCloseoutDoesNotRejectSyntheticShortcuts,
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
