#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoverySourceApplicationRole {
    CheckpointBase,
    WalTailRedo,
    PageSkipApply,
    CompactionVisibility,
    ResidueDiscoveryOnly,
    RecoveryBlocked,
}
