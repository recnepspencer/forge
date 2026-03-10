use serde_json::json;

use crate::facade::{
    BranchId, CommitOutcome, RecordPayload, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use super::super::fixture::{FintechCaseRole, FintechWorld};

pub(crate) fn emit_case_audit_record(
    world: &mut FintechWorld,
    branch_id: BranchId,
    case_role: FintechCaseRole,
    event: &str,
) -> CommitOutcome {
    let case = world.workflow_case(case_role);
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("audit-{}", event.replace(' ', "-"))).push(
            TransactionIntent::UpdateEntity {
                entity_id: case.audit_record,
                payload: RecordPayload::StructuredJson(json!({
                    "entity_type": "audit_record",
                    "case": format!("{:?}", case.role),
                    "event": event,
                    "recorded_by": "fintech-domain-workflow",
                })),
            },
        ),
    );
    txn.commit().unwrap()
}

pub(crate) fn emit_trade_correction_audit_record(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitOutcome {
    emit_case_audit_record(
        world,
        branch_id,
        FintechCaseRole::LateTradeCorrection,
        "trade-correction-confirmed",
    )
}
