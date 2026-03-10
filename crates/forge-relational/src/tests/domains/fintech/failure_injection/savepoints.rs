use crate::facade::{BranchId, DiagnosticCode, RollbackOutcome, SavepointId, TransactionOptions};

use super::super::actions::rollback_case_trade_after_savepoint;
use super::super::fixture::FintechWorld;

pub(crate) fn rollback_seeded_trade_correction_after_savepoint(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> RollbackOutcome {
    rollback_case_trade_after_savepoint(world, branch_id)
}

pub(crate) fn invalid_savepoint_rollback_code(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> DiagnosticCode {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.rollback_to_savepoint(SavepointId(999)).unwrap_err().code
}
