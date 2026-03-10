use crate::facade::{NodeId, SignalGraph, SignalRuntime, SignalRuntimePolicy};

use super::node_families::{
    build_bucket_exposure_nodes, build_instrument_nodes, build_scenario_nodes, FintechRuntime,
    InstrumentNodes,
};
use super::scales::FintechScale;

#[derive(Debug)]
pub(super) struct InstrumentFixture {
    pub core: InstrumentNodes,
    pub buckets: Vec<NodeId>,
    pub scenarios: Vec<NodeId>,
}

pub(super) struct FintechDomainFixture {
    pub runtime: FintechRuntime,
    pub instruments: Vec<InstrumentFixture>,
    pub scenario_aggregates: Vec<NodeId>,
    pub bucket_aggregates: Vec<NodeId>,
}

impl FintechDomainFixture {
    pub(super) fn live_node_count(&self) -> usize {
        self.runtime.graph().live_node_ids().len()
    }
}

pub(super) fn build_fixture(scale: FintechScale) -> FintechDomainFixture {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::fintech()
            .with_history_limit(8)
            .with_detail_limit(4),
    );

    let mut instruments = Vec::with_capacity(scale.instruments);
    for _ in 0..scale.instruments {
        let core = build_instrument_nodes(&mut runtime);
        let buckets = build_bucket_exposure_nodes(&mut runtime, &core, scale.buckets);
        let scenarios = build_scenario_nodes(&mut runtime, &core, scale.scenarios);
        instruments.push(InstrumentFixture {
            core,
            buckets,
            scenarios,
        });
    }

    let mut scenario_aggregates = Vec::with_capacity(scale.scenarios);
    for scenario_index in 0..scale.scenarios {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(5)
            .build();
        for instrument in &instruments {
            runtime
                .graph_mut()
                .add_dependency(aggregate, instrument.scenarios[scenario_index], super::aspects::RISK)
                .unwrap();
        }
        scenario_aggregates.push(aggregate);
    }

    let mut bucket_aggregates = Vec::with_capacity(scale.buckets);
    for bucket_index in 0..scale.buckets {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(5)
            .build();
        for instrument in &instruments {
            runtime
                .graph_mut()
                .add_dependency(aggregate, instrument.buckets[bucket_index], super::aspects::RISK)
                .unwrap();
        }
        bucket_aggregates.push(aggregate);
    }

    FintechDomainFixture {
        runtime,
        instruments,
        scenario_aggregates,
        bucket_aggregates,
    }
}
