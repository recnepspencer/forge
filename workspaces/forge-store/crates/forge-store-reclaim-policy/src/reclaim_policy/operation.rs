#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimPolicyOperation {
    Trim,
    PunchHole,
    SparseDeclare,
    ColdTierMovementPosture,
}

impl ReclaimPolicyOperation {
    pub const fn is_cold_tier(self) -> bool {
        matches!(self, Self::ColdTierMovementPosture)
    }
}
