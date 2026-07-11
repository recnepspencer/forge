#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLayoutAccessDenialKind {
    LocatorProjectionCannotStandInForCheckpointAuthority,
    RecoveryBlockedByIntegrityDamage,
    ReplayProjectionCannotStandInForWalAuthority,
    RecoverySourceRowCannotStandInForRecoveryAuthority,
    BackendResidueCannotStandInForCrashBoundaryAuthority,
    AmbiguousResidueCannotStandInForCrashBoundaryAuthority,
    DerivedRollbackCannotStandInForCrashBoundaryAuthority,
    BoundedWalTailLookupOutOfRange,
    ReplayTailCheckpointGap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLayoutAccessDenial {
    kind: RecoveryLayoutAccessDenialKind,
}

impl RecoveryLayoutAccessDenial {
    pub const fn new(kind: RecoveryLayoutAccessDenialKind) -> Self { Self { kind } }
    pub const fn kind(&self) -> RecoveryLayoutAccessDenialKind { self.kind }
}
