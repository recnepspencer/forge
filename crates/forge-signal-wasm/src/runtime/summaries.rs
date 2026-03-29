use serde::{Deserialize, Serialize};

use forge_signal::diagnostics::ReplayEvent;
use forge_signal::facade::diagnostics::GraphSummary;
use forge_signal::facade::history::{LineageEvent as NativeLineageEvent, ReplayView, RuntimeSnapshot};

use crate::expression::model::SignalValue;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunSummary {
    pub touched_nodes: u32,
    pub nodes_evaluated: u32,
    pub nodes_recomputed: u32,
    pub nodes_suppressed: u32,
    pub plans_built: u32,
    pub stages_executed: u32,
    pub total_nanos: String,
    pub evaluation_nanos: String,
    pub commit_nanos: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WhySummary {
    pub id: String,
    pub node: String,
    pub state: String,
    pub upstream: Vec<String>,
    pub changed_regions: Vec<String>,
    pub propagation_suppressed: bool,
    pub output_change: Option<String>,
    pub output_identity: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthSummary {
    pub active_node_count: u32,
    pub clean_node_count: u32,
    pub maybe_stale_node_count: u32,
    pub dirty_node_count: u32,
    pub dependency_edge_count: u32,
    pub subscriber_edge_count: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayFrameSummary {
    pub cursor: u64,
    pub kind: String,
    pub branch_id: u64,
    pub snapshot_id: Option<u64>,
    pub node: Option<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySummary {
    pub frames: Vec<ReplayFrameSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageSummary {
    pub events: Vec<LineageEventSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEventSummary {
    pub sequence: u64,
    pub label: String,
    pub emitted_on_branch_id: u64,
    pub node: Option<String>,
    pub subject_artifact_id: Option<u64>,
    pub snapshot_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSnapshotEnvelope {
    pub snapshot: RuntimeSnapshot,
    pub state: RuntimeStoreSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStoreSnapshot {
    pub sources: Vec<StoredSourceSnapshot>,
    pub recipes: Vec<StoredRecipeSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredSourceSnapshot {
    pub id: String,
    pub value: SignalValue,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredRecipeSnapshot {
    pub id: String,
    pub value: SignalValue,
    pub version: u64,
    pub initialized: bool,
    pub output_identity: Option<String>,
}

impl From<GraphSummary> for HealthSummary {
    fn from(value: GraphSummary) -> Self {
        Self {
            active_node_count: value.active_node_count,
            clean_node_count: value.clean_node_count,
            maybe_stale_node_count: value.maybe_stale_node_count,
            dirty_node_count: value.dirty_node_count,
            dependency_edge_count: value.dependency_edge_count,
            subscriber_edge_count: value.subscriber_edge_count,
        }
    }
}

impl From<ReplayView> for ReplaySummary {
    fn from(value: ReplayView) -> Self {
        Self {
            frames: value
                .frames
                .into_iter()
                .map(ReplayFrameSummary::from)
                .collect(),
        }
    }
}

impl From<ReplayEvent> for ReplayFrameSummary {
    fn from(value: ReplayEvent) -> Self {
        Self {
            cursor: value.cursor.0,
            kind: format!("{:?}", value.kind),
            branch_id: value.branch_id.0,
            snapshot_id: value.snapshot_id.map(|id| id.0),
            node: value.node.map(|node| node.to_string()),
            detail: value
                .detail
                .and_then(|detail| detail.as_message().map(str::to_owned))
                .or_else(|| value.execution_record_id.map(|id| format!("executionRecord:{id}"))),
        }
    }
}

impl From<Vec<NativeLineageEvent>> for LineageSummary {
    fn from(value: Vec<NativeLineageEvent>) -> Self {
        Self {
            events: value
                .into_iter()
                .map(|record| LineageEventSummary {
                    sequence: record.sequence,
                    label: record.label().to_owned(),
                    emitted_on_branch_id: record.emitted_on_branch_id().0,
                    node: record.node().map(|node| node.to_string()),
                    subject_artifact_id: record.subject_artifact_id().map(|id| id.0),
                    snapshot_id: record.snapshot_id().map(|id| id.0),
                })
                .collect(),
        }
    }
}
