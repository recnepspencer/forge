#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoInventoryDisposition {
    Migrate,
    Delete,
    Cap,
    QueryGap,
}
