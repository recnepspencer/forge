use std::sync::MutexGuard;

use super::inventory::PhysicalWalInventorySnapshot;
use super::runtime_owner::{PhysicalWalRuntimeOwner, PhysicalWalRuntimeState};

/// Short WAL-owner fence protecting one checkpoint's final publication cutover.
pub(in crate::physical_runtime::durability) struct PhysicalWalCheckpointCutover<'owner> {
    state: MutexGuard<'owner, PhysicalWalRuntimeState>,
}

impl PhysicalWalRuntimeOwner {
    pub(in crate::physical_runtime::durability) fn checkpoint_cutover(
        &self,
    ) -> Option<PhysicalWalCheckpointCutover<'_>> {
        let state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.sealed || state.durable_lsn_end.is_none() {
            return None;
        }
        Some(PhysicalWalCheckpointCutover { state })
    }
}

impl PhysicalWalCheckpointCutover<'_> {
    pub(in crate::physical_runtime::durability) fn inventory_snapshot(
        &self,
    ) -> PhysicalWalInventorySnapshot {
        self.state
            .segments
            .snapshot(
                self.state
                    .durable_lsn_end
                    .expect("an admitted checkpoint cutover has a durable WAL frontier"),
            )
            .expect("an admitted checkpoint cutover has a nonempty WAL inventory")
    }

    pub(in crate::physical_runtime::durability) fn reclamation_plan(
        &self,
        publication: &crate::physical_runtime::durability::NamespaceDurableCheckpointPublication,
    ) -> Result<
        super::reclamation::PhysicalWalReclamationPlan,
        super::reclamation::PhysicalWalReclamationEligibilityDenial,
    > {
        super::reclamation::plan_reclamation(&self.state, publication)
    }
}
