use super::fixture::{build_fixture, FintechWorld};
use super::market_seed::MarketSeed;
use super::regimes::MarketRegime;
use super::scales::FintechScale;

#[derive(Clone, Copy, Debug)]
pub(super) enum FintechScenario {
    IntradayPricingAndRisk,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct WorldAssembly {
    scale: FintechScale,
    scenario: FintechScenario,
    market_seed: Option<MarketSeed>,
}

impl WorldAssembly {
    pub(super) fn smoke() -> Self {
        Self::new(FintechScale::smoke())
    }

    pub(super) fn new(scale: FintechScale) -> Self {
        Self {
            scale,
            scenario: FintechScenario::IntradayPricingAndRisk,
            market_seed: Some(MarketSeed::calm(7)),
        }
    }

    pub(super) fn without_market_seed(mut self) -> Self {
        self.market_seed = None;
        self
    }

    pub(super) fn with_regime(mut self, regime: MarketRegime, seed: u64) -> Self {
        self.market_seed = Some(MarketSeed::new(regime, seed));
        self
    }

    pub(super) fn build(self) -> FintechWorld {
        let mut world = match self.scenario {
            FintechScenario::IntradayPricingAndRisk => build_fixture(self.scale),
        };
        if let Some(market_seed) = self.market_seed {
            world.seed_market(market_seed).unwrap();
        }
        world
    }
}
