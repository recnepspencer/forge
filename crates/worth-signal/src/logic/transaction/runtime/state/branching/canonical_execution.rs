use worth_proof::TransitionOutcome;

use crate::data::error::SignalError;

use super::super::runtime_state::SignalRuntime;
use super::{
    BranchTargetedTransactionExecutionOutcome, LoweredBranchTargetedTransactionPlan,
    PlannedSignalBranchRetirement, PlannedSignalBranchRetirementBatch,
    SignalBranchRetirementBatchDenial, SignalBranchRetirementBatchReceipt,
    SignalBranchRetirementDenial, SignalBranchRetirementReceipt,
};

/// Public execution adapters for the canonical Foundational-basis planning
/// doors.  The legacy head/request engine remains private to this module; the
/// opaque plan is intentionally only useful when returned by the canonical
/// planner in the same facade.
impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn execute_signal_branch_targeted_transaction<F>(
        &mut self,
        runtime_ctx: &mut Ctx,
        plan: LoweredBranchTargetedTransactionPlan,
        apply: F,
    ) -> BranchTargetedTransactionExecutionOutcome
    where
        F: FnOnce(
            &mut crate::logic::transaction::runtime::transaction::SignalTransaction<
                '_,
                D,
                I,
                E,
                Ctx,
                T,
            >,
        ) -> Result<(), SignalError>,
    {
        self.execute_branch_targeted_transaction(runtime_ctx, plan, apply)
    }

    pub fn retire_signal_branch(
        &mut self,
        plan: PlannedSignalBranchRetirement,
    ) -> TransitionOutcome<SignalBranchRetirementReceipt, SignalBranchRetirementDenial> {
        self.retire_branch(plan)
    }

    pub fn retire_signal_branch_batch(
        &mut self,
        plan: PlannedSignalBranchRetirementBatch,
    ) -> TransitionOutcome<SignalBranchRetirementBatchReceipt, SignalBranchRetirementBatchDenial>
    {
        self.retire_branch_batch(plan)
    }
}
