use std::sync::{Arc, Condvar, Mutex};

use super::{SignalBranchExecutionCell, SignalOwnerOperationAdmission};

#[derive(Debug, Default)]
pub(super) struct SignalBranchForkCustodyGate {
    state: Mutex<SignalBranchForkCustodyState>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct SignalBranchForkCustodyState {
    fork_active: bool,
    ordinary_operations: usize,
}

pub(super) struct SignalBranchOrdinaryCellCustody {
    gate: Arc<SignalBranchForkCustodyGate>,
}

pub(in crate::branch::owner_services) struct SignalBranchForkSourceCustody<'admission, 'owner, S> {
    cell: Arc<SignalBranchExecutionCell<S>>,
    admission: &'admission SignalOwnerOperationAdmission<'owner>,
    gate: Arc<SignalBranchForkCustodyGate>,
}

impl SignalBranchForkCustodyGate {
    pub(super) fn acquire_ordinary(
        gate: &Arc<Self>,
        on_wait: impl FnOnce(),
    ) -> SignalBranchOrdinaryCellCustody {
        let mut state = gate.lock();
        let mut on_wait = Some(on_wait);
        while state.fork_active {
            if let Some(on_wait) = on_wait.take() {
                on_wait();
            }
            state = gate
                .changed
                .wait(state)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        state.ordinary_operations += 1;
        drop(state);
        SignalBranchOrdinaryCellCustody {
            gate: Arc::clone(gate),
        }
    }

    pub(super) fn acquire_fork<'admission, 'owner, S>(
        gate: &Arc<Self>,
        cell: &Arc<SignalBranchExecutionCell<S>>,
        admission: &'admission SignalOwnerOperationAdmission<'owner>,
        on_wait: impl FnOnce(),
    ) -> SignalBranchForkSourceCustody<'admission, 'owner, S> {
        let mut state = gate.lock();
        let mut on_wait = Some(on_wait);
        while state.fork_active || state.ordinary_operations != 0 {
            if let Some(on_wait) = on_wait.take() {
                on_wait();
            }
            state = gate
                .changed
                .wait(state)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        state.fork_active = true;
        drop(state);
        SignalBranchForkSourceCustody {
            cell: Arc::clone(cell),
            admission,
            gate: Arc::clone(gate),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, SignalBranchForkCustodyState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl<S> SignalBranchForkSourceCustody<'_, '_, S> {
    pub(in crate::branch::owner_services) fn cell(&self) -> &SignalBranchExecutionCell<S> {
        &self.cell
    }

    pub(super) fn matches(&self, cell: &SignalBranchExecutionCell<S>) -> bool {
        std::ptr::eq(self.cell.as_ref(), cell)
    }

    pub(super) fn admission(&self) -> &SignalOwnerOperationAdmission<'_> {
        self.admission
    }
}

impl Drop for SignalBranchOrdinaryCellCustody {
    fn drop(&mut self) {
        let mut state = self.gate.lock();
        state.ordinary_operations = state
            .ordinary_operations
            .checked_sub(1)
            .expect("ordinary branch-cell custody releases exactly once");
        drop(state);
        self.gate.changed.notify_all();
    }
}

impl<S> Drop for SignalBranchForkSourceCustody<'_, '_, S> {
    fn drop(&mut self) {
        let mut state = self.gate.lock();
        assert!(
            state.fork_active,
            "fork source custody releases exactly once"
        );
        state.fork_active = false;
        drop(state);
        self.gate.changed.notify_all();
    }
}
