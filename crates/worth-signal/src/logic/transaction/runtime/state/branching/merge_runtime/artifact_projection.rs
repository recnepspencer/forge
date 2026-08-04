use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::trace::{ArtifactMergeAuthority, RuntimeArtifactHot, RuntimeArtifactWarm};
use crate::diagnostics::lineage::LineageArtifactId;
use crate::logic::transaction::{ArtifactMergeComparable, DependencyFingerprint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NodeMergeProjection {
    pub(super) comparable: ArtifactMergeComparable,
    pub(super) current_artifact_id: Option<LineageArtifactId>,
    pub(super) authority: ArtifactMergeAuthority,
}

pub(super) fn node_merge_projection(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<Option<NodeMergeProjection>, SignalError> {
    let hot = graph.node_runtime_artifact_hot(node)?;
    let warm = graph.node_runtime_artifact_warm(node)?;
    match (hot, warm) {
        (Some(hot), Some(warm)) => Ok(Some(NodeMergeProjection {
            comparable: merge_comparable_from_lanes(hot, warm),
            current_artifact_id: warm.lineage_artifact_id.get(),
            authority: warm.merge_authority.clone(),
        })),
        (None, None) => Ok(None),
        _ => Err(SignalError::internal(format!(
            "runtime artifact hot/warm lane mismatch for merge-comparable node {}",
            node
        ))),
    }
}

fn merge_comparable_from_lanes(
    hot: &RuntimeArtifactHot,
    warm: &RuntimeArtifactWarm,
) -> ArtifactMergeComparable {
    ArtifactMergeComparable {
        output_identity: warm.output_identity.clone(),
        continuity_token: warm.continuity_token.clone_inner(),
        reuse_basis: warm.reuse_basis.clone_inner(),
        dependency_fingerprint: DependencyFingerprint {
            dependency_count: hot.dependency_count,
            meaningful_input_changes: hot.meaningful_input_changes,
            output_hash: hot.output_hash,
        },
        authority: warm.merge_authority.clone(),
    }
}
