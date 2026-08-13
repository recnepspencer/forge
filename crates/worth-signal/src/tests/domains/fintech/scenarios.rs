use super::fixture::FintechWorld;
use super::regimes::MarketRegime;
use super::scales::FintechScale;
use super::world::{compile_runtime_fixture, FinancialWorldDefinition};

pub(crate) fn setup_world() -> FintechWorld {
    compile_runtime_fixture(FinancialWorldDefinition::runtime_fixture(
        FintechScale::smoke(),
        MarketRegime::Calm,
        7,
    ))
}

pub(crate) fn setup_seeded_world() -> FintechWorld {
    setup_world()
}

pub(crate) fn setup_seeded_world_with(
    scale: FintechScale,
    regime: MarketRegime,
    seed: u64,
) -> FintechWorld {
    compile_runtime_fixture(FinancialWorldDefinition::runtime_fixture(
        scale, regime, seed,
    ))
}
