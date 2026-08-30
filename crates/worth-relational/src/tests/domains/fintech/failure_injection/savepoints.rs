use crate::facade::diagnostics::DiagnosticCode;
use crate::facade::history::BranchId;
use crate::facade::transactions::{RollbackOutcome, SavepointId};

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
    let mut txn =
        crate::tests::support::test_owner_begin_transaction_for_branch(&world.runtime, branch_id);
    txn.rollback_to_savepoint(SavepointId(999))
        .unwrap_err()
        .code
}
