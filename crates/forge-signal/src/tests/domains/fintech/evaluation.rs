use crate::data::handle::NodeId;
use crate::data::error::SignalError;
use crate::facade::{AspectVersion, NodeEvaluationResult};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};
use crate::data::output::PartitionSubscription;

use super::aspects::{ALERT, CURVE, LIQUIDITY, PRICE, RISK, VOL};
use super::fixture::FintechDomainFixture;
use super::node_families::{AggregateSourceNodes, FxNodes, InstrumentNodes};
use super::partition_surface::PartitionSurfaceNodes;

#[derive(Clone)]
pub(super) struct FintechEvaluationShape {
    fx: FxNodes,
    aggregate_sources: Vec<AggregateSourceNodes>,
    curve_buckets: Vec<NodeId>,
    vol_surface_buckets: Vec<NodeId>,
    scenario_sources: Vec<NodeId>,
    instruments: Vec<InstrumentShape>,
    book_aggregates: Vec<NodeId>,
    desk_aggregates: Vec<NodeId>,
    scenario_aggregates: Vec<NodeId>,
    bucket_aggregates: Vec<NodeId>,
    partition: PartitionSurfaceNodes,
}

#[derive(Clone)]
struct InstrumentShape {
    book_index: usize,
    core: InstrumentNodes,
    buckets: Vec<NodeId>,
    scenarios: Vec<NodeId>,
}

impl FintechEvaluationShape {
    pub(super) fn from_fixture(fixture: &FintechDomainFixture) -> Self {
        Self {
            fx: fixture.fx,
            aggregate_sources: fixture.aggregate_sources.clone(),
            curve_buckets: fixture.curve_buckets.clone(),
            vol_surface_buckets: fixture.vol_surface_buckets.clone(),
            scenario_sources: fixture.scenario_sources.clone(),
            instruments: fixture
                .instruments
                .iter()
                .map(|instrument| InstrumentShape {
                    book_index: instrument.book_index,
                    core: instrument.core,
                    buckets: instrument.buckets.clone(),
                    scenarios: instrument.scenarios.clone(),
                })
                .collect(),
            book_aggregates: fixture.book_aggregates.clone(),
            desk_aggregates: fixture.desk_aggregates.clone(),
            scenario_aggregates: fixture.scenario_aggregates.clone(),
            bucket_aggregates: fixture.bucket_aggregates.clone(),
            partition: fixture.handles.partition,
        }
    }

    pub(super) fn precompute(
        &self,
    ) -> impl Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync + '_ {
        move |node, view| self.precompute_node(node, view)
    }

    fn precompute_node(
        &self,
        node: NodeId,
        view: &ExecutionReadView<'_>,
    ) -> Result<PreparedEvaluation, SignalError> {
        if node == self.fx.eur_jpy {
            let eur_usd = view.read_aspect_version(self.fx.eur_usd, PRICE)?.get(PRICE);
            let usd_jpy = view.read_aspect_version(self.fx.usd_jpy, PRICE)?.get(PRICE);
            let eur_jpy = eur_usd.saturating_mul(usd_jpy) / 10_000;
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, eur_jpy)]))
                    .with_output_identity(format!("eur-jpy-{eur_jpy}"))
                    .with_continuity_token("fx-cross"),
            ));
        }

        for instrument in &self.instruments {
            if node == instrument.core.normalized {
                let price = view.read_aspect_version(instrument.core.market, PRICE)?.get(PRICE);
                let vol = view.read_aspect_version(instrument.core.market, VOL)?.get(VOL);
                let curve = view.read_aspect_version(instrument.core.market, CURVE)?.get(CURVE);
                let liquidity = view
                    .read_aspect_version(instrument.core.market, LIQUIDITY)?
                    .get(LIQUIDITY);
                let risk_hint = price / 24 + vol + curve / 20 + liquidity / 12;
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([
                        (PRICE, price),
                        (VOL, vol),
                        (CURVE, curve),
                        (LIQUIDITY, liquidity),
                        (RISK, risk_hint),
                    ]))
                    .with_output_identity(format!("normalized-{price}-{vol}-{curve}-{liquidity}"))
                    .with_continuity_token("normalized"),
                ));
            }

            if node == instrument.core.price {
                let price = view.read_aspect_version(instrument.core.normalized, PRICE)?.get(PRICE);
                let vol = view.read_aspect_version(instrument.core.normalized, VOL)?.get(VOL);
                let curve = view.read_aspect_version(instrument.core.normalized, CURVE)?.get(CURVE);
                let priced = price + vol / 8 + curve / 32;
                let priced_risk = priced / 3 + vol / 2 + curve / 6;
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([
                        (PRICE, priced),
                        (RISK, priced_risk),
                    ]))
                    .with_output_identity(format!("price-{priced}-{priced_risk}"))
                    .with_continuity_token("price"),
                ));
            }

            if node == instrument.core.risk {
                let priced_risk = view.read_aspect_version(instrument.core.price, RISK)?.get(RISK);
                let liquidity = view
                    .read_aspect_version(instrument.core.normalized, LIQUIDITY)?
                    .get(LIQUIDITY);
                let risk = priced_risk + liquidity / 3;
                let alert = u64::from(risk > 1_600);
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([
                        (RISK, risk),
                        (ALERT, alert),
                    ]))
                    .with_output_identity(format!("risk-{risk}-{alert}"))
                    .with_continuity_token("risk"),
                ));
            }

            if node == instrument.core.alert {
                let alert = view.read_aspect_version(instrument.core.risk, ALERT)?.get(ALERT);
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(ALERT, alert)]))
                        .with_output_identity(format!("alert-{alert}"))
                        .with_continuity_token("alert"),
                ));
            }

            if node == instrument.core.threshold {
                let price = view.read_aspect_version(instrument.core.price, PRICE)?.get(PRICE);
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, price)]))
                        .with_output_identity(format!("threshold-{price}"))
                        .with_continuity_token("threshold"),
                ));
            }

            if let Some(bucket_index) = instrument.buckets.iter().position(|candidate| *candidate == node) {
                let risk = view.read_aspect_version(instrument.core.risk, RISK)?.get(RISK);
                let threshold = view
                    .read_aspect_version(instrument.core.threshold, PRICE)?
                    .get(PRICE);
                let curve = view.read_aspect_version(self.curve_buckets[bucket_index], CURVE)?.get(CURVE);
                let surface_vol = view
                    .read_aspect_version(self.vol_surface_buckets[bucket_index], VOL)?
                    .get(VOL);
                let bucket_risk = risk + threshold / 5 + curve / 9 + surface_vol / 7;
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(RISK, bucket_risk)]))
                        .with_output_identity(format!("bucket-{bucket_index}-{bucket_risk}"))
                        .with_continuity_token("bucket-risk"),
                ));
            }

            if let Some(scenario_index) = instrument
                .scenarios
                .iter()
                .position(|candidate| *candidate == node)
            {
                let price = view.read_aspect_version(instrument.core.price, PRICE)?.get(PRICE);
                let risk = view.read_aspect_version(instrument.core.risk, RISK)?.get(RISK);
                let alert = view.read_aspect_version(instrument.core.alert, ALERT)?.get(ALERT);
                let scenario_risk = view
                    .read_aspect_version(self.scenario_sources[scenario_index], RISK)?
                    .get(RISK);
                let scenario_vol = view
                    .read_aspect_version(self.scenario_sources[scenario_index], VOL)?
                    .get(VOL);
                let aggregate = risk + scenario_risk + scenario_vol + price / 10;
                let scenario_alert = u64::from(alert == 1 || aggregate > 2_700);
                return Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([
                        (RISK, aggregate),
                        (ALERT, scenario_alert),
                    ]))
                    .with_output_identity(format!("scenario-{scenario_index}-{aggregate}"))
                    .with_continuity_token("scenario-risk"),
                ));
            }
        }

        if let Some(book_index) = self.book_aggregates.iter().position(|candidate| *candidate == node) {
            let mut risk_total = view
                .read_aspect_version(self.aggregate_sources[book_index].book_state, RISK)?
                .get(RISK);
            let mut alert_total = view
                .read_aspect_version(self.aggregate_sources[book_index].book_state, ALERT)?
                .get(ALERT);
            let fx = view.read_aspect_version(self.fx.eur_jpy, PRICE)?.get(PRICE);
            for instrument in &self.instruments {
                if instrument.book_index == book_index {
                    risk_total += view.read_aspect_version(instrument.core.risk, RISK)?.get(RISK);
                    alert_total += view.read_aspect_version(instrument.core.alert, ALERT)?.get(ALERT);
                }
            }
            risk_total += fx / 100;
            let aggregate_alert = u64::from(alert_total > 0);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([
                    (RISK, risk_total),
                    (ALERT, aggregate_alert),
                ]))
                .with_output_identity(format!("book-{book_index}-{risk_total}-{aggregate_alert}"))
                .with_continuity_token("book-aggregate"),
            ));
        }

        if let Some(desk_index) = self.desk_aggregates.iter().position(|candidate| *candidate == node) {
            let mut risk_total = view
                .read_aspect_version(self.aggregate_sources[desk_index].desk_limit, RISK)?
                .get(RISK);
            let mut alert_total = view
                .read_aspect_version(self.aggregate_sources[desk_index].desk_limit, ALERT)?
                .get(ALERT);
            for (book_index, book_node) in self.book_aggregates.iter().enumerate() {
                if book_index % self.desk_aggregates.len() == desk_index {
                    risk_total += view.read_aspect_version(*book_node, RISK)?.get(RISK);
                    alert_total += view.read_aspect_version(*book_node, ALERT)?.get(ALERT);
                }
            }
            let aggregate_alert = u64::from(alert_total > 0 || risk_total > 25_000);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([
                    (RISK, risk_total),
                    (ALERT, aggregate_alert),
                ]))
                .with_output_identity(format!("desk-{desk_index}-{risk_total}-{aggregate_alert}"))
                .with_continuity_token("desk-aggregate"),
            ));
        }

        if let Some(scenario_index) = self
            .scenario_aggregates
            .iter()
            .position(|candidate| *candidate == node)
        {
            let mut total = 0_u64;
            for instrument in &self.instruments {
                total += view
                    .read_aspect_version(instrument.scenarios[scenario_index], RISK)?
                    .get(RISK);
            }
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(RISK, total)]))
                    .with_output_identity(format!("scenario-agg-{scenario_index}-{total}"))
                    .with_continuity_token("scenario-aggregate"),
            ));
        }

        if let Some(bucket_index) = self
            .bucket_aggregates
            .iter()
            .position(|candidate| *candidate == node)
        {
            let mut total = 0_u64;
            for instrument in &self.instruments {
                total += view
                    .read_aspect_version(instrument.buckets[bucket_index], RISK)?
                    .get(RISK);
            }
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(RISK, total)]))
                    .with_output_identity(format!("bucket-agg-{bucket_index}-{total}"))
                    .with_continuity_token("bucket-aggregate"),
            ));
        }

        if node == self.partition.rates_partition {
            let price = view
                .read_partitioned_aspect_version(
                    self.partition.market_regions,
                    PRICE,
                    PartitionSubscription::whole_partition("rates"),
                )?
                .get(PRICE);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, price)]))
                    .with_output_identity(format!("rates-partition-{price}"))
                    .with_continuity_token("rates-partition"),
            ));
        }

        if node == self.partition.credit_partition {
            let price = view
                .read_partitioned_aspect_version(
                    self.partition.market_regions,
                    PRICE,
                    PartitionSubscription::whole_partition("credit"),
                )?
                .get(PRICE);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, price)]))
                    .with_output_identity(format!("credit-partition-{price}"))
                    .with_continuity_token("credit-partition"),
            ));
        }

        if node == self.partition.rates_bucket_zero {
            let price = view
                .read_partitioned_aspect_version(
                    self.partition.market_regions,
                    PRICE,
                    PartitionSubscription::partition_and_detail("rates", "bucket-0"),
                )?
                .get(PRICE);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, price)]))
                    .with_output_identity(format!("rates-bucket-zero-{price}"))
                    .with_continuity_token("rates-bucket-zero"),
            ));
        }

        if node == self.partition.coarse_book {
            let rates = view
                .read_aspect_version(self.partition.rates_partition, PRICE)?
                .get(PRICE);
            let credit = view
                .read_aspect_version(self.partition.credit_partition, PRICE)?
                .get(PRICE);
            let total = rates + credit;
            return Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(PRICE, total)]))
                    .with_output_identity(format!("coarse-book-{total}"))
                    .with_continuity_token("coarse-book"),
            ));
        }

        Err(SignalError::invalid_input(format!(
            "unexpected fintech node {node}"
        )))
    }
}
