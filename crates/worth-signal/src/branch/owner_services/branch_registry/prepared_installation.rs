use std::sync::Arc;

use super::{SignalBranchExecutionCell, SignalBranchRegistryEntry, SignalBranchReservation};

pub(crate) struct SignalPreparedBranchCell<S> {
    pub(super) cell: Arc<SignalBranchExecutionCell<S>>,
    pub(super) is_fork_destination: bool,
}

pub(crate) struct SignalPreparedBranchInstallation<'a, S> {
    pub(super) reservation: SignalBranchReservation<'a, S>,
    pub(super) cell: Arc<SignalBranchExecutionCell<S>>,
    pub(super) is_fork_destination: bool,
}

impl<S> SignalPreparedBranchInstallation<'_, S> {
    pub(crate) fn install(mut self) -> Arc<SignalBranchExecutionCell<S>> {
        let mut state = self.reservation.registry.lock_state();
        let entry = state
            .entries
            .get_mut(&self.reservation.branch_id)
            .expect("prepared Signal branch reservation must remain registered");
        assert!(
            matches!(entry, SignalBranchRegistryEntry::Reserved),
            "prepared Signal branch reservation must remain vacant"
        );
        *entry = SignalBranchRegistryEntry::Live(Arc::clone(&self.cell));
        state.reservation_count = state
            .reservation_count
            .checked_sub(1)
            .expect("prepared Signal branch installation must consume one reservation");
        state.live_count += 1;
        self.reservation.consumed = true;
        drop(state);
        if self.is_fork_destination {
            self.reservation
                .registry
                .counters
                .record_fork_destination_installation();
        }
        Arc::clone(&self.cell)
    }
}
