use serde_json::json;

use crate::facade::history::BranchId;
use crate::facade::identity::EntityId;
use crate::facade::payloads::RecordPayload;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent,
    WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn correct_trade_with_replacement(
    world: &mut FintechWorld,
    branch_id: BranchId,
    trade_id: EntityId,
) -> CommitResult {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("replace-trade").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: trade_id,
                payload: RecordPayload::StructuredJson(json!({
                    "entity_type": "trade",
                    "desk": "macro-flow",
                    "book": "analysis-risk",
                    "notional": 1_750_000,
                    "ccy": "USD",
                    "corrected": true,
                })),
            }),
        )),
    );
    txn.commit().unwrap()
}

pub(crate) fn correct_seeded_trade_candidate(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    correct_trade_with_replacement(world, branch_id, world.late_trade_correction_case().trade)
}
