use super::{FinancialMarketInputs, FinancialPosition, MarketFactorKey, PositionKind, FIXED_SCALE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct FinancialAmount(i64);

impl FinancialAmount {
    pub(super) const fn from_micros(value: i64) -> Self {
        Self(value)
    }

    pub(in crate::tests::domains::fintech) const fn micros(self) -> i64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct PositionFinancialResult {
    pub(in crate::tests::domains::fintech) value: FinancialAmount,
    pub(in crate::tests::domains::fintech) risk: FinancialAmount,
}

pub(in crate::tests::domains::fintech) fn reference_position_result(
    market: &FinancialMarketInputs,
    position: &FinancialPosition,
) -> PositionFinancialResult {
    match position.kind {
        PositionKind::ZeroCouponBond {
            quote,
            curve,
            maturity_years,
        } => reference_zero_coupon_bond(
            position.quantity_micros,
            market.value(MarketFactorKey::Quote(quote)).scaled(),
            market.value(MarketFactorKey::Curve(curve)).scaled(),
            maturity_years,
        ),
        PositionKind::FxForward {
            pair,
            strike,
            domestic_curve,
            foreign_curve,
            maturity_years,
        } => reference_fx_forward(
            position.quantity_micros,
            market.value(MarketFactorKey::FxSpot(pair)).scaled(),
            strike.scaled(),
            market
                .value(MarketFactorKey::Curve(domestic_curve))
                .scaled(),
            market.value(MarketFactorKey::Curve(foreign_curve)).scaled(),
            maturity_years,
        ),
        PositionKind::VarianceSwap {
            volatility,
            strike_variance,
            maturity_years,
        } => reference_variance_swap(
            position.quantity_micros,
            market
                .value(MarketFactorKey::Volatility(volatility))
                .scaled(),
            strike_variance.scaled(),
            maturity_years,
        ),
    }
}

fn reference_zero_coupon_bond(
    quantity_micros: i64,
    quoted_clean_price: i64,
    annual_yield: i64,
    maturity_years: u32,
) -> PositionFinancialResult {
    let marked_notional = mul_div(quantity_micros, quoted_clean_price, FIXED_SCALE);
    let present_value = discount_simple(marked_notional, annual_yield, maturity_years);
    let lower = discount_simple(marked_notional, annual_yield - 100, maturity_years);
    let upper = discount_simple(marked_notional, annual_yield + 100, maturity_years);
    PositionFinancialResult {
        value: FinancialAmount::from_micros(present_value),
        risk: FinancialAmount::from_micros(lower.abs_diff(upper) as i64 / 2),
    }
}

fn reference_fx_forward(
    quantity_micros: i64,
    spot: i64,
    strike: i64,
    domestic_rate: i64,
    foreign_rate: i64,
    maturity_years: u32,
) -> PositionFinancialResult {
    let years = i64::from(maturity_years);
    let domestic_growth = FIXED_SCALE + domestic_rate * years;
    let foreign_growth = FIXED_SCALE + foreign_rate * years;
    let fair_forward = mul_div(spot, domestic_growth, foreign_growth);
    let value = mul_div(quantity_micros, fair_forward - strike, FIXED_SCALE);
    let dollar_delta = mul_div(quantity_micros, spot, FIXED_SCALE).abs();
    let gross_risk = dollar_delta
        .checked_add(value.abs())
        .expect("FX forward gross-risk overflow");
    PositionFinancialResult {
        value: FinancialAmount::from_micros(value),
        risk: FinancialAmount::from_micros(gross_risk),
    }
}

fn reference_variance_swap(
    quantity_micros: i64,
    volatility: i64,
    strike_variance: i64,
    maturity_years: u32,
) -> PositionFinancialResult {
    let fair_variance = mul_div(volatility, volatility, FIXED_SCALE);
    let time_weighted_variance = fair_variance * i64::from(maturity_years);
    let value = mul_div(
        quantity_micros,
        time_weighted_variance - strike_variance,
        FIXED_SCALE,
    );
    let vega = mul_div(
        2_i64
            .checked_mul(quantity_micros)
            .expect("variance vega quantity overflow"),
        volatility,
        FIXED_SCALE,
    )
    .abs();
    PositionFinancialResult {
        value: FinancialAmount::from_micros(value),
        risk: FinancialAmount::from_micros(vega),
    }
}

fn discount_simple(notional: i64, annual_rate: i64, years: u32) -> i64 {
    let denominator = FIXED_SCALE
        .checked_add(
            annual_rate
                .checked_mul(i64::from(years))
                .expect("discount tenor overflow"),
        )
        .expect("discount denominator overflow");
    mul_div(notional, FIXED_SCALE, denominator)
}

fn mul_div(left: i64, right: i64, denominator: i64) -> i64 {
    let value = i128::from(left)
        .checked_mul(i128::from(right))
        .expect("fixed-point multiplication overflow")
        / i128::from(denominator);
    i64::try_from(value).expect("fixed-point result exceeds i64")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::domains::fintech::world::FinancialWorldDefinition;

    #[test]
    fn reference_formulas_match_independently_fixed_financial_table() {
        let definition = FinancialWorldDefinition::deterministic(41);
        let results = definition
            .positions()
            .iter()
            .map(|position| {
                (
                    position.instrument.0,
                    reference_position_result(definition.market(), position),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(results["UST-2Y-ZERO"].value.micros(), 887_906_029_870);
        assert_eq!(results["UST-2Y-ZERO"].risk.micros(), 159_962_898);
        assert_eq!(results["EURUSD-1Y-FWD"].value.micros(), 35_355_000_000);
        assert_eq!(results["EURUSD-1Y-FWD"].risk.micros(), 5_538_855_000_000);
        assert_eq!(results["EURUSD-1Y-VAR"].value.micros(), 4_560_000_000);
        assert_eq!(results["EURUSD-1Y-VAR"].risk.micros(), 802_800_000_000);
    }

    #[test]
    fn reference_fx_risk_changes_when_the_spot_quote_changes() {
        let base = FinancialWorldDefinition::deterministic(41);
        let shocked = base.with_market_factor_delta(
            MarketFactorKey::FxSpot(super::super::FxPair::EurUsd),
            20_000,
        );
        let position = base.position(super::super::InstrumentId("EURUSD-1Y-FWD"));
        let before = reference_position_result(base.market(), position);
        let after = reference_position_result(shocked.market(), position);

        assert_ne!(before.value, after.value);
        assert_ne!(before.risk, after.risk);
    }
}
