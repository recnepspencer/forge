use std::sync::Arc;

use crate::branch::{PlannedSignalBranchRetirement, SignalBranchRetirementDenial};
use crate::state::SignalBranchId;

use super::super::branch_execution_cell::retirement::SignalBranchRetirementCellOutcome;
use super::super::{SignalBranchCellState, SignalOwnerCancellationToken};
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
    pub(super) completed: bool,
}

impl<S> SignalBranchRetirement<'_, S> {
    #[cfg(test)]
    pub(crate) fn execute<R, D>(
        mut self,
        operation: impl FnOnce(&mut S, &super::super::SignalBranchCellWork<'_>) -> Result<R, D>,
    ) -> Result<Result<R, D>, SignalBranchRegistryDenial> {
        self.registry.validate_admission(self.admission)?;
        let result = self
            .cell
            .with_retirement(self.admission, operation)
            .map_err(SignalBranchRegistryDenial::TargetCellDenied)?;
        if result.is_err() {
            return Ok(result);
        }
        let mut state = self.registry.lock_state();
        if !entry_is_retiring_cell(state.entries.get(&self.branch_id), &self.cell) {
            return Err(SignalBranchRegistryDenial::ExpiredRetirement(
                self.branch_id,
            ));
        }
        state.entries.remove(&self.branch_id);
        state.live_count = state
            .live_count
            .checked_sub(1)
            .expect("completed Signal branch retirement must release live capacity");
        self.completed = true;
        Ok(result)
    }
}

impl<D, I, T> SignalBranchRetirement<'_, SignalBranchCellState<D, I, T>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn execute_exact(
        mut self,
        plan: PlannedSignalBranchRetirement,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<
        Result<SignalBranchRetirementCellOutcome, SignalBranchRetirementDenial>,
        SignalBranchRegistryDenial,
    > {
        self.registry.validate_admission(self.admission)?;
        let outcome = self.cell.retire_exact(self.admission, plan, cancellation);
        if outcome.is_err() {
            return Ok(outcome);
        }
        let mut state = self.registry.lock_state();
        if !entry_is_retiring_cell(state.entries.get(&self.branch_id), &self.cell) {
            return Err(SignalBranchRegistryDenial::ExpiredRetirement(
                self.branch_id,
            ));
        }
        state.entries.remove(&self.branch_id);
        state.live_count = state
            .live_count
            .checked_sub(1)
            .expect("completed Signal branch retirement must release live capacity");
        self.completed = true;
        Ok(outcome)
    }
}

impl<S> Drop for SignalBranchRetirement<'_, S> {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        let mut state = self.registry.lock_state();
        if entry_is_retiring_cell(state.entries.get(&self.branch_id), &self.cell) {
            state.entries.insert(
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
