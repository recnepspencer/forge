use crate::lineage::data::{LineageGraphRequest, LineageGraphSnapshot};
use crate::lineage::logic::access::LineageAccess;

impl<'runtime> LineageAccess<'runtime> {
    pub fn graph(&self, request: LineageGraphRequest) -> LineageGraphSnapshot {
        let nodes = self.branch_nodes_snapshot(&request.branch_id);
        self.runtime
            .performance_access()
            .count_lineage_graph_snapshot_request(nodes.len());
        LineageGraphSnapshot {
            branch_id: request.branch_id.clone(),
            nodes,
            events: self.branch_events_snapshot(&request.branch_id),
            correspondence_candidates: self
                .runtime
                .lineage
                .correspondence_candidates
                .iter()
                .filter(|candidate| candidate.branch_id == request.branch_id)
                .cloned()
                .collect(),
        }
    }
}
