use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent, ReplaySlice};
use crate::state::SignalBranchHandle;

impl SignalGraph {
    pub fn replay_events(&self) -> &std::collections::VecDeque<ReplayEvent> {
        self.diagnostics.replay_events()
    }

    pub fn replay_slice(
        &self,
        start: Option<ReplayCursor>,
        end: Option<ReplayCursor>,
    ) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| start.is_none_or(|cursor| frame.cursor >= cursor))
            .filter(|frame| end.is_none_or(|cursor| frame.cursor <= cursor))
            .cloned()
            .collect();
        ReplaySlice { start, end, frames }
    }

    pub fn replay_for_branch(&self, branch_id: crate::state::SignalBranchId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.branch_id == branch_id)
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.node == Some(node))
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.lineage_artifact_id == Some(artifact_id))
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
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
        ReplaySlice {
            start: self.replay_events().get(start).map(|event| event.cursor),
            end: self
                .replay_events()
                .get(end.saturating_sub(1))
                .map(|event| event.cursor),
            frames: self
                .replay_events()
                .iter()
                .skip(start)
                .take(end.saturating_sub(start))
                .cloned()
                .collect(),
        }
    }

    pub fn lineage_records(&self) -> &std::collections::VecDeque<LineageRecord> {
        self.diagnostics.lineage_records()
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
        self.diagnostics.active_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.diagnostics.branch_catalog().values().cloned().collect()
    }

    pub fn branch_handle(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<SignalBranchHandle> {
        self.diagnostics.branch_catalog().get(&branch_id).cloned()
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
