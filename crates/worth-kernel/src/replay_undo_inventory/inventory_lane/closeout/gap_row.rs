use crate::replay_undo_inventory::inventory_lane::declaration::ReplayUndoDeclaredSourceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoInventoryGapRow {
    source_identity: ReplayUndoDeclaredSourceIdentity,
    removal_trigger: String,
}

impl ReplayUndoInventoryGapRow {
    pub(crate) fn new(
        source_identity: ReplayUndoDeclaredSourceIdentity,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self {
            source_identity,
            removal_trigger: removal_trigger.into(),
        }
    }

    pub const fn source_identity(&self) -> ReplayUndoDeclaredSourceIdentity {
        self.source_identity
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}
