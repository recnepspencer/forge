use crate::facade::history::BranchId;
use crate::facade::identity::EntityId;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
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
        WorkerIntentBatch::new("replace-trade")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: trade_id,
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("trade"),
                        ),
                        (
                            "desk",
                            crate::tests::support::string_aspect_value("macro-flow"),
                        ),
                        (
                            "book",
                            crate::tests::support::string_aspect_value("analysis-risk"),
                        ),
                        (
                            "notional",
                            crate::tests::support::u64_aspect_value(1_750_000),
                        ),
                        ("ccy", crate::tests::support::string_aspect_value("USD")),
                        ("corrected", crate::tests::support::bool_aspect_value(true)),
                    ]),
                },
            )))
            .into(),
    );
    txn.commit().unwrap()
}

pub(crate) fn correct_seeded_trade_candidate(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    correct_trade_with_replacement(world, branch_id, world.late_trade_correction_case().trade)
}
