use forge_store_recovery_physics::S5RecoveryReadinessDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationEntryDenial {
    RecoveryReadiness(S5RecoveryReadinessDenial),
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
    S4RecoveryReadinessMustBeRebound,
}

impl From<S5RecoveryReadinessDenial> for PhysicalIsolationEntryDenial {
    fn from(denial: S5RecoveryReadinessDenial) -> Self {
        Self::RecoveryReadiness(denial)
    }
}
