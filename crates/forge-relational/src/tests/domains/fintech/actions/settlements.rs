use serde_json::json;

use crate::facade::{
    BranchId, CommitOutcome, RecordPayload, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn repair_seeded_failed_settlement(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitOutcome {
    let case = world.failed_settlement_repair_case();
    repair_settlement_with_payloads(
        world,
        branch_id,
        case.settlement,
        case.cash_event,
        case.audit_record,
    )
}

pub(crate) fn repair_settlement_with_payloads(
    world: &mut FintechWorld,
    branch_id: BranchId,
    settlement_id: crate::facade::EntityId,
    cash_event_id: crate::facade::EntityId,
    audit_record_id: crate::facade::EntityId,
) -> CommitOutcome {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(WorkerIntentBatch::new("repair-settlement").push(
        TransactionIntent::UpdateEntity {
            entity_id: settlement_id,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "settlement",
                "case": "failed-settlement-repair",
                "status": "repaired",
                "repair_completed": true,
            })),
        },
    ));
    txn.push_batch(WorkerIntentBatch::new("repair-cash-event").push(
        TransactionIntent::UpdateEntity {
            entity_id: cash_event_id,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "cash_event",
                "case": "failed-settlement-repair",
                "kind": "repair-funding",
                "status": "applied",
            })),
        },
    ));
    txn.push_batch(WorkerIntentBatch::new("repair-audit-record").push(
        TransactionIntent::UpdateEntity {
            entity_id: audit_record_id,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "audit_record",
                "case": "failed-settlement-repair",
                "event": "settlement-repaired",
            })),
        },
    ));
    txn.commit().unwrap()
}
