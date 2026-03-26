use std::collections::BTreeSet;

use crate::history::data::BranchId;
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    CorrespondenceCandidate, LineageDecisionRecord, LineageEventRecord, LineageNode,
};
use crate::lineage::logic::access::LineageAccess;
use crate::visibility::cache_state::cached_state_for_version;

impl<'runtime> LineageAccess<'runtime> {
    pub fn for_record(&self, entity_id: EntityId) -> Option<&LineageNode> {
        let lineage_id = self
            .runtime
            .partitions
            .get(&entity_id.partition_id)?
            .entity_arena
            .get(&entity_id)
            .and_then(|slot_view| slot_view.extra().lineage_id)?;
        self.runtime.lineage.nodes.get(&lineage_id)
    }

    pub(crate) fn nodes_snapshot(&self) -> Vec<LineageNode> {
        self.runtime.lineage.nodes.values().cloned().collect()
    }

    pub(crate) fn branch_events_snapshot(
        &self,
        branch_id: &crate::history::data::BranchId,
    ) -> Vec<LineageEventRecord> {
        self.runtime
            .lineage
            .branch_events(branch_id)
            .cloned()
            .collect()
    }

    pub(crate) fn branch_nodes_snapshot(&self, branch_id: &BranchId) -> Vec<LineageNode> {
        let history = self.runtime.history_access();
        let Some(head) = history.branch_head(branch_id) else {
            return Vec::new();
        };
        let cache_hit = cached_state_for_version(self.runtime, head.version_id).is_some();
        let read_view = self
            .runtime
            .visibility_reads()
            .read_version(head.version_id);
        self.runtime
            .performance_access()
            .count_lineage_graph_snapshot_visibility_cache(cache_hit);
        let mut seen = BTreeSet::<LineageId>::new();
        let mut nodes = Vec::new();

        for record in read_view.entities() {
            let Some(lineage_id) = record.lineage_id else {
                continue;
            };
            let Some(node) = self.runtime.lineage.nodes.get(&lineage_id) else {
                continue;
            };
            if seen.insert(node.lineage_id) {
                nodes.push(node.clone());
            }
        }

        nodes
    }

    pub(crate) fn correspondence_candidates_snapshot(&self) -> Vec<CorrespondenceCandidate> {
        self.runtime.lineage.correspondence_candidates.clone()
    }

    pub(crate) fn rejected_decisions_snapshot(&self) -> Vec<LineageDecisionRecord> {
        self.runtime.lineage.rejected_decisions.clone()
    }
}
