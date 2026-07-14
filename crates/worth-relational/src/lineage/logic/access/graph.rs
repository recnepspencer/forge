use crate::lineage::data::{
    LineageGraphDigestBasis, LineageGraphDigestMode, LineageGraphMetrics, LineageGraphRequest,
    LineageGraphSnapshot,
};
use crate::lineage::logic::access::LineageAccess;

impl<'runtime> LineageAccess<'runtime> {
    pub fn graph(&self, request: LineageGraphRequest) -> LineageGraphSnapshot {
        let mut nodes = self.branch_nodes_snapshot(&request.branch_id);
        nodes.sort_by_key(|node| node.lineage_id);
        let mut events = self.branch_events_snapshot(&request.branch_id);
        events.sort_by_key(|event| event.event_id);
        let mut correspondence_candidates = self
            .runtime
            .lineage
            .correspondence_candidates
            .iter()
            .filter(|candidate| candidate.branch_id == request.branch_id)
            .cloned()
            .collect::<Vec<_>>();
        correspondence_candidates.sort_by_key(|candidate| candidate.candidate_id);
        self.runtime
            .performance_access()
            .count_lineage_graph_snapshot_request(
                nodes.len(),
                events.len(),
                correspondence_candidates.len(),
            );
        let node_count = nodes.len();
        let event_count = events.len();
        let candidate_count = correspondence_candidates.len();
        let digest_basis = LineageGraphDigestBasis::new(
            request.branch_id.clone(),
            request.traversal_basis,
            nodes.iter().map(|node| node.lineage_id).collect(),
            events.iter().map(|event| event.event_id).collect(),
            correspondence_candidates
                .iter()
                .map(|candidate| candidate.candidate_id)
                .collect(),
            LineageGraphDigestMode::ExactDigestCanonicalOrder,
        );
        LineageGraphSnapshot::new(
            request.branch_id.clone(),
            nodes,
            events,
            correspondence_candidates,
            request.traversal_basis,
            digest_basis,
            LineageGraphMetrics {
                node_count,
                event_count,
                candidate_count,
            },
        )
    }
}
