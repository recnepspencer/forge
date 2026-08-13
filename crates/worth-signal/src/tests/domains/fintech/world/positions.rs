use super::{
    Currency, CurveBucket, FixedPrice, FxPair, MarketFactorKey, QuoteId, VolatilityBucket,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct InstrumentId(
    pub(in crate::tests::domains::fintech) &'static str,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct BookId(
    pub(in crate::tests::domains::fintech) &'static str,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct DeskId(
    pub(in crate::tests::domains::fintech) &'static str,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialAspect {
    Price,
    Curve,
    Volatility,
    Risk,
    Alert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum PricingModel {
    DiscountedCashFlow,
    FxForward,
    VarianceSwap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialComparatorPolicy {
    Exact,
    Tolerance { epsilon: u64 },
    InstalledTolerance { epsilon: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialOutputEquivalencePolicy {
    Exact,
    Tolerance { epsilon: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FactorSubscription {
    pub(in crate::tests::domains::fintech) factor: MarketFactorKey,
    pub(in crate::tests::domains::fintech) input_aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) partition: &'static str,
    pub(in crate::tests::domains::fintech) detail: &'static str,
}

impl FactorSubscription {
    fn new(factor: MarketFactorKey, input_aspect: FinancialAspect) -> Self {
        let (partition, detail) = factor.partition();
        Self {
            factor,
            input_aspect,
            partition,
            detail,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum PositionKind {
    ZeroCouponBond {
        quote: QuoteId,
        curve: CurveBucket,
        maturity_years: u32,
    },
    FxForward {
        pair: FxPair,
        strike: FixedPrice,
        domestic_curve: CurveBucket,
        foreign_curve: CurveBucket,
        maturity_years: u32,
    },
    VarianceSwap {
        volatility: VolatilityBucket,
        strike_variance: FixedPrice,
        maturity_years: u32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialPosition {
    pub(in crate::tests::domains::fintech) instrument: InstrumentId,
    pub(in crate::tests::domains::fintech) quantity_micros: i64,
    pub(in crate::tests::domains::fintech) currency: Currency,
    pub(in crate::tests::domains::fintech) book: BookId,
    pub(in crate::tests::domains::fintech) desk: DeskId,
    pub(in crate::tests::domains::fintech) model: PricingModel,
    pub(in crate::tests::domains::fintech) kind: PositionKind,
    pub(in crate::tests::domains::fintech) subscriptions: Vec<FactorSubscription>,
}

impl FinancialPosition {
    pub(super) fn rates_bond() -> Self {
        let quote = QuoteId::TreasuryTwoYear;
        let curve = CurveBucket::UsdTwoYear;
        Self {
            instrument: InstrumentId("UST-2Y-ZERO"),
            quantity_micros: 1_000_000_000_000,
            currency: Currency::Usd,
            book: BookId("rates-book"),
            desk: DeskId("macro-desk"),
            model: PricingModel::DiscountedCashFlow,
            kind: PositionKind::ZeroCouponBond {
                quote,
                curve,
                maturity_years: 2,
            },
            subscriptions: vec![
                FactorSubscription::new(MarketFactorKey::Quote(quote), FinancialAspect::Price),
                FactorSubscription::new(MarketFactorKey::Curve(curve), FinancialAspect::Curve),
            ],
        }
    }

    pub(super) fn fx_forward() -> Self {
        let pair = FxPair::EurUsd;
        let domestic_curve = CurveBucket::UsdOneYear;
        let foreign_curve = CurveBucket::EurOneYear;
        Self {
            instrument: InstrumentId("EURUSD-1Y-FWD"),
            quantity_micros: 5_000_000_000_000,
            currency: Currency::Usd,
            book: BookId("fx-book"),
            desk: DeskId("macro-desk"),
            model: PricingModel::FxForward,
            kind: PositionKind::FxForward {
                pair,
                strike: FixedPrice::from_scaled(1_115_000),
                domestic_curve,
                foreign_curve,
                maturity_years: 1,
            },
            subscriptions: vec![
                FactorSubscription::new(MarketFactorKey::FxSpot(pair), FinancialAspect::Price),
                FactorSubscription::new(
                    MarketFactorKey::Curve(domestic_curve),
                    FinancialAspect::Curve,
                ),
                FactorSubscription::new(
                    MarketFactorKey::Curve(foreign_curve),
                    FinancialAspect::Curve,
                ),
            ],
        }
    }

    pub(super) fn variance_swap() -> Self {
        let volatility = VolatilityBucket::EurUsdOneYear;
        Self {
            instrument: InstrumentId("EURUSD-1Y-VAR"),
            quantity_micros: 2_000_000_000_000,
            currency: Currency::Eur,
            book: BookId("vol-book"),
            desk: DeskId("options-desk"),
            model: PricingModel::VarianceSwap,
            kind: PositionKind::VarianceSwap {
                volatility,
                strike_variance: FixedPrice::from_scaled(38_000),
                maturity_years: 1,
            },
            subscriptions: vec![FactorSubscription::new(
                MarketFactorKey::Volatility(volatility),
                FinancialAspect::Volatility,
            )],
        }
    }
}
