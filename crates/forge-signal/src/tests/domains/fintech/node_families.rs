use crate::facade::{EvaluationCondition, NodeId, SignalRuntime};

use super::aspects::{full_mask, market_mask, pricing_mask, ALERT};
use super::execution_tier::FintechTier;

pub(super) type FintechRuntime = SignalRuntime<(), (), (), (), FintechTier>;

#[derive(Clone, Copy, Debug)]
pub(super) struct FxNodes {
    pub eur_usd: NodeId,
    pub usd_jpy: NodeId,
    pub eur_jpy: NodeId,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct AggregateSourceNodes {
    pub book_state: NodeId,
    pub desk_limit: NodeId,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct InstrumentNodes {
    pub market: NodeId,
    pub normalized: NodeId,
    pub price: NodeId,
    pub risk: NodeId,
    pub alert: NodeId,
    pub threshold: NodeId,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PartitionLocalityNodes {
    pub market_regions: NodeId,
    pub rates_partition: NodeId,
    pub credit_partition: NodeId,
    pub rates_bucket_zero: NodeId,
    pub coarse_book: NodeId,
}

pub(super) fn build_instrument_nodes(runtime: &mut FintechRuntime) -> InstrumentNodes {
    let market = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(full_mask())
        .build();
    let normalized = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(market_mask())
        .tolerance(1)
        .build();
    let price = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(2)
        .build();
    let risk = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(3)
        .build();
    let alert = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(full_mask())
        .aspect_filter(ALERT)
        .tolerance(1)
        .build();
    let threshold = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .tolerance(2)
        .build();

    runtime
        .graph_mut()
        .add_dependency(normalized, market, super::aspects::PRICE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(normalized, market, super::aspects::VOL)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(normalized, market, super::aspects::CURVE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(normalized, market, super::aspects::LIQUIDITY)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(price, normalized, super::aspects::PRICE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(price, normalized, super::aspects::VOL)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(price, normalized, super::aspects::CURVE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(risk, price, super::aspects::RISK)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(risk, normalized, super::aspects::LIQUIDITY)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(alert, risk, super::aspects::ALERT)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(threshold, price, super::aspects::PRICE)
        .unwrap();

    InstrumentNodes {
        market,
        normalized,
        price,
        risk,
        alert,
        threshold,
    }
}

pub(super) fn build_bucket_exposure_nodes(
    runtime: &mut FintechRuntime,
    instrument: &InstrumentNodes,
    buckets: usize,
) -> Vec<NodeId> {
    let mut nodes = Vec::with_capacity(buckets);
    for _ in 0..buckets {
        let node = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(pricing_mask())
            .tolerance(3)
            .build();
        runtime
            .graph_mut()
            .add_dependency(node, instrument.risk, super::aspects::RISK)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(node, instrument.threshold, super::aspects::PRICE)
            .unwrap();
        nodes.push(node);
    }
    nodes
}

pub(super) fn build_aggregate_sources(runtime: &mut FintechRuntime) -> AggregateSourceNodes {
    let book_state = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(super::aspects::full_mask())
        .tolerance(2)
        .build();
    let desk_limit = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(super::aspects::full_mask())
        .tolerance(2)
        .build();

    AggregateSourceNodes {
        book_state,
        desk_limit,
    }
}

pub(super) fn build_scenario_nodes(
    runtime: &mut FintechRuntime,
    instrument: &InstrumentNodes,
    scenario_sources: &[NodeId],
    scenarios: usize,
) -> Vec<NodeId> {
    let mut nodes = Vec::with_capacity(scenarios);
    for scenario_source in scenario_sources.iter().copied().take(scenarios) {
        let node = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(full_mask())
            .tolerance(4)
            .build();
        runtime
            .graph_mut()
            .add_dependency(node, instrument.price, super::aspects::PRICE)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(node, instrument.risk, super::aspects::RISK)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(node, instrument.alert, super::aspects::ALERT)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(node, scenario_source, super::aspects::RISK)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(node, scenario_source, super::aspects::VOL)
            .unwrap();
        nodes.push(node);
    }
    nodes
}

pub(super) fn build_fx_nodes(runtime: &mut FintechRuntime) -> FxNodes {
    let eur_usd = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(super::aspects::full_mask())
        .tolerance(1)
        .build();
    let usd_jpy = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(super::aspects::full_mask())
        .tolerance(1)
        .build();
    let eur_jpy = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(super::aspects::full_mask())
        .tolerance(2)
        .build();
    runtime
        .graph_mut()
        .add_dependency(eur_jpy, eur_usd, super::aspects::PRICE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(eur_jpy, usd_jpy, super::aspects::PRICE)
        .unwrap();
    FxNodes {
        eur_usd,
        usd_jpy,
        eur_jpy,
    }
}

pub(super) fn build_partition_locality_nodes(
    runtime: &mut FintechRuntime,
) -> PartitionLocalityNodes {
    let market_regions = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .partitioned_output()
        .build();
    let rates_partition = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(1)
        .build();
    let credit_partition = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(1)
        .build();
    let rates_bucket_zero = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(1)
        .build();
    let coarse_book = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(pricing_mask())
        .tolerance(2)
        .build();

    runtime
        .graph_mut()
        .add_partition_dependency(
            rates_partition,
            market_regions,
            super::aspects::PRICE,
            "rates",
        )
        .unwrap();
    runtime
        .graph_mut()
        .add_partition_dependency(
            credit_partition,
            market_regions,
            super::aspects::PRICE,
            "credit",
        )
        .unwrap();
    runtime
        .graph_mut()
        .add_partition_detail_dependency(
            rates_bucket_zero,
            market_regions,
            super::aspects::PRICE,
            "rates",
            "bucket-0",
        )
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(coarse_book, rates_partition, super::aspects::PRICE)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(coarse_book, credit_partition, super::aspects::PRICE)
        .unwrap();

    PartitionLocalityNodes {
        market_regions,
        rates_partition,
        credit_partition,
        rates_bucket_zero,
        coarse_book,
    }
}

pub(super) fn build_bucket_sources(runtime: &mut FintechRuntime, buckets: usize) -> Vec<NodeId> {
    let mut nodes = Vec::with_capacity(buckets);
    for _ in 0..buckets {
        nodes.push(
            runtime
                .graph_mut()
                .node()
                .depends_on_aspects(super::aspects::full_mask())
                .tolerance(1)
                .build(),
        );
    }
    nodes
}

pub(super) fn build_scenario_sources(
    runtime: &mut FintechRuntime,
    scenarios: usize,
) -> Vec<NodeId> {
    let mut nodes = Vec::with_capacity(scenarios);
    for _ in 0..scenarios {
        nodes.push(
            runtime
                .graph_mut()
                .node()
                .depends_on_aspects(super::aspects::full_mask())
                .tolerance(2)
                .build(),
        );
    }
    nodes
}
