use crate::facade::NodeId;

use super::partition_surface::PartitionSurfaceNodes;

#[derive(Debug, Clone, Copy)]
pub(super) struct PrimaryNodes {
    pub market_source: NodeId,
    pub threshold: NodeId,
    pub risk: NodeId,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct AggregateNodes {
    pub top_desk: NodeId,
    pub top_scenario: NodeId,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FintechWorldHandles {
    pub primary: PrimaryNodes,
    pub aggregate: AggregateNodes,
    pub partition: PartitionSurfaceNodes,
}

impl FintechWorldHandles {
    pub(super) fn new(
        market_source: NodeId,
        threshold: NodeId,
        risk: NodeId,
        top_desk: NodeId,
        top_scenario: NodeId,
        partition: PartitionSurfaceNodes,
    ) -> Self {
        Self {
            primary: PrimaryNodes {
                market_source,
                threshold,
                risk,
            },
            aggregate: AggregateNodes {
                top_desk,
                top_scenario,
            },
            partition,
        }
    }
}
