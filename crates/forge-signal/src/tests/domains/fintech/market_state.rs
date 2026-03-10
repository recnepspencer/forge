use crate::facade::{AspectVersion, NodeEvaluationResult, SignalError};

use super::aspects::{ALERT, CURVE, LIQUIDITY, PRICE, RISK, VOL};
use super::fixture::FintechDomainFixture;
use super::model::{AggregateState, CorrelatedMarketModel, MarketPoint, ScenarioShock};
use super::regimes::MarketRegime;

fn point_version(point: MarketPoint) -> AspectVersion {
    AspectVersion::from_updates([
        (PRICE, point.price),
        (VOL, point.vol),
        (CURVE, point.curve),
        (LIQUIDITY, point.liquidity),
        (RISK, point.risk),
        (ALERT, point.alert),
    ])
}

fn aggregate_version(state: AggregateState) -> AspectVersion {
    AspectVersion::from_updates([(RISK, state.risk), (ALERT, state.alert)])
}

fn scenario_version(shock: ScenarioShock) -> AspectVersion {
    AspectVersion::from_updates([(RISK, shock.risk), (VOL, shock.vol)])
}

pub(super) fn seed_market_regime(
    fixture: &mut FintechDomainFixture,
    regime: MarketRegime,
    seed: u64,
) -> Result<(), SignalError> {
    let model = CorrelatedMarketModel::new(regime, seed);
    let mut ctx = ();
    fixture.runtime.transaction(&mut ctx, |tx| {
        for instrument in &fixture.instruments {
            let point = model.market_point(instrument.instrument_index, instrument.book_index);
            let label = format!("market-{}-{:?}", instrument.instrument_index, regime);
            tx.read(instrument.core.market, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(point_version(point))
                        .with_output_identity(label.clone()),
                ))
            })?;
        }

        let fx = model.fx_market();
        tx.read(fixture.fx.eur_usd, &move |_node, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, fx.eur_usd)]))
                    .with_output_identity("eur-usd"),
            ))
        })?;
        tx.read(fixture.fx.usd_jpy, &move |_node, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, fx.usd_jpy)]))
                    .with_output_identity("usd-jpy"),
            ))
        })?;

        let curve_series = model.curve_bucket_series(fixture.curve_buckets.len());
        for (index, node) in fixture.curve_buckets.iter().enumerate() {
            let value = curve_series[index];
            tx.read(*node, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(CURVE, value)]))
                        .with_output_identity(format!("curve-{index}")),
                ))
            })?;
        }

        let vol_series = model.vol_surface_series(fixture.vol_surface_buckets.len());
        for (index, node) in fixture.vol_surface_buckets.iter().enumerate() {
            let value = vol_series[index];
            tx.read(*node, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(VOL, value)]))
                        .with_output_identity(format!("surface-{index}")),
                ))
            })?;
        }

        let scenario_shocks = model.scenario_shocks(fixture.scenario_sources.len());
        for (index, node) in fixture.scenario_sources.iter().enumerate() {
            let shock = scenario_shocks[index];
            tx.read(*node, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(scenario_version(shock))
                        .with_output_identity(format!("scenario-shock-{index}-{seed}")),
                ))
            })?;
        }

        let book_states = model.book_states(fixture.book_aggregates.len());
        for (index, sources) in fixture
            .aggregate_sources
            .iter()
            .take(fixture.book_aggregates.len())
            .enumerate()
        {
            let state = book_states[index];
            tx.read(sources.book_state, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(aggregate_version(state))
                        .with_output_identity(format!("book-state-{index}-{seed}")),
                ))
            })?;
        }

        let desk_limits = model.desk_limits(fixture.desk_aggregates.len());
        for (index, sources) in fixture
            .aggregate_sources
            .iter()
            .take(fixture.desk_aggregates.len())
            .enumerate()
        {
            let state = desk_limits[index];
            tx.read(sources.desk_limit, &move |_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(aggregate_version(state))
                        .with_output_identity(format!("desk-limit-{index}-{seed}")),
                ))
            })?;
        }
        Ok(())
    })?;
    Ok(())
}
