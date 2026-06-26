use super::super::inventory_lane::{WorthGraphReadAccessInventoryRow, WorthGraphReadAccessOwner};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthGraphReadAccessInventoryRowIdentity {
    source_path: String,
    owner: WorthGraphReadAccessOwner,
    current_caller: String,
}

impl WorthGraphReadAccessInventoryRowIdentity {
    pub(crate) fn from_row(row: &WorthGraphReadAccessInventoryRow) -> Self {
        Self {
            source_path: row.source_path().to_string(),
            owner: row.owner(),
            current_caller: row.current_caller().to_string(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn owner(&self) -> WorthGraphReadAccessOwner {
        self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }
}
