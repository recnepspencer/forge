use super::counters::WorthUiCompositionGraphAccessCounters;
use super::indexes::WorthUiCompositionGraphIndexes;
use super::request::WorthUiCompositionGraphAccessRequest;
use crate::runtime::composition_graph::WorthUiAdmittedCompositionGraphReceipt;

pub(super) fn counters_for_request(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    indexes: &WorthUiCompositionGraphIndexes,
    request: &WorthUiCompositionGraphAccessRequest,
) -> WorthUiCompositionGraphAccessCounters {
    let index_build_counts = CompositionAccessIndexBuildCounts::from_graph(graph);
    match request {
        WorthUiCompositionGraphAccessRequest::MountedProductTree => {
            let ancestor_row_count = graph
                .nodes()
                .iter()
                .map(|node| indexes.ancestors_of(node.node_id().as_str()).len())
                .sum::<usize>();
            index_build_counts.planned(
                5,
                graph.edges().len(),
                ancestor_row_count,
                indexes
                    .participating_descendants(graph.root().root_id().as_str())
                    .len(),
                graph.edges().len(),
            )
        }
        WorthUiCompositionGraphAccessRequest::RootChildren => index_build_counts.planned(
            2,
            indexes.children(graph.root().root_id().as_str()).len(),
            0,
            0,
            0,
        ),
        WorthUiCompositionGraphAccessRequest::OrderedChildren { parent_id } => {
            index_build_counts.planned(2, indexes.children(parent_id).len(), 0, 0, 0)
        }
        WorthUiCompositionGraphAccessRequest::ParentOf { .. } => {
            index_build_counts.planned(1, 0, 1, 0, 0)
        }
        WorthUiCompositionGraphAccessRequest::AncestorsOf { node_id } => {
            index_build_counts.planned(2, 0, indexes.ancestors_of(node_id.as_str()).len(), 0, 0)
        }
        WorthUiCompositionGraphAccessRequest::ParticipatingDescendants { parent_id } => {
            index_build_counts.planned(
                3,
                0,
                0,
                indexes.participating_descendants(parent_id).len(),
                0,
            )
        }
        WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedNode { .. }
        | WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedEdge { .. }
        | WorthUiCompositionGraphAccessRequest::AffectedConsumersForChangedPolicy { .. } => {
            index_build_counts.planned(2, 0, 0, 0, 1)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CompositionAccessIndexBuildCounts {
    node_count: usize,
    edge_count: usize,
    policy_count: usize,
}

impl CompositionAccessIndexBuildCounts {
    fn from_graph(graph: &WorthUiAdmittedCompositionGraphReceipt) -> Self {
        Self {
            node_count: graph.nodes().len(),
            edge_count: graph.edges().len(),
            policy_count: graph.policy_attachments().len(),
        }
    }

    fn planned(
        self,
        planned_index_family_count: usize,
        request_child_row_count: usize,
        request_ancestor_row_count: usize,
        request_participation_row_count: usize,
        request_affected_consumer_row_count: usize,
    ) -> WorthUiCompositionGraphAccessCounters {
        WorthUiCompositionGraphAccessCounters::planned(
            planned_index_family_count,
            self.node_count,
            self.edge_count,
            self.policy_count,
            request_child_row_count,
            request_ancestor_row_count,
            request_participation_row_count,
            request_affected_consumer_row_count,
        )
    }
}
