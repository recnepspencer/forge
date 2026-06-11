#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InventoryDecision {
    ElevateToWorkloadPlatform,
    WrapAsLocalUnitSupport,
    DeleteAfterReplacement,
    LeaveUnitOnly,
}
