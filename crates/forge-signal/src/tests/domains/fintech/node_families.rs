use crate::facade::{EvaluationCondition, NodeId, SignalRuntime};

use super::aspects::{full_mask, market_mask, pricing_mask, ALERT};

pub(super) type FintechRuntime = SignalRuntime<(), (), (), (), ()>;

#[derive(Clone, Copy, Debug)]
pub(super) struct InstrumentNodes {
    pub market: NodeId,
    pub normalized: NodeId,
    pub price: NodeId,
    pub risk: NodeId,
    pub alert: NodeId,
    pub threshold: NodeId,
}

pub(super) fn build_instrument_nodes(runtime: &mut FintechRuntime) -> InstrumentNodes {
    let market = runtime.graph_mut().node().depends_on_aspects(full_mask()).build();
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

pub(super) fn build_scenario_nodes(
    runtime: &mut FintechRuntime,
    instrument: &InstrumentNodes,
    scenarios: usize,
) -> Vec<NodeId> {
    let mut nodes = Vec::with_capacity(scenarios);
    for _ in 0..scenarios {
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
        nodes.push(node);
    }
    nodes
}
