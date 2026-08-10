use crate::facade::{AspectVersion, NodeId, SignalError, StageExecutor};
use crate::tests::support::DependencyBatchBuilder;

use super::super::node_families::FintechRuntime;
use super::super::scales::FintechScale;
use super::portfolio_world::PortfolioWorld;

pub(super) struct RiskWorld {
    pub(super) scenario_aggregates: Vec<NodeId>,
    pub(super) bucket_aggregates: Vec<NodeId>,
}

pub(in crate::tests::domains::fintech) fn build_risk_world(
    runtime: &mut FintechRuntime,
    scale: FintechScale,
    portfolio: &PortfolioWorld,
) -> RiskWorld {
    let instruments = &portfolio.instruments;
    let mut scenario_aggregates = Vec::with_capacity(scale.scenarios);
    for scenario_index in 0..scale.scenarios {
        let aggregate = runtime
            .graph_mut()
            .node()
            .reads_aspects(super::super::aspects::full_mask())
            .tolerance(5)
            .build();
        let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
        for instrument in instruments {
            dependencies
                .append_dependency(
                    aggregate,
                    instrument.scenarios[scenario_index],
                    super::super::aspects::RISK,
                )
                .unwrap();
        }
        dependencies.commit().unwrap();
        scenario_aggregates.push(aggregate);
    }

    let mut bucket_aggregates = Vec::with_capacity(scale.buckets);
    for bucket_index in 0..scale.buckets {
        let aggregate = runtime
            .graph_mut()
            .node()
            .reads_aspects(super::super::aspects::full_mask())
            .tolerance(5)
            .build();
        let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
        for instrument in instruments {
            dependencies
                .append_dependency(
                    aggregate,
                    instrument.buckets[bucket_index],
                    super::super::aspects::RISK,
                )
                .unwrap();
        }
        dependencies.commit().unwrap();
        bucket_aggregates.push(aggregate);
    }
    RiskWorld {
        scenario_aggregates,
        bucket_aggregates,
    }
}

impl super::FintechWorld {
    pub(in crate::tests::domains::fintech) fn main_risk_node(&self) -> NodeId {
        self.handles.primary.risk
    }

    pub(in crate::tests::domains::fintech) fn primary_threshold_node(&self) -> NodeId {
        self.handles.primary.threshold
    }

    pub(in crate::tests::domains::fintech) fn read_primary_threshold_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError> {
        self.read_node_with_executor(self.primary_threshold_node(), executor)
    }

    pub(in crate::tests::domains::fintech) fn read_bucket_aggregate_with_executor(
        &mut self,
        index: usize,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError> {
        self.read_node_with_executor(self.bucket_aggregates[index], executor)
    }

    pub(in crate::tests::domains::fintech) fn read_scenario_aggregate_with_executor(
        &mut self,
        index: usize,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError> {
        self.read_node_with_executor(self.scenario_aggregates[index], executor)
    }
}
