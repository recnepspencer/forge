use serde_json::json;

use crate::facade::history::BranchId;
use crate::facade::identity::EntityId;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn repair_seeded_failed_settlement(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    let case = world.failed_settlement_repair_case();
    repair_settlement_with_aspect_field_patches(
        world,
        branch_id,
        case.settlement,
        case.cash_event,
        case.audit_record,
    )
}

pub(crate) fn repair_settlement_with_aspect_field_patches(
    world: &mut FintechWorld,
    branch_id: BranchId,
    settlement_id: EntityId,
    cash_event_id: EntityId,
    audit_record_id: EntityId,
) -> CommitResult {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("repair-settlement")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: settlement_id,
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        json!({
                            "entity_type": "settlement",
                            "case": "failed-settlement-repair",
                            "status": "repaired",
                            "repair_completed": true,
                        }),
                    ),
                },
            )))
            .into(),
    );
    txn.push_batch(
        WorkerIntentBatch::new("repair-cash-event")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: cash_event_id,
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        json!({
                            "entity_type": "cash_event",
                            "case": "failed-settlement-repair",
                            "kind": "repair-funding",
                            "status": "applied",
                        }),
                    ),
                },
            )))
            .into(),
    );
    txn.push_batch(
        WorkerIntentBatch::new("repair-audit-record")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: audit_record_id,
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        json!({
                            "entity_type": "audit_record",
                            "case": "failed-settlement-repair",
                            "event": "settlement-repaired",
                        }),
                    ),
                },
            )))
            .into(),
    );
    txn.commit().unwrap()
}
