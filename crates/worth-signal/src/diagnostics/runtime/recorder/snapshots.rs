use crate::data::graph::SignalGraph;
use crate::diagnostics::lineage::{LineageRecord, SnapshotRestoreKind};
use crate::diagnostics::policy::SnapshotRestoreLineageMode;
use crate::diagnostics::replay::ReplayEventKind;
use crate::state::SignalSnapshotId;

use super::events;

pub(crate) fn record_snapshot_restore_lineage(
    graph: &mut SignalGraph,
    snapshot_id: SignalSnapshotId,
) {
    match graph.runtime_policy().snapshot_restore_lineage_mode {
        SnapshotRestoreLineageMode::CompactGlobal => {
            let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
            let emitted_on_branch_id = graph.observe().current_branch().id;
            graph
                .diagnostics_state_mut()
                .record_lineage_record(LineageRecord::snapshot_restore(
                    sequence,
                    emitted_on_branch_id,
                    snapshot_id,
                    None,
                    None,
                    SnapshotRestoreKind::CompactGlobal,
                ));
        }
        SnapshotRestoreLineageMode::PerNode => {
            let emitted_on_branch_id = graph.observe().current_branch().id;
            let restored_nodes = graph
                .live_node_ids()
                .into_iter()
                .filter_map(|node| {
                    graph
                        .node_lineage_artifact_id(node)
                        .ok()
                        .flatten()
                        .map(|artifact_id| (node, artifact_id))
                })
                .collect::<Vec<_>>();
            for (node, artifact_id) in restored_nodes {
                let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
                graph.diagnostics_state_mut().record_lineage_record(
                    LineageRecord::snapshot_restore(
                        sequence,
                        emitted_on_branch_id,
                        snapshot_id,
                        Some(node),
                        Some(artifact_id),
                        SnapshotRestoreKind::PerNodeArtifact,
                    ),
                );
            }
        }
    }
    events::record_snapshot_event(
        graph,
        ReplayEventKind::SnapshotRestored,
        Some(snapshot_id),
        format!("restored snapshot {}", snapshot_id.0),
    );
}
