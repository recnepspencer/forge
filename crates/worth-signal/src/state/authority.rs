use serde::{Deserialize, Deserializer, Serialize};

use crate::data::graph::{DependencyEdgeStore, SubscriberEdgeStore};
use crate::data::node::{CheckpointNodeImage, NodeEntry};
use crate::data::proof::SnapshotBatchCommit;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::state::DiagnosticsState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheckpointSlot {
    #[serde(
        default,
        alias = "entry",
        deserialize_with = "deserialize_checkpoint_slot_node"
    )]
    pub node: Option<CheckpointNodeImage>,
    pub generation: u32,
    pub retired: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CheckpointSlotNodeRepr {
    Image(CheckpointNodeImage),
    Legacy(NodeEntry),
}

fn deserialize_checkpoint_slot_node<'de, D>(
    deserializer: D,
) -> Result<Option<CheckpointNodeImage>, D::Error>
where
    D: Deserializer<'de>,
{
    let repr = Option::<CheckpointSlotNodeRepr>::deserialize(deserializer)?;
    Ok(repr.map(|repr| match repr {
        CheckpointSlotNodeRepr::Image(image) => image,
        CheckpointSlotNodeRepr::Legacy(entry) => entry.to_checkpoint_image(),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheckpointArena {
    pub slots: Vec<SignalCheckpointSlot>,
    pub free_list: Vec<u32>,
    pub active_nodes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheckpointTopology {
    pub dependency_edges: DependencyEdgeStore,
    pub subscriber_edges: SubscriberEdgeStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Narrow checkpoint-owned authority payload used to reconstruct operational
/// graph truth without carrying runtime observation baggage.
pub struct SignalCheckpointAuthority {
    pub(crate) arena: SignalCheckpointArena,
    pub(crate) topology: SignalCheckpointTopology,
    pub(crate) diagnostics: DiagnosticsState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonical checkpoint-carried authority image for reconstructive restore.
///
/// Supported restore paths must consume this image rather than treating the
/// entire snapshot bundle as the authority carrier.
pub struct SignalCheckpointImage {
    pub authority: SignalCheckpointAuthority,
    pub dependency_snapshot_batch: SnapshotBatchCommit,
    pub graph_telemetry: RuntimeTelemetry,
}
