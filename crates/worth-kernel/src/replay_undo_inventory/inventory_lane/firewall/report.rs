use crate::replay_undo_inventory::inventory_lane::closeout::ReplayUndoInventoryCloseout;
use crate::replay_undo_inventory::inventory_lane::declaration::{
    ReplayUndoDeclaredInputRole, ReplayUndoDeclaredSourceIdentity,
};

use super::current_manifests::required_role_for_source;
use super::violation::ReplayUndoSourceFirewallViolation;

#[derive(Clone, Debug)]
pub struct ReplayUndoSourceFirewallReport {
    closeout: ReplayUndoInventoryCloseout,
}

impl ReplayUndoSourceFirewallReport {
    pub(crate) fn new(closeout: ReplayUndoInventoryCloseout) -> Self {
        Self { closeout }
    }

    pub fn require_declared_receipt_role(
        &self,
        source_identity: ReplayUndoDeclaredSourceIdentity,
        role: ReplayUndoDeclaredInputRole,
    ) -> Result<(), ReplayUndoSourceFirewallViolation> {
        if required_role_for_source(self.closeout.declared_sources(), source_identity, role) {
            Ok(())
        } else {
            Err(ReplayUndoSourceFirewallViolation::new(
                source_identity,
                role,
            ))
        }
    }

    pub fn require_no_undeclared_receipt_consumers(&self) -> bool {
        self.closeout.rows().iter().all(|row| {
            self.closeout
                .declared_sources()
                .require_source(row.source_identity())
                .is_some()
        })
    }
}
