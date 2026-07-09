use super::super::fixture::FintechWorld;
use super::super::scales::FintechScale;

pub(crate) fn assert_fixture_shape(world: &FintechWorld, scale: FintechScale) {
    assert_eq!(world.ledger.accounts.len(), scale.accounts);
    assert_eq!(world.ledger.trades.len(), scale.trades);
    assert_eq!(world.market.market_points.len(), scale.market_points);
    assert_eq!(world.market.instruments.len(), scale.trades);
    assert_eq!(world.risk.risk_views.len(), scale.trades);
    assert_eq!(world.risk.limits.len(), scale.trades);
    assert_eq!(world.risk.breaches.len(), scale.trades);
    assert!(!world.ledger.desks.is_empty());
    assert!(!world.ledger.books.is_empty());
    assert!(!world.ledger.counterparties.is_empty());
    assert_eq!(world.ledger.settlements.len(), scale.trades);
    assert_eq!(world.ledger.cash_events.len(), scale.trades);
    assert_eq!(world.ledger.audit_records.len(), scale.trades);
    assert!(!world.relations.is_empty());
}
