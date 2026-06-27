use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredInputRole, ReplayUndoDeclaredSourceIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoSourceFirewallViolation {
    source_identity: ReplayUndoDeclaredSourceIdentity,
    missing_role: ReplayUndoDeclaredInputRole,
}

impl ReplayUndoSourceFirewallViolation {
    pub(crate) fn new(
        source_identity: ReplayUndoDeclaredSourceIdentity,
        missing_role: ReplayUndoDeclaredInputRole,
    ) -> Self {
        Self {
            source_identity,
            missing_role,
        }
    }

    pub const fn source_identity(&self) -> ReplayUndoDeclaredSourceIdentity {
        self.source_identity
    }

    pub const fn missing_role(&self) -> ReplayUndoDeclaredInputRole {
        self.missing_role
    }
}
