use std::collections::BTreeMap;
use std::sync::Arc;

use crate::branch::RelationalBranchRoot;
use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::history::data::{BranchId, CommitId};
use crate::runtime::RelationalRuntime;

/// Reconstructive ownership for roots that a later tail branch admission may
/// reference after every current branch has already advanced past them.
pub(super) struct RecoveredRootInventory {
    roots: BTreeMap<CommitId, Arc<RelationalBranchRoot>>,
}

impl RecoveredRootInventory {
    pub(super) fn capture(runtime: &RelationalRuntime) -> Result<Self, DurabilityError> {
        let mut inventory = Self {
            roots: BTreeMap::new(),
        };
        for branch_id in runtime.history.branch_ids_snapshot() {
            inventory.retain_current(runtime, &branch_id)?;
        }
        Ok(inventory)
    }

    pub(super) fn resolve(&self, commit_id: CommitId) -> Option<Arc<RelationalBranchRoot>> {
        self.roots.get(&commit_id).cloned()
    }

    pub(super) fn retain_current(
        &mut self,
        runtime: &RelationalRuntime,
        branch_id: &BranchId,
    ) -> Result<(), DurabilityError> {
        let Some(cell) = runtime.history.branch_cell(branch_id) else {
            return Ok(());
        };
        let Some(root) = cell.root() else {
            return Ok(());
        };
        if matches!(
            cell.observation().target(),
            worth_foundational::FoundationalBranchTarget::Empty
        ) {
            if root.id() != 0 || root.descriptor().is_some() {
                return Err(DurabilityError::new(
                    RecoveryFailureClass::CorruptCheckpoint,
                    format!(
                        "recovered empty branch `{}` owns a committed root",
                        branch_id.0
                    ),
                ));
            }
            return Ok(());
        }
        let commit_id = root.commit_id().ok_or_else(|| {
            DurabilityError::new(
                RecoveryFailureClass::CorruptCheckpoint,
                format!(
                    "recovered branch `{}` owns an uncommitted root",
                    branch_id.0
                ),
            )
        })?;
        if let Some(existing) = self.roots.get(&commit_id) {
            if !Arc::ptr_eq(existing, &root) {
                return Err(DurabilityError::new(
                    RecoveryFailureClass::CorruptCheckpoint,
                    format!(
                        "recovery produced competing roots for commit {}",
                        commit_id.0
                    ),
                ));
            }
            return Ok(());
        }
        self.roots.insert(commit_id, Arc::clone(&root));
        Ok(())
    }

    pub(super) fn finish(self) {}
}
