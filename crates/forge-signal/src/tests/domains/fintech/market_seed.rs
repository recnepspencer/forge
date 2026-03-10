use super::regimes::MarketRegime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MarketSeed {
    pub regime: MarketRegime,
    pub seed: u64,
}

impl MarketSeed {
    pub(super) fn new(regime: MarketRegime, seed: u64) -> Self {
        Self { regime, seed }
    }

    pub(super) fn calm(seed: u64) -> Self {
        Self::new(MarketRegime::Calm, seed)
    }

    pub(super) fn high_vol(seed: u64) -> Self {
        Self::new(MarketRegime::HighVol, seed)
    }

    pub(super) fn fx_dislocation(seed: u64) -> Self {
        Self::new(MarketRegime::FxDislocation, seed)
    }
}
