use super::fixture::FintechDomainFixture;
use super::scales::FintechScale;

pub(super) fn assert_fixture_shape(
    fixture: &FintechDomainFixture,
    scale: FintechScale,
) {
    assert_eq!(fixture.instruments.len(), scale.instruments);
    assert_eq!(fixture.scenario_aggregates.len(), scale.scenarios);
    assert_eq!(fixture.bucket_aggregates.len(), scale.buckets);
    assert!(
        fixture
            .instruments
            .iter()
            .all(|instrument| instrument.scenarios.len() == scale.scenarios),
        "every instrument should have one scenario node per scenario"
    );
    assert!(
        fixture
            .instruments
            .iter()
            .all(|instrument| instrument.buckets.len() == scale.buckets),
        "every instrument should have one bucket node per bucket"
    );
    assert!(
        fixture
            .instruments
            .iter()
            .all(|instrument| fixture
                .runtime
                .graph()
                .depends_on(instrument.core.normalized, instrument.core.market, super::aspects::PRICE)
                .unwrap()),
        "normalized nodes should remain wired to market state"
    );
}
