use crate::facade::history::BranchId;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::fixture::{FintechCaseRole, FintechWorld};

pub(crate) fn diverge_case_trade_on_branch(
    world: &mut FintechWorld,
    branch_id: BranchId,
    case_role: FintechCaseRole,
    notional: i64,
) -> CommitResult {
    let case = world.workflow_case(case_role);
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("diverge-case-trade").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: case.trade,
                fields: crate::tests::support::aspect_field_patch_from_values([
                    (
                        "entity_type",
                        crate::tests::support::string_aspect_value("trade"),
                    ),
                    (
                        "case",
                        crate::tests::support::string_aspect_value(&format!("{:?}", case.role)),
                    ),
                    (
                        "desk",
                        crate::tests::support::string_aspect_value("analysis-branch"),
                    ),
                    (
                        "book",
                        crate::tests::support::string_aspect_value("branch-divergence"),
                    ),
                    (
                        "notional",
                        crate::tests::support::fixture_i64_number_aspect_value(notional),
                    ),
                    ("ccy", crate::tests::support::string_aspect_value("USD")),
                    ("diverged", crate::tests::support::bool_aspect_value(true)),
                ]),
            }),
        )),
    );
    txn.commit().unwrap()
}

pub(crate) fn merge_branch_into_main(
    world: &mut FintechWorld,
    merge_parent_branch: BranchId,
) -> CommitResult {
    let txn = world.runtime.begin_transaction(
        TransactionOptions {
            target_branch: Some(BranchId("main".to_string())),
            ..TransactionOptions::default()
        }
        .merge_from_branches(vec![merge_parent_branch]),
    );
    txn.commit().unwrap()
}
