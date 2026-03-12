use super::fixture::FintechWorld;
use super::regimes::MarketRegime;
use super::scales::FintechScale;
use super::world_assembly::WorldAssembly;

pub(crate) fn setup_world() -> FintechWorld {
    WorldAssembly::smoke().build()
}

pub(crate) fn setup_seeded_world() -> FintechWorld {
    setup_world()
}

pub(crate) fn setup_seeded_world_with(
    scale: FintechScale,
    regime: MarketRegime,
    seed: u64,
) -> FintechWorld {
    WorldAssembly::new(scale).with_regime(regime, seed).build()
}
