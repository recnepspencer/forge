use std::collections::BTreeMap;

pub(super) const FIXED_SCALE: i64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum Currency {
    Usd,
    Eur,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FxPair {
    EurUsd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum QuoteId {
    TreasuryTwoYear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CurveBucket {
    UsdOneYear,
    UsdTwoYear,
    EurOneYear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VolatilityBucket {
    EurUsdOneYear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MarketFactorKey {
    Quote(QuoteId),
    FxSpot(FxPair),
    Curve(CurveBucket),
    Volatility(VolatilityBucket),
}

impl MarketFactorKey {
    pub(in crate::tests::domains::fintech) const fn partition(
        self,
    ) -> (&'static str, &'static str) {
        match self {
            Self::Quote(QuoteId::TreasuryTwoYear) => ("rates", "treasury-2y"),
            Self::FxSpot(FxPair::EurUsd) => ("fx", "eur-usd"),
            Self::Curve(CurveBucket::UsdOneYear) => ("rates", "usd-1y"),
            Self::Curve(CurveBucket::UsdTwoYear) => ("rates", "usd-2y"),
            Self::Curve(CurveBucket::EurOneYear) => ("rates", "eur-1y"),
            Self::Volatility(VolatilityBucket::EurUsdOneYear) => ("volatility", "eur-usd-1y"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct FixedPrice(i64);

impl FixedPrice {
    pub(super) const fn from_scaled(value: i64) -> Self {
        Self(value)
    }

    pub(super) const fn scaled(self) -> i64 {
        self.0
    }

    pub(super) fn checked_add(self, delta: i64) -> Self {
        Self(self.0.checked_add(delta).expect("financial price overflow"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialMarketInputs {
    quotes: BTreeMap<QuoteId, FixedPrice>,
    fx_spots: BTreeMap<FxPair, FixedPrice>,
    curve_rates: BTreeMap<CurveBucket, FixedPrice>,
    volatilities: BTreeMap<VolatilityBucket, FixedPrice>,
}

impl FinancialMarketInputs {
    pub(super) fn deterministic(seed: u64) -> Self {
        let seed_shift = (seed % 17) as i64 * 100;
        Self {
            quotes: BTreeMap::from([(
                QuoteId::TreasuryTwoYear,
                FixedPrice::from_scaled(985_000 + seed_shift),
            )]),
            fx_spots: BTreeMap::from([(
                FxPair::EurUsd,
                FixedPrice::from_scaled(1_100_000 + seed_shift),
            )]),
            curve_rates: BTreeMap::from([
                (
                    CurveBucket::UsdOneYear,
                    FixedPrice::from_scaled(50_000 + seed_shift / 10),
                ),
                (
                    CurveBucket::UsdTwoYear,
                    FixedPrice::from_scaled(55_000 + seed_shift / 10),
                ),
                (
                    CurveBucket::EurOneYear,
                    FixedPrice::from_scaled(30_000 + seed_shift / 10),
                ),
            ]),
            volatilities: BTreeMap::from([(
                VolatilityBucket::EurUsdOneYear,
                FixedPrice::from_scaled(200_000 + seed_shift),
            )]),
        }
    }

    pub(super) fn value(&self, factor: MarketFactorKey) -> FixedPrice {
        match factor {
            MarketFactorKey::Quote(key) => self.quotes[&key],
            MarketFactorKey::FxSpot(key) => self.fx_spots[&key],
            MarketFactorKey::Curve(key) => self.curve_rates[&key],
            MarketFactorKey::Volatility(key) => self.volatilities[&key],
        }
    }

    pub(super) fn with_factor_delta(&self, factor: MarketFactorKey, delta: i64) -> Self {
        let mut changed = self.clone();
        match factor {
            MarketFactorKey::Quote(key) => {
                changed
                    .quotes
                    .insert(key, self.quotes[&key].checked_add(delta));
            }
            MarketFactorKey::FxSpot(key) => {
                changed
                    .fx_spots
                    .insert(key, self.fx_spots[&key].checked_add(delta));
            }
            MarketFactorKey::Curve(key) => {
                changed
                    .curve_rates
                    .insert(key, self.curve_rates[&key].checked_add(delta));
            }
            MarketFactorKey::Volatility(key) => {
                changed
                    .volatilities
                    .insert(key, self.volatilities[&key].checked_add(delta));
            }
        }
        changed
    }

    pub(super) fn factors(&self) -> impl Iterator<Item = MarketFactorKey> + '_ {
        self.quotes
            .keys()
            .copied()
            .map(MarketFactorKey::Quote)
            .chain(self.fx_spots.keys().copied().map(MarketFactorKey::FxSpot))
            .chain(self.curve_rates.keys().copied().map(MarketFactorKey::Curve))
            .chain(
                self.volatilities
                    .keys()
                    .copied()
                    .map(MarketFactorKey::Volatility),
            )
    }
}
