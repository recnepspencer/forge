use super::fixture::FintechDomainFixture;
use super::scales::FintechScale;

pub(super) fn assert_world_shape(world: &FintechDomainFixture, scale: FintechScale) {
    assert_eq!(world.instruments.len(), scale.instruments);
    assert_eq!(world.scenario_aggregates.len(), scale.scenarios);
    assert_eq!(world.bucket_aggregates.len(), scale.buckets);
    assert!(
        world
            .instruments
            .iter()
            .all(|instrument| instrument.scenarios.len() == scale.scenarios),
        "every instrument should have one scenario node per scenario"
    );
    assert!(
        world
            .instruments
            .iter()
            .all(|instrument| instrument.buckets.len() == scale.buckets),
        "every instrument should have one bucket node per bucket"
    );
    assert_eq!(world.book_aggregates.len(), scale.books);
    assert_eq!(world.desk_aggregates.len(), scale.desks);
    assert_eq!(world.aggregate_sources.len(), scale.books.max(scale.desks));
    assert_eq!(world.curve_buckets.len(), scale.buckets);
    assert_eq!(world.vol_surface_buckets.len(), scale.buckets);
    assert_eq!(world.scenario_sources.len(), scale.scenarios);
    assert!(
        world.instruments.iter().all(|instrument| world
            .runtime
            .graph()
            .depends_on(
                instrument.core.normalized,
                instrument.core.market,
                super::aspects::PRICE
            )
            .unwrap()),
        "normalized nodes should remain wired to market state"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(world.fx.eur_jpy, world.fx.eur_usd, super::aspects::PRICE)
            .unwrap(),
        "cross FX node should depend on EUR/USD"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(world.fx.eur_jpy, world.fx.usd_jpy, super::aspects::PRICE)
            .unwrap(),
        "cross FX node should depend on USD/JPY"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.book_aggregates[0],
                world.aggregate_sources[0].book_state,
                super::aspects::RISK,
            )
            .unwrap(),
        "book aggregates should depend on model-driven book state"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.desk_aggregates[0],
                world.aggregate_sources[0].desk_limit,
                super::aspects::RISK,
            )
            .unwrap(),
        "desk aggregates should depend on model-driven desk limits"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.instruments[0].scenarios[0],
                world.scenario_sources[0],
                super::aspects::RISK,
            )
            .unwrap(),
        "scenario nodes should depend on scenario shock sources"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.rates_partition_node(),
                world.partitioned_market_source(),
                super::aspects::PRICE,
            )
            .unwrap(),
        "rates locality node should depend on the partitioned market source"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.credit_partition_node(),
                world.partitioned_market_source(),
                super::aspects::PRICE,
            )
            .unwrap(),
        "credit locality node should depend on the partitioned market source"
    );
    assert!(
        world
            .runtime
            .graph()
            .depends_on(
                world.coarse_partition_book_node(),
                world.rates_partition_node(),
                super::aspects::PRICE,
            )
            .unwrap(),
        "coarse locality book should depend on the rates partition view"
    );
}
