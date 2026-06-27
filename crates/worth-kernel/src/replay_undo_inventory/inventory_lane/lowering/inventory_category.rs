#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoInventoryCategory {
    TopologyReplayScope,
    SpatialReplayScope,
    UndoScope,
    TransactionBoundary,
    Residue,
}
