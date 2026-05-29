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
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("trade"),
                        ),
                        (
                            "case",
                            crate::tests::support::string_aspect_value("LateTradeCorrection"),
                        ),
                        (
                            "desk",
                            crate::tests::support::string_aspect_value("analysis-risk"),
                        ),
                        (
                            "book",
                            crate::tests::support::string_aspect_value("temporary-correction"),
                        ),
                        (
                            "notional",
                            crate::tests::support::u64_aspect_value(1_999_999),
                        ),
                        ("ccy", crate::tests::support::string_aspect_value("USD")),
                        ("corrected", crate::tests::support::bool_aspect_value(true)),
                        ("transient", crate::tests::support::bool_aspect_value(true)),
                    ]),
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
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("trade"),
                        ),
                        (
                            "case",
                            crate::tests::support::string_aspect_value("LateTradeCorrection"),
                        ),
                        (
                            "desk",
                            crate::tests::support::string_aspect_value("analysis-risk"),
                        ),
                        (
                            "book",
                            crate::tests::support::string_aspect_value("saved-correction"),
                        ),
                        (
                            "notional",
                            crate::tests::support::u64_aspect_value(1_800_000),
                        ),
                        ("ccy", crate::tests::support::string_aspect_value("USD")),
                        ("corrected", crate::tests::support::bool_aspect_value(true)),
                        (
                            "savepoint_applied",
                            crate::tests::support::bool_aspect_value(true),
                        ),
                    ]),
                },
            ))
            .into(),
        ),
    );
    txn.commit().unwrap()
}
