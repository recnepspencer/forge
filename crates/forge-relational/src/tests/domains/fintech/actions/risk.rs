use serde_json::json;

use crate::facade::{
    BranchId, CommitOutcome, RecordPayload, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use super::super::fixture::FintechWorld;

pub(crate) fn shock_market_on_branch(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    for (idx, market_point) in world.market.market_points.iter().enumerate() {
        txn.push_batch(
            WorkerIntentBatch::new(format!("shock-market-{idx}")).push(
                TransactionIntent::UpdateEntity {
                    entity_id: *market_point,
                    payload: RecordPayload::StructuredJson(json!({
                        "entity_type": "market_point",
                        "curve_bucket": idx,
                        "mid": 102_00 + (idx as i64 * 40),
                        "stress_regime": "intraday-shock",
                    })),
                },
            ),
        );
    }
    txn.commit().unwrap()
}

pub(crate) fn refresh_risk_views(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    for (idx, risk_view) in world.risk.risk_views.iter().enumerate() {
        txn.push_batch(
            WorkerIntentBatch::new(format!("refresh-risk-{idx}")).push(
                TransactionIntent::UpdateEntity {
                    entity_id: *risk_view,
                    payload: RecordPayload::StructuredJson(json!({
                        "entity_type": "risk_view",
                        "scenario": "intraday-shock",
                        "trade_index": idx,
                        "refreshed": true,
                    })),
                },
            ),
        );
    }
    txn.commit().unwrap()
}

pub(crate) fn stress_seeded_intraday_risk(
    world: &mut FintechWorld,
    branch_id: BranchId,
) -> CommitOutcome {
    let case = world.intraday_risk_case();
    let mut txn = world.runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-market").push(TransactionIntent::UpdateEntity {
            entity_id: case.market_point,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "market_point",
                "case": "intraday-risk",
                "curve_bucket": 2,
                "mid": 103_75,
                "stress_regime": "intraday-shock",
            })),
        }),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-risk").push(TransactionIntent::UpdateEntity {
            entity_id: case.risk_view,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "risk_view",
                "case": "intraday-risk",
                "scenario": "intraday-shock",
                "limit_status": "breached",
                "refreshed": true,
            })),
        }),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-limit").push(TransactionIntent::UpdateEntity {
            entity_id: case.limit,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "limit",
                "case": "intraday-risk",
                "threshold_bps": 140,
                "breach_state": "open",
            })),
        }),
    );
    txn.push_batch(
        WorkerIntentBatch::new("stress-intraday-breach").push(TransactionIntent::UpdateEntity {
            entity_id: case.breach,
            payload: RecordPayload::StructuredJson(json!({
                "entity_type": "limit_breach",
                "case": "intraday-risk",
                "status": "open",
                "severity": "critical",
            })),
        }),
    );
    txn.commit().unwrap()
}
