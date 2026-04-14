use crate::facade::identity::{EntityId, KindId, PartitionId, RelationId};
use crate::facade::payloads::RecordPayload;
use crate::facade::runtime::RelationalRuntime;
use crate::facade::symbols::InternedString;
use crate::facade::transactions::{
    BulkRelationCreateIntent, CommitResult, CreateIntent, MutationIntent, RecordRef,
    TransactionOptions, WorkerIntentBatch,
};
use serde_json::json;

use super::entity_seeding::SeededEntityState;
use super::seed_catalog::FintechCaseSeed;
use super::{FintechWorkflowCase, LEDGER_PARTITION, MARKET_PARTITION, RISK_PARTITION};

pub(super) fn seed_relations(
    runtime: &mut RelationalRuntime,
    case_seeds: &[FintechCaseSeed],
    seeded: &SeededEntityState,
    workflow_cases: &[FintechWorkflowCase],
) -> Vec<RelationId> {
    let mut relations = Vec::new();
    relations.extend(bulk_create_relations(
        runtime,
        "desk-book",
        LEDGER_PARTITION,
        seeded.book_names.iter().map(|name| {
            (
                format!("desk-book-{name}"),
                (
                    *seeded.book_map.get(name).expect("book exists"),
                    *seeded
                        .desk_map
                        .get(
                            case_seeds
                                .iter()
                                .find(|seed| seed.book_name == *name)
                                .map(|seed| seed.desk_name)
                                .expect("book should belong to desk"),
                        )
                        .expect("desk exists"),
                ),
                json!({ "role": "owned_by_desk" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-account",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-account-{:?}", case.role),
                (case.trade, case.account),
                json!({ "role": "book_owner" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-book",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-book-{:?}", case.role),
                (case.trade, case.book),
                json!({ "role": "booked_in" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-counterparty",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-counterparty-{:?}", case.role),
                (case.trade, case.counterparty),
                json!({ "role": "facing" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-settlement",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-settlement-{:?}", case.role),
                (case.trade, case.settlement),
                json!({ "role": "settles_via" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "settlement-cash-event",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("settlement-cash-event-{:?}", case.role),
                (case.settlement, case.cash_event),
                json!({ "role": "funded_by" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-audit-record",
        LEDGER_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-audit-{:?}", case.role),
                (case.trade, case.audit_record),
                json!({ "role": "audited_by" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-instrument",
        MARKET_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-instrument-{:?}", case.role),
                (case.trade, case.instrument),
                json!({ "role": "references_instrument" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-market",
        MARKET_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-market-{:?}", case.role),
                (case.trade, case.market_point),
                json!({ "role": "marks" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "trade-risk",
        RISK_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("trade-risk-{:?}", case.role),
                (case.trade, case.risk_view),
                json!({ "role": "derived_risk" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "risk-limit",
        RISK_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("risk-limit-{:?}", case.role),
                (case.risk_view, case.limit),
                json!({ "role": "checked_against" }),
            )
        }),
    ));
    relations.extend(bulk_create_relations(
        runtime,
        "limit-breach",
        RISK_PARTITION,
        workflow_cases.iter().map(|case| {
            (
                format!("limit-breach-{:?}", case.role),
                (case.limit, case.breach),
                json!({ "role": "breach_state" }),
            )
        }),
    ));
    relations
}

pub(super) fn bulk_create_relations<I>(
    runtime: &mut RelationalRuntime,
    batch_name: &str,
    partition_id: PartitionId,
    specs: I,
) -> Vec<RelationId>
where
    I: IntoIterator<Item = (String, (EntityId, EntityId), serde_json::Value)>,
{
    let mut client_keys = Vec::new();
    let mut endpoints = Vec::new();
    let mut payloads = Vec::new();
    for (key, (source, target), payload) in specs {
        client_keys.push(InternedString::Raw(key));
        endpoints.push((crate::transactions::data::EntityReference::Existing(source), crate::transactions::data::EntityReference::Existing(target)));
        payloads.push(Some(RecordPayload::StructuredJson(payload)));
    }
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new(batch_name).push(MutationIntent::Create(
            CreateIntent::BulkRelations(BulkRelationCreateIntent {
                partition_id,
                kind_id: KindId(2),
                client_keys,
                endpoints,
                payloads,
            }),
        )),
    );
    changed_relations(&txn.commit().unwrap())
}

fn changed_relations(outcome: &CommitResult) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Entity(_) => None,
            RecordRef::Relation(id) => Some(*id),
        })
        .collect()
}

