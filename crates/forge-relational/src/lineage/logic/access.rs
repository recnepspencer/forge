use std::collections::BTreeSet;

use crate::history::data::BranchId;
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    CorrespondenceCandidate, HistoricalLineageResolution, LineageDivergenceSummary,
    LineageEventKind, LineageEventRecord, LineageGraphSnapshot, LineageNode,
};
use crate::logic::runtime::RelationalRuntime;

pub struct LineageAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn lineage_access(&self) -> LineageAccess<'_> {
        LineageAccess::new(self)
    }
}

impl<'runtime> LineageAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn graph(&self, branch_id: &BranchId) -> LineageGraphSnapshot {
        LineageGraphSnapshot {
            branch_id: branch_id.clone(),
            nodes: self.nodes_snapshot(),
            events: self
                .runtime
                .lineage
                .events
                .iter()
                .filter(|event| &event.branch_id == branch_id)
                .cloned()
                .collect(),
            correspondence_candidates: self
                .runtime
                .lineage
                .correspondence_candidates
                .iter()
                .filter(|candidate| &candidate.branch_id == branch_id)
                .cloned()
                .collect(),
        }
    }

    pub fn divergence_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> LineageDivergenceSummary {
        let left_graph = self.graph(left_branch);
        let right_graph = self.graph(right_branch);
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
            left_branch: left_branch.clone(),
            right_branch: right_branch.clone(),
            left_only_event_ids: left_event_ids
                .difference(&right_event_ids)
                .copied()
                .collect(),
            right_only_event_ids: right_event_ids
                .difference(&left_event_ids)
                .copied()
                .collect(),
            shared_lineage_ids,
        }
    }

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

    pub fn resolve_historical_lineage(
        &self,
        branch_id: &BranchId,
        lineage_id: LineageId,
    ) -> HistoricalLineageResolution {
        let mut current = BTreeSet::from([lineage_id]);
        let mut traversed_event_ids = Vec::new();

        for event in self
            .runtime
            .lineage
            .events
            .iter()
            .filter(|event| &event.branch_id == branch_id)
        {
            if !event.sources.iter().any(|source| current.contains(source)) {
                continue;
            }
            match event.kind {
                LineageEventKind::Replace
                | LineageEventKind::Split
                | LineageEventKind::Merge
                | LineageEventKind::Correspond => {
                    traversed_event_ids.push(event.event_id);
                    for source in &event.sources {
                        current.remove(source);
                    }
                    current.extend(event.targets.iter().copied());
                }
                LineageEventKind::Create | LineageEventKind::Retire => {}
            }
        }

        HistoricalLineageResolution {
            branch_id: branch_id.clone(),
            start: lineage_id,
            resolved: current.into_iter().collect(),
            traversed_event_ids,
        }
    }

    pub fn resolve_record_history(
        &self,
        branch_id: &BranchId,
        entity_id: EntityId,
    ) -> Option<HistoricalLineageResolution> {
        let lineage = self.for_record(entity_id)?;
        Some(self.resolve_historical_lineage(branch_id, lineage.lineage_id))
    }

    pub(crate) fn nodes_snapshot(&self) -> Vec<LineageNode> {
        self.runtime.lineage.nodes.values().cloned().collect()
    }

    pub(crate) fn events_snapshot(&self) -> Vec<LineageEventRecord> {
        self.runtime.lineage.events.clone()
    }

    pub(crate) fn correspondence_candidates_snapshot(&self) -> Vec<CorrespondenceCandidate> {
        self.runtime.lineage.correspondence_candidates.clone()
    }
}
