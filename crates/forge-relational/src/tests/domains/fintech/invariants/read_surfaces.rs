use crate::facade::runtime::RelationalReadView;
use crate::tests::support::read_entity_field;

use super::super::fixture::{LEDGER_PARTITION, MARKET_PARTITION, RISK_PARTITION};

pub(crate) fn assert_partitioned_aspect_state(read: &RelationalReadView) {
    let mut saw_ledger = false;
    let mut saw_market = false;
    let mut saw_risk = false;
    let mut saw_instrument = false;
    let mut saw_settlement = false;
    let mut saw_limit = false;
    for entity in read.entities() {
        match entity.entity_id.partition_id {
            LEDGER_PARTITION => saw_ledger = true,
            MARKET_PARTITION => saw_market = true,
            RISK_PARTITION => saw_risk = true,
            _ => {}
        }
        if let Some(entity_type) = read_entity_field(entity, "entity_type") {
            match entity_type.as_ref() {
                "instrument" => saw_instrument = true,
                "settlement" => saw_settlement = true,
                "limit" => saw_limit = true,
                _ => {}
            }
        }
        if entity.entity_id.partition_id == MARKET_PARTITION {
            assert!(entity.authoritative_aspect_state.is_some());
        }
    }
    assert!(saw_ledger && saw_market && saw_risk);
    assert!(saw_instrument && saw_settlement && saw_limit);
}

pub(crate) fn assert_cross_context_relations(read: &RelationalReadView) {
    assert!(read.relations().iter().any(|relation| {
        relation.source.partition_id != relation.target.partition_id
            && matches!(
                (relation.source.partition_id, relation.target.partition_id),
                (LEDGER_PARTITION, MARKET_PARTITION)
                    | (LEDGER_PARTITION, RISK_PARTITION)
                    | (MARKET_PARTITION, LEDGER_PARTITION)
                    | (RISK_PARTITION, LEDGER_PARTITION)
            )
    }));
}
