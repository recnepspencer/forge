use crate::facade::history::BranchId;
use crate::facade::transactions::{
    CommitResult, EntityMutationIntent, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn shock_market_on_branch(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    for (idx, market_point) in world.market.market_points.iter().enumerate() {
        txn.push_batch(WorkerIntentBatch::new(format!("shock-market-{idx}")).push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: *market_point,
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("market_point"),
                        ),
                        (
                            "curve_bucket",
                            crate::tests::support::usize_aspect_value(idx),
                        ),
                        (
                            "mid",
                            crate::tests::support::fixture_i64_number_aspect_value(
                                102_00 + (idx as i64 * 40),
                            ),
                        ),
                        (
                            "stress_regime",
                            crate::tests::support::string_aspect_value("intraday-shock"),
                        ),
                    ]),
                },
            )),
        ));
    }
    txn.commit().unwrap()
}

pub(crate) fn refresh_risk_views(world: &mut FintechWorld, branch_id: BranchId) -> CommitResult {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    for (idx, risk_view) in world.risk.risk_views.iter().enumerate() {
        txn.push_batch(
            WorkerIntentBatch::new(format!("refresh-risk-{idx}")).push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: *risk_view,
                        fields: crate::tests::support::aspect_field_patch_from_values([
                            (
                                "entity_type",
                                crate::tests::support::string_aspect_value("risk_view"),
                            ),
                            (
                                "scenario",
                                crate::tests::support::string_aspect_value("intraday-shock"),
                            ),
                            (
                                "trade_index",
                                crate::tests::support::usize_aspect_value(idx),
                            ),
                            ("refreshed", crate::tests::support::bool_aspect_value(true)),
                        ]),
                    },
                ))
                .into(),
            ),
        );
    }
    txn.commit().unwrap()
}

pub(crate) fn stress_seeded_intraday_risk(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitResult {
    let case = world.intraday_risk_case();
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-market")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.market_point,
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("market_point"),
                        ),
                        (
                            "case",
                            crate::tests::support::string_aspect_value("intraday-risk"),
                        ),
                        ("curve_bucket", crate::tests::support::u64_aspect_value(2)),
                        ("mid", crate::tests::support::u64_aspect_value(103_75)),
                        (
                            "stress_regime",
                            crate::tests::support::string_aspect_value("intraday-shock"),
                        ),
                    ]),
                },
            )))
            .into(),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-risk")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.risk_view,
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("risk_view"),
                        ),
                        (
                            "case",
                            crate::tests::support::string_aspect_value("intraday-risk"),
                        ),
                        (
                            "scenario",
                            crate::tests::support::string_aspect_value("intraday-shock"),
                        ),
                        (
                            "limit_status",
                            crate::tests::support::string_aspect_value("breached"),
                        ),
                        ("refreshed", crate::tests::support::bool_aspect_value(true)),
                    ]),
                },
            )))
            .into(),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-limit")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.limit,
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "entity_type",
                            crate::tests::support::string_aspect_value("limit"),
                        ),
                        (
                            "case",
                            crate::tests::support::string_aspect_value("intraday-risk"),
                        ),
                        (
                            "threshold_bps",
                            crate::tests::support::u64_aspect_value(140),
                        ),
                        (
                            "breach_state",
                            crate::tests::support::string_aspect_value("open"),
                        ),
                    ]),
                },
            )))
            .into(),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-breach")
            .push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: case.breach,
                    fields: crate::tests::support::string_aspect_field_patch([
                        ("entity_type", "limit_breach"),
                        ("case", "intraday-risk"),
                        ("status", "open"),
                        ("severity", "critical"),
                    ]),
                },
            )))
            .into(),
    );
    txn.commit().unwrap()
}
