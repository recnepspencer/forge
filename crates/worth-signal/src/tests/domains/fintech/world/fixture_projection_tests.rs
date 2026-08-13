use super::{FinancialFixtureProjection, FinancialWorldDefinition};
use crate::tests::domains::fintech::{regimes::MarketRegime, scales::FintechScale};

#[test]
fn fintech_regime_library_exposes_distinct_market_shapes() {
    let projection = |regime| {
        FinancialFixtureProjection::from_definition(&FinancialWorldDefinition::runtime_fixture(
            FintechScale::smoke(),
            regime,
            3,
        ))
    };
    let calm = projection(MarketRegime::Calm).market_point(0, 0);
    let blowout = projection(MarketRegime::SpreadBlowout).market_point(0, 0);
    let curve = projection(MarketRegime::CurveShock).market_point(0, 0);
    let fx = projection(MarketRegime::FxDislocation).market_point(0, 0);

    assert!(blowout.liquidity > calm.liquidity);
    assert!(curve.curve > calm.curve);
    assert!(fx.risk > calm.risk);
}

#[test]
fn fintech_market_model_preserves_cross_rate_and_bucket_shape() {
    let model =
        FinancialFixtureProjection::from_definition(&FinancialWorldDefinition::runtime_fixture(
            FintechScale::smoke(),
            MarketRegime::FxDislocation,
            41,
        ));
    let fx = model.fx_market();
    let curves = model.curve_bucket_series(5);
    let vols = model.vol_surface_series(5);
    let shocks = model.scenario_shocks(4);
    let books = model.book_states(3);
    let desks = model.desk_limits(2);

    assert_eq!(fx.eur_jpy, fx.eur_usd.saturating_mul(fx.usd_jpy) / 10_000);
    assert_eq!(curves.len(), 5);
    assert_eq!(vols.len(), 5);
    assert_eq!(shocks.len(), 4);
    assert_eq!(books.len(), 3);
    assert_eq!(desks.len(), 2);
    assert!(curves.windows(2).all(|w| w[0] != w[1]));
    assert!(vols.windows(2).all(|w| w[0] != w[1]));
    assert!(shocks.iter().all(|shock| shock.risk > shock.vol));
}

#[test]
fn fintech_market_model_uses_deterministic_probability_windows() {
    let definition =
        |regime| FinancialWorldDefinition::runtime_fixture(FintechScale::smoke(), regime, 7);
    let calm_a = FinancialFixtureProjection::from_definition(&definition(MarketRegime::Calm))
        .regime_window_signature();
    let calm_b = FinancialFixtureProjection::from_definition(&definition(MarketRegime::Calm))
        .regime_window_signature();
    let vol = FinancialFixtureProjection::from_definition(&definition(MarketRegime::HighVol))
        .regime_window_signature();

    assert_eq!(calm_a, calm_b);
    assert_ne!(calm_a, vol);
}
