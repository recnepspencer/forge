use serde_json::json;

use crate::facade::{
    BranchId, CommitOutcome, RecordPayload, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use super::super::fixture::{FintechCaseRole, FintechWorld};

pub(crate) fn diverge_case_trade_on_branch(
    world: &mut FintechWorld,
    branch_id: BranchId,
    case_role: FintechCaseRole,
    notional: i64,
) -> CommitOutcome {
    let case = world.workflow_case(case_role);
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("diverge-case-trade").push(TransactionIntent::UpdateEntity {
            entity_id: case.trade,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "trade",
                "case": format!("{:?}", case.role),
                "desk": "analysis-branch",
                "book": "branch-divergence",
                "notional": notional,
                "ccy": "USD",
                "diverged": true,
            })),
        }),
    );
    txn.commit().unwrap()
}

pub(crate) fn merge_branch_into_main(
    world: &mut FintechWorld,
    merge_parent_branch: BranchId,
) -> CommitOutcome {
    let txn = world.runtime.begin_transaction(
        TransactionOptions {
            target_branch: Some(BranchId("main".to_string())),
            ..TransactionOptions::default()
        }
        .merge_from_branches(vec![merge_parent_branch]),
    );
    txn.commit().unwrap()
}
