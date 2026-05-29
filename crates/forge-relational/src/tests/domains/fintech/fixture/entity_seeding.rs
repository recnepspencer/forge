use std::collections::BTreeMap;

use crate::facade::identity::{EntityId, KindId, PartitionId};
use crate::facade::runtime::RelationalRuntime;
use crate::facade::transactions::{
    BulkEntityCreateIntent, CommitResult, CreateIntent, MutationIntent, RecordRef,
    TransactionOptions, WorkerIntentBatch,
};
use serde_json::json;

use super::seed_catalog::FintechCaseSeed;
use super::{FintechCaseRole, LEDGER_PARTITION, MARKET_PARTITION, RISK_PARTITION};

#[derive(Debug)]
pub(super) struct SeededEntityState {
    pub(super) book_names: Vec<String>,
    pub(super) desk_map: BTreeMap<String, EntityId>,
    pub(super) book_map: BTreeMap<String, EntityId>,
    pub(super) desks: Vec<EntityId>,
    pub(super) books: Vec<EntityId>,
    pub(super) accounts: Vec<EntityId>,
    pub(super) counterparties: Vec<EntityId>,
    pub(super) trades: Vec<EntityId>,
    pub(super) settlements: Vec<EntityId>,
    pub(super) cash_events: Vec<EntityId>,
    pub(super) audit_records: Vec<EntityId>,
    pub(super) instruments: Vec<EntityId>,
    pub(super) market_points: Vec<EntityId>,
    pub(super) risk_views: Vec<EntityId>,
    pub(super) limits: Vec<EntityId>,
    pub(super) breaches: Vec<EntityId>,
}

pub(super) fn seed_entities(
    runtime: &mut RelationalRuntime,
    case_seeds: &[FintechCaseSeed],
) -> SeededEntityState {
    let desk_names = unique_names(case_seeds.iter().map(|seed| seed.desk_name));
    let book_names = unique_names(case_seeds.iter().map(|seed| seed.book_name));

    let desks = bulk_create_entities(
        runtime,
        "seed-desks",
        LEDGER_PARTITION,
        desk_names.iter().enumerate().map(|(idx, name)| {
            (
                format!("desk-{name}"),
                json!({
                    "entity_type": "desk",
                    "desk_name": name,
                    "desk_index": idx,
                }),
            )
        }),
    );
    let books = bulk_create_entities(
        runtime,
        "seed-books",
        LEDGER_PARTITION,
        book_names.iter().enumerate().map(|(idx, name)| {
            (
                format!("book-{name}"),
                json!({
                    "entity_type": "book",
                    "book_name": name,
                    "desk_name": case_seeds
                        .iter()
                        .find(|seed| seed.book_name == *name)
                        .map(|seed| seed.desk_name)
                        .unwrap_or("macro-flow"),
                    "book_index": idx,
                }),
            )
        }),
    );
    let accounts = bulk_create_entities(
        runtime,
        "seed-accounts",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("account-{}", seed.slug),
                json!({
                    "entity_type": "account",
                    "case": seed.slug,
                    "book_name": seed.book_name,
                    "balance_cents": seed.balance_cents,
                }),
            )
        }),
    );
    let counterparties = bulk_create_entities(
        runtime,
        "seed-counterparties",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("counterparty-{}", seed.slug),
                json!({
                    "entity_type": "counterparty",
                    "name": seed.counterparty_name,
                    "rating": seed.counterparty_rating,
                    "case": seed.slug,
                }),
            )
        }),
    );
    let trades = bulk_create_entities(
        runtime,
        "seed-trades",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("trade-{}", seed.slug),
                json!({
                    "entity_type": "trade",
                    "case": seed.slug,
                    "desk": seed.desk_name,
                    "book": seed.book_name,
                    "notional": seed.notional,
                    "ccy": seed.ccy,
                    "correction_candidate": matches!(seed.role, FintechCaseRole::LateTradeCorrection),
                }),
            )
        }),
    );
    let settlements = bulk_create_entities(
        runtime,
        "seed-settlements",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("settlement-{}", seed.slug),
                json!({
                    "entity_type": "settlement",
                    "case": seed.slug,
                    "status": seed.settlement_status,
                }),
            )
        }),
    );
    let cash_events = bulk_create_entities(
        runtime,
        "seed-cash-events",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("cash-event-{}", seed.slug),
                json!({
                    "entity_type": "cash_event",
                    "case": seed.slug,
                    "kind": seed.cash_event_kind,
                }),
            )
        }),
    );
    let audit_records = bulk_create_entities(
        runtime,
        "seed-audit-records",
        LEDGER_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("audit-{}", seed.slug),
                json!({
                    "entity_type": "audit_record",
                    "case": seed.slug,
                    "event": seed.audit_event,
                }),
            )
        }),
    );
    let instruments = bulk_create_entities(
        runtime,
        "seed-instruments",
        MARKET_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("instrument-{}", seed.slug),
                json!({
                    "entity_type": "instrument",
                    "case": seed.slug,
                    "symbol": seed.symbol,
                    "asset_class": seed.asset_class,
                }),
            )
        }),
    );
    let market_points = bulk_create_entities(
        runtime,
        "seed-market",
        MARKET_PARTITION,
        case_seeds.iter().enumerate().map(|(idx, seed)| {
            (
                format!("curve-{}", seed.slug),
                json!({
                    "entity_type": "market_point",
                    "case": seed.slug,
                    "curve_bucket": idx,
                    "mid": seed.market_mid,
                }),
            )
        }),
    );
    let risk_views = bulk_create_entities(
        runtime,
        "seed-risk",
        RISK_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("risk-{}", seed.slug),
                json!({
                    "entity_type": "risk_view",
                    "case": seed.slug,
                    "scenario": seed.risk_scenario,
                }),
            )
        }),
    );
    let limits = bulk_create_entities(
        runtime,
        "seed-limits",
        RISK_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("limit-{}", seed.slug),
                json!({
                    "entity_type": "limit",
                    "case": seed.slug,
                    "threshold_bps": seed.limit_threshold_bps,
                }),
            )
        }),
    );
    let breaches = bulk_create_entities(
        runtime,
        "seed-breaches",
        RISK_PARTITION,
        case_seeds.iter().map(|seed| {
            (
                format!("breach-{}", seed.slug),
                json!({
                    "entity_type": "limit_breach",
                    "case": seed.slug,
                    "status": seed.breach_status,
                }),
            )
        }),
    );

    let desk_map = name_id_map(&desk_names, &desks);
    let book_map = name_id_map(&book_names, &books);

    SeededEntityState {
        book_names,
        desk_map,
        book_map,
        desks,
        books,
        accounts,
        counterparties,
        trades,
        settlements,
        cash_events,
        audit_records,
        instruments,
        market_points,
        risk_views,
        limits,
        breaches,
    }
}

pub(super) fn bulk_create_entities<I>(
    runtime: &mut RelationalRuntime,
    batch_name: &str,
    partition_id: PartitionId,
    specs: I,
) -> Vec<EntityId>
where
    I: IntoIterator<Item = (String, serde_json::Value)>,
{
    let (client_keys, field_patches): (Vec<_>, Vec<_>) = specs
        .into_iter()
        .map(|(key, payload)| {
            (
                crate::facade::symbols::ClientKey::raw(key),
                crate::tests::support::aspect_field_patch_from_compatibility_json(payload),
            )
        })
        .unzip();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new(batch_name).push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id,
                kind_id: KindId(1),
                client_keys,
                field_patches,
            }),
        )),
    );
    changed_entities(&txn.commit().unwrap())
}

fn changed_entities(outcome: &CommitResult) -> Vec<EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Entity(id) => Some(*id),
            RecordRef::Relation(_) => None,
        })
        .collect()
}

fn unique_names<'a>(values: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let mut names = Vec::new();
    for value in values {
        if !names.iter().any(|existing| existing == value) {
            names.push(value.to_string());
        }
    }
    names
}

fn name_id_map(names: &[String], ids: &[EntityId]) -> BTreeMap<String, EntityId> {
    names
        .iter()
        .cloned()
        .zip(ids.iter().copied())
        .collect::<BTreeMap<_, _>>()
}
