use std::collections::BTreeSet;

use crate::lineage::data::{
    LineageDivergenceMetrics, LineageDivergenceRequest, LineageDivergenceSummary,
    LineageGraphRequest,
};
use crate::lineage::logic::access::LineageAccess;

impl<'runtime> LineageAccess<'runtime> {
    pub fn divergence_between_branches(
        &self,
        request: LineageDivergenceRequest,
    ) -> LineageDivergenceSummary {
        let left_graph = self.graph(LineageGraphRequest {
            branch_id: request.left_branch.clone(),
        });
        let right_graph = self.graph(LineageGraphRequest {
            branch_id: request.right_branch.clone(),
        });
        self.runtime.performance_access().count_lineage_branch_divergence(
            left_graph.events.len(),
            right_graph.events.len(),
            left_graph.nodes.len(),
            right_graph.nodes.len(),
        );
        let left_event_ids = left_graph
            .events
            .iter()
            .map(|event| event.event_id)
            .collect::<BTreeSet<_>>();
        let right_event_ids = right_graph
            .events
            .iter()
            .map(|event| event.event_id)
            .collect::<BTreeSet<_>>();
        let shared_lineage_ids = left_graph
            .nodes
            .iter()
            .map(|node| node.lineage_id)
            .collect::<BTreeSet<_>>()
            .intersection(
                &right_graph
                    .nodes
                    .iter()
                    .map(|node| node.lineage_id)
                    .collect::<BTreeSet<_>>(),
            )
            .copied()
            .collect::<Vec<_>>();
        LineageDivergenceSummary {
            left_branch: request.left_branch,
            right_branch: request.right_branch,
            left_only_event_ids: left_event_ids
                .difference(&right_event_ids)
                .copied()
                .collect(),
            right_only_event_ids: right_event_ids
                .difference(&left_event_ids)
                .copied()
                .collect(),
            shared_lineage_ids,
            metrics: LineageDivergenceMetrics {
                left_event_count: left_graph.events.len(),
                right_event_count: right_graph.events.len(),
                left_node_count: left_graph.nodes.len(),
                right_node_count: right_graph.nodes.len(),
                shared_lineage_count: left_graph
                    .nodes
                    .iter()
                    .map(|node| node.lineage_id)
                    .collect::<BTreeSet<_>>()
                    .intersection(
                        &right_graph
                            .nodes
                            .iter()
                            .map(|node| node.lineage_id)
                            .collect::<BTreeSet<_>>(),
                    )
                    .count(),
            },
        }
    }
}
