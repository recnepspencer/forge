use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent, ReplaySlice};
use crate::state::SignalBranchHandle;

impl SignalGraph {
    pub fn replay_events(&self) -> &std::collections::VecDeque<ReplayEvent> {
        self.observation.diagnostics.replay_events()
    }

    pub fn replay_where(
        &self,
        mut predicate: impl FnMut(&ReplayEvent) -> bool,
    ) -> ReplaySlice {
        ReplaySlice {
            start: None,
            end: None,
            frames: self
                .replay_events()
                .iter()
                .filter(|frame| predicate(frame))
                .cloned()
                .collect(),
        }
    }

    pub fn replay_slice(
        &self,
        start: Option<ReplayCursor>,
        end: Option<ReplayCursor>,
    ) -> ReplaySlice {
        let mut slice = self.replay_where(|frame| {
            start.is_none_or(|cursor| frame.cursor >= cursor)
                && end.is_none_or(|cursor| frame.cursor <= cursor)
        });
        slice.start = start;
        slice.end = end;
        slice
    }

    pub fn replay_for_branch(&self, branch_id: crate::state::SignalBranchId) -> ReplaySlice {
        self.replay_where(|frame| frame.branch_id == branch_id)
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplaySlice {
        self.replay_where(|frame| frame.node == Some(node))
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplaySlice {
        self.replay_where(|frame| frame.lineage_artifact_id == Some(artifact_id))
    }

    pub fn replay_from_cursor(&self, start: ReplayCursor) -> ReplaySlice {
        self.replay_slice(Some(start), None)
    }

    pub fn replay_between(&self, start: ReplayCursor, end: ReplayCursor) -> ReplaySlice {
        self.replay_slice(Some(start), Some(end))
    }

    pub fn replay_around_snapshot(
        &self,
        snapshot_id: crate::state::SignalSnapshotId,
    ) -> ReplaySlice {
        let Some(index) = self
            .replay_events()
            .iter()
            .position(|event| event.snapshot_id == Some(snapshot_id))
        else {
            return ReplaySlice::default();
        };
        let start = index.saturating_sub(4);
        let end = (index + 5).min(self.replay_events().len());
        let cursors = self
            .replay_events()
            .iter()
            .skip(start)
            .take(end.saturating_sub(start))
            .map(|event| event.cursor)
            .collect::<std::collections::BTreeSet<_>>();
        let mut slice = self.replay_where(|event| cursors.contains(&event.cursor));
        slice.start = self.replay_events().get(start).map(|event| event.cursor);
        slice.end = self
            .replay_events()
            .get(end.saturating_sub(1))
            .map(|event| event.cursor);
        slice
    }

    pub fn lineage_records(&self) -> &std::collections::VecDeque<LineageRecord> {
        self.observation.diagnostics.lineage_records()
    }

    pub fn lineage_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.node == Some(node))
            .cloned()
            .collect()
    }

    pub fn lineage_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.artifact_id == Some(artifact_id))
            .cloned()
            .collect()
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.get_entry(node)
            .ok()
            .and_then(|entry| entry.get_trace_summary())
            .and_then(|summary| summary.lineage_artifact_id)
    }

    pub fn lineage_chain_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        let mut chain = Vec::new();
        let mut current = Some(artifact_id);
        let mut visited = std::collections::BTreeSet::new();
        while let Some(artifact_id) = current {
            if !visited.insert(artifact_id) {
                break;
            }
            let mut artifact_records = self
                .lineage_records()
                .iter()
                .filter(|record| record.artifact_id == Some(artifact_id))
                .cloned()
                .collect::<Vec<_>>();
            if artifact_records.is_empty() {
                break;
            }
            artifact_records.sort_by_key(|record| record.sequence);
            current = artifact_records.iter().find_map(|record| {
                record
                    .parent_artifact_id
                    .filter(|parent| *parent != artifact_id)
            });
            chain.extend(artifact_records);
        }
        chain.sort_by_key(|record| record.sequence);
        chain
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
        self.current_lineage_artifact(node)
            .map(|artifact_id| self.lineage_chain_for_artifact(artifact_id))
            .unwrap_or_default()
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.observation.diagnostics.active_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.observation
            .diagnostics
            .branch_catalog()
            .values()
            .cloned()
            .collect()
    }

    pub fn branch_handle(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<SignalBranchHandle> {
        self.observation
            .diagnostics
            .branch_catalog()
            .get(&branch_id)
            .cloned()
    }

    pub fn branch_head_snapshot_id(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<crate::state::SignalSnapshotId> {
        self.branch_handle(branch_id)
            .and_then(|branch| branch.head_snapshot_id)
    }

    pub fn branch_ancestry(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Vec<SignalBranchHandle> {
        let mut lineage = Vec::new();
        let mut current = self.branch_handle(branch_id);
        while let Some(branch) = current {
            current = branch
                .parent_branch_id
                .and_then(|parent_id| self.branch_handle(parent_id));
            lineage.push(branch);
        }
        lineage.reverse();
        lineage
    }
}
