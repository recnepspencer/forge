use serde_json::json;

use crate::facade::history::BranchId;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, RollbackOutcome, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn rollback_case_trade_after_savepoint(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> RollbackOutcome {
    let case = world.late_trade_correction_case();
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    let savepoint = txn.create_savepoint();
    txn.push_batch(
        WorkerIntentBatch::new("temporary-case-trade-correction")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.trade,
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        json!({
                            "entity_type": "trade",
                            "case": "LateTradeCorrection",
                            "desk": "analysis-risk",
                            "book": "temporary-correction",
                            "notional": 1_999_999,
                            "ccy": "USD",
                            "corrected": true,
                            "transient": true,
                        }),
                    ),
                },
            )))
            .into(),
    );
    txn.rollback_to_savepoint(savepoint).unwrap()
}

pub(crate) fn commit_case_trade_after_savepoint(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    let case = world.late_trade_correction_case();
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    let _savepoint = txn.create_savepoint();
    txn.push_batch(
        WorkerIntentBatch::new("saved-case-trade-correction").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.trade,
                    fields: crate::tests::support::aspect_field_patch_from_compatibility_json(
                        json!({
                            "entity_type": "trade",
                            "case": "LateTradeCorrection",
                            "desk": "analysis-risk",
                            "book": "saved-correction",
                            "notional": 1_800_000,
                            "ccy": "USD",
                            "corrected": true,
                            "savepoint_applied": true,
                        }),
                    ),
                },
            ))
            .into(),
        ),
    );
    txn.commit().unwrap()
}
