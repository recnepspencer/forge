use super::super::fixture::{build_fixture, FintechWorld};
use super::FinancialWorldDefinition;

pub(in crate::tests::domains::fintech) fn compile_runtime_fixture(
    definition: FinancialWorldDefinition,
) -> FintechWorld {
    let mut world = build_fixture(definition.clone());
    world
        .seed_financial_definition(definition)
        .expect("authoritative financial fixture seed must compile");
    world
}

pub(in crate::tests::domains::fintech) fn compile_unseeded_runtime_fixture(
    definition: FinancialWorldDefinition,
) -> FintechWorld {
    build_fixture(definition)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::{
        aspects::PRICE, regimes::MarketRegime, scales::FintechScale,
    };

    #[test]
    fn runtime_fixture_retains_definition_and_separates_value_from_revision() {
        let calm =
            FinancialWorldDefinition::runtime_fixture(FintechScale::smoke(), MarketRegime::Calm, 7);
        let mut world = compile_runtime_fixture(calm.clone());
        let source = world.primary_market_source();

        assert_eq!(world.financial_definition, calm);
        assert_eq!(world.market_revision, 1);
        assert_eq!(
            world
                .runtime
                .graph()
                .node_aspect_version(source)
                .unwrap()
                .get(PRICE),
            1
        );

        let high_vol = FinancialWorldDefinition::runtime_fixture(
            FintechScale::smoke(),
            MarketRegime::HighVol,
            7,
        );
        world.seed_financial_definition(high_vol.clone()).unwrap();

        assert_eq!(world.financial_definition, high_vol);
        assert_eq!(world.market_revision, 2);
        assert_eq!(
            world
                .runtime
                .graph()
                .node_aspect_version(source)
                .unwrap()
                .get(PRICE),
            2
        );
    }
}
