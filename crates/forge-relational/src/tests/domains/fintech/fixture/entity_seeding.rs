use std::collections::BTreeMap;

use crate::facade::identity::{EntityId, KindId, PartitionId};
use crate::facade::runtime::RelationalRuntime;
use crate::facade::transactions::{
    AspectFieldPatch, BulkEntityCreateIntent, CommitResult, CreateIntent, MutationIntent,
    RecordRef, TransactionOptions, WorkerIntentBatch,
};
use crate::tests::support::{
    aspect_field_patch_from_values, bool_aspect_value, field_key, fixture_i64_number_aspect_value,
    string_aspect_value, u64_aspect_value, usize_aspect_value,
};

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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("desk")),
                    (field_key("desk_name"), string_aspect_value(name)),
                    (field_key("desk_index"), usize_aspect_value(idx)),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("book")),
                    (field_key("book_name"), string_aspect_value(name)),
                    (
                        field_key("desk_name"),
                        string_aspect_value(
                            case_seeds
                                .iter()
                                .find(|seed| seed.book_name == *name)
                                .map(|seed| seed.desk_name)
                                .unwrap_or("macro-flow"),
                        ),
                    ),
                    (field_key("book_index"), usize_aspect_value(idx)),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("account")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("book_name"), string_aspect_value(seed.book_name)),
                    (
                        field_key("balance_cents"),
                        fixture_i64_number_aspect_value(seed.balance_cents),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (
                        field_key("entity_type"),
                        string_aspect_value("counterparty"),
                    ),
                    (
                        field_key("name"),
                        string_aspect_value(seed.counterparty_name),
                    ),
                    (
                        field_key("rating"),
                        string_aspect_value(seed.counterparty_rating),
                    ),
                    (field_key("case"), string_aspect_value(seed.slug)),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("trade")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("desk"), string_aspect_value(seed.desk_name)),
                    (field_key("book"), string_aspect_value(seed.book_name)),
                    (
                        field_key("notional"),
                        fixture_i64_number_aspect_value(seed.notional),
                    ),
                    (field_key("ccy"), string_aspect_value(seed.ccy)),
                    (
                        field_key("correction_candidate"),
                        bool_aspect_value(matches!(
                            seed.role,
                            FintechCaseRole::LateTradeCorrection
                        )),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("settlement")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (
                        field_key("status"),
                        string_aspect_value(seed.settlement_status),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("cash_event")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("kind"), string_aspect_value(seed.cash_event_kind)),
                ]),
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
                aspect_field_patch_from_values([
                    (
                        field_key("entity_type"),
                        string_aspect_value("audit_record"),
                    ),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("event"), string_aspect_value(seed.audit_event)),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("instrument")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("symbol"), string_aspect_value(seed.symbol)),
                    (
                        field_key("asset_class"),
                        string_aspect_value(seed.asset_class),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (
                        field_key("entity_type"),
                        string_aspect_value("market_point"),
                    ),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("curve_bucket"), usize_aspect_value(idx)),
                    (
                        field_key("mid"),
                        fixture_i64_number_aspect_value(seed.market_mid),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("risk_view")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (
                        field_key("scenario"),
                        string_aspect_value(seed.risk_scenario),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (field_key("entity_type"), string_aspect_value("limit")),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (
                        field_key("threshold_bps"),
                        u64_aspect_value(seed.limit_threshold_bps as u64),
                    ),
                ]),
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
                aspect_field_patch_from_values([
                    (
                        field_key("entity_type"),
                        string_aspect_value("limit_breach"),
                    ),
                    (field_key("case"), string_aspect_value(seed.slug)),
                    (field_key("status"), string_aspect_value(seed.breach_status)),
                ]),
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
    I: IntoIterator<Item = (String, AspectFieldPatch)>,
{
    let (client_keys, field_patches): (Vec<_>, Vec<_>) = specs
        .into_iter()
        .map(|(key, fields)| (crate::facade::symbols::ClientKey::raw(key), fields))
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
