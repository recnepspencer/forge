#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationEntryDenial {
    CopiedRecoveryFields,
    LiveRuntimeState,
    TerminalProjection,
    SemanticSnapshot,
    JsonAuthority,
    FoundationalOrProofProjection,
    StaleRecoveryReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationEntryRebindRequired {
    RecoveryReadinessMustBeRebound,
}
