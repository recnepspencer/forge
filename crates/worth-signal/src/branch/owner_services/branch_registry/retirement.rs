use std::sync::Arc;

use crate::state::SignalBranchId;

use super::{
    SignalBranchExecutionCell, SignalBranchRegistry, SignalBranchRegistryDenial,
    SignalBranchRegistryEntry, SignalOwnerOperationAdmission,
};

/// RAII owner of one registry-marked branch retirement.
///
/// Dropping before the cell becomes inert reopens the same canonical cell.
/// Dropping after cell retirement removes membership so capacity cannot leak.
#[derive(Debug)]
pub(crate) struct SignalBranchRetirement<'a, S> {
    pub(super) registry: &'a SignalBranchRegistry<S>,
    pub(super) admission: &'a SignalOwnerOperationAdmission,
    pub(super) branch_id: SignalBranchId,
    pub(super) cell: Arc<SignalBranchExecutionCell<S>>,
    pub(super) cell_marked_retiring: bool,
    pub(super) cell_retired: bool,
    pub(super) completed: bool,
}

impl<S> SignalBranchRetirement<'_, S> {
    pub(crate) fn complete(mut self) -> Result<(), SignalBranchRegistryDenial> {
        self.registry.validate_admission(self.admission)?;
        self.cell
            .finish_retirement(self.admission)
            .map_err(SignalBranchRegistryDenial::TargetCellDenied)?;
        self.cell_retired = true;
        let mut state = self.registry.lock_state();
        if !entry_is_retiring_cell(state.cells.get(&self.branch_id), &self.cell) {
            return Err(SignalBranchRegistryDenial::ExpiredRetirement(
                self.branch_id,
            ));
        }
        state.cells.remove(&self.branch_id);
        self.completed = true;
        Ok(())
    }
}

impl<S> Drop for SignalBranchRetirement<'_, S> {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if self.cell_retired {
            let mut state = self.registry.lock_state();
            if entry_is_retiring_cell(state.cells.get(&self.branch_id), &self.cell) {
                state.cells.remove(&self.branch_id);
            }
            return;
        }
        if self.cell_marked_retiring {
            self.cell.cancel_retirement();
        }
        let mut state = self.registry.lock_state();
        if entry_is_retiring_cell(state.cells.get(&self.branch_id), &self.cell) {
            state.cells.insert(
                self.branch_id,
                SignalBranchRegistryEntry::Live(Arc::clone(&self.cell)),
            );
        }
    }
}

fn entry_is_retiring_cell<S>(
    entry: Option<&SignalBranchRegistryEntry<S>>,
    expected: &Arc<SignalBranchExecutionCell<S>>,
) -> bool {
    matches!(
        entry,
        Some(SignalBranchRegistryEntry::Retiring(cell)) if Arc::ptr_eq(cell, expected)
    )
}
