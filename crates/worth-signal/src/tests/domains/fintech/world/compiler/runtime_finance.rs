use std::collections::BTreeMap;

use super::super::{
    FinancialAmount, FinancialEconomicSnapshot, FinancialWorldDefinition, MarketFactorKey,
    PositionFinancialResult, PositionKind, FIXED_SCALE,
};

pub(super) fn runtime_financial_snapshot(
    definition: &FinancialWorldDefinition,
) -> FinancialEconomicSnapshot {
    let results = definition
        .positions()
        .iter()
        .map(|position| {
            let result = match position.kind {
                PositionKind::ZeroCouponBond {
                    quote,
                    curve,
                    maturity_years,
                } => {
                    let quote = definition
                        .market()
                        .value(MarketFactorKey::Quote(quote))
                        .scaled();
                    let rate = definition
                        .market()
                        .value(MarketFactorKey::Curve(curve))
                        .scaled();
                    runtime_bond(position.quantity_micros, quote, rate, maturity_years)
                }
                PositionKind::FxForward {
                    pair,
                    strike,
                    domestic_curve,
                    foreign_curve,
                    maturity_years,
                } => runtime_fx_forward(
                    position.quantity_micros,
                    definition
                        .market()
                        .value(MarketFactorKey::FxSpot(pair))
                        .scaled(),
                    strike.scaled(),
                    definition
                        .market()
                        .value(MarketFactorKey::Curve(domestic_curve))
                        .scaled(),
                    definition
                        .market()
                        .value(MarketFactorKey::Curve(foreign_curve))
                        .scaled(),
                    maturity_years,
                ),
                PositionKind::VarianceSwap {
                    volatility,
                    strike_variance,
                    maturity_years,
                } => runtime_variance_swap(
                    position.quantity_micros,
                    definition
                        .market()
                        .value(MarketFactorKey::Volatility(volatility))
                        .scaled(),
                    strike_variance.scaled(),
                    maturity_years,
                ),
            };
            (position.instrument, result)
        })
        .collect::<BTreeMap<_, _>>();
    FinancialEconomicSnapshot::from_results(
        definition.market(),
        definition.positions(),
        results,
        definition
            .consumers()
            .iter()
            .map(|consumer| (consumer.role, consumer.position)),
    )
}

fn runtime_bond(quantity: i64, quote: i64, rate: i64, years: u32) -> PositionFinancialResult {
    let marked = checked_ratio(quantity, quote, FIXED_SCALE);
    let value = bond_value(marked, rate, years);
    let down = bond_value(marked, rate - 100, years);
    let up = bond_value(marked, rate + 100, years);
    PositionFinancialResult {
        value: FinancialAmount::from_micros(value),
        risk: FinancialAmount::from_micros(down.abs_diff(up) as i64 / 2),
    }
}

fn bond_value(marked: i64, rate: i64, years: u32) -> i64 {
    let accrual = rate
        .checked_mul(i64::from(years))
        .expect("runtime bond accrual overflow");
    checked_ratio(marked, FIXED_SCALE, FIXED_SCALE + accrual)
}

fn runtime_fx_forward(
    quantity: i64,
    spot: i64,
    strike: i64,
    domestic: i64,
    foreign: i64,
    years: u32,
) -> PositionFinancialResult {
    let years = i64::from(years);
    let forward = checked_ratio(
        spot,
        FIXED_SCALE + domestic * years,
        FIXED_SCALE + foreign * years,
    );
    let value = checked_ratio(quantity, forward - strike, FIXED_SCALE);
    let risk = checked_ratio(quantity, spot, FIXED_SCALE)
        .abs()
        .checked_add(value.abs())
        .expect("runtime FX forward risk overflow");
    PositionFinancialResult {
        value: FinancialAmount::from_micros(value),
        risk: FinancialAmount::from_micros(risk),
    }
}

fn runtime_variance_swap(
    quantity: i64,
    volatility: i64,
    strike_variance: i64,
    years: u32,
) -> PositionFinancialResult {
    let variance = checked_ratio(volatility, volatility, FIXED_SCALE) * i64::from(years);
    let value = checked_ratio(quantity, variance - strike_variance, FIXED_SCALE);
    let vega = checked_ratio(
        quantity
            .checked_mul(2)
            .expect("runtime variance quantity overflow"),
        volatility,
        FIXED_SCALE,
    );
    PositionFinancialResult {
        value: FinancialAmount::from_micros(value),
        risk: FinancialAmount::from_micros(vega.abs()),
    }
}

fn checked_ratio(left: i64, right: i64, divisor: i64) -> i64 {
    i64::try_from(
        i128::from(left)
            .checked_mul(i128::from(right))
            .expect("runtime fixed-point multiplication overflow")
            / i128::from(divisor),
    )
    .expect("runtime fixed-point result exceeds i64")
}
