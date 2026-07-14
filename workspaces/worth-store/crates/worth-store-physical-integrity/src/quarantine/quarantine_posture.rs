#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineLifecyclePosture {
    Proposed,
    Sealed,
    SupersededByRecovery,
    ReleasedAfterRepair,
    RetainedForAudit,
    InvalidatedByRootChange,
}

impl QuarantineLifecyclePosture {
    pub const fn is_physical_integrity_mintable(self) -> bool {
        matches!(self, Self::Proposed | Self::Sealed)
    }

    pub const fn sealed_after_physical_integrity_mint(self) -> Self {
        match self {
            Self::Proposed | Self::Sealed => Self::Sealed,
            Self::SupersededByRecovery
            | Self::ReleasedAfterRepair
            | Self::RetainedForAudit
            | Self::InvalidatedByRootChange => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineHandoffPosture {
    RecoveryOwnerRequired,
    RepairOwnerRequired,
    AuditRetentionOwnerRequired,
    RootChangeRevalidationRequired,
}
