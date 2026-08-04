use crate::state::{SignalBranchHandle, SignalBranchId};

use super::{
    AuthorityTransferPacket, BranchLifecycleTransfer, RestoreTransferPacket, SignalRuntime,
};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn load_branch_state(
        &mut self,
        packet: AuthorityTransferPacket<D, I, T>,
        count_temporal_restore: bool,
    ) -> Result<(), crate::data::error::SignalError> {
        let preserved_transaction = self.telemetry.transaction;
        let branch_id = packet.branch_id();
        let state = packet.into_state();
        if branch_id != state.ancestry().branch_id() {
            return Err(crate::data::error::SignalError::internal(format!(
                "branch lifecycle transfer mismatch: packet branch {} does not match state branch {}",
                branch_id.0,
                state.ancestry().branch_id().0
            )));
        }
        Self::ensure_managed_queue_branch_transfer_allowed(&self.resource)?;
        Self::ensure_managed_queue_branch_transfer_allowed(state.resource())?;
        self.branches.restore_active_state(
            state,
            &mut self.graph,
            &mut self.config,
            &mut self.checkpoint,
            &mut self.resource,
            &mut self.temporal,
            &mut self.telemetry,
            count_temporal_restore,
        );
        Self::merge_global_transaction_telemetry(
            preserved_transaction,
            &mut self.telemetry.transaction,
        );
        Ok(())
    }

    fn load_restored_branch_state(
        &mut self,
        packet: RestoreTransferPacket<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        self.telemetry.transaction.restore_transfer_count += 1;
        self.load_branch_state(
            AuthorityTransferPacket::new(packet.branch_id(), packet.into_state()),
            true,
        )
    }

    pub(in crate::logic::transaction::runtime::state) fn apply_branch_lifecycle_transfer(
        &mut self,
        transfer: BranchLifecycleTransfer<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        match transfer {
            BranchLifecycleTransfer::Move(packet) => self.load_branch_state(packet, false),
            BranchLifecycleTransfer::Restore(packet) => self.load_restored_branch_state(packet),
        }
    }

    pub(in crate::logic::transaction::runtime::state) fn synchronize_branch_catalogs(
        &mut self,
        branch_catalog: std::collections::BTreeMap<SignalBranchId, SignalBranchHandle>,
    ) {
        let active_branch = self.graph.current_branch().id;
        self.branches
            .synchronize_catalogs(&branch_catalog, active_branch, &mut self.graph);
    }
}
