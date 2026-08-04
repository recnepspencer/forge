use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{AspectMergePolicyName, BranchMergeFailureKind};
use crate::schema::data::SignalSchemaRegistry;

pub(super) fn unanimous_node_aspect_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    aspect: Aspect,
) -> Result<Option<AspectMergePolicyName>, SignalError> {
    let mut selected = None;
    for node in candidate_nodes {
        let candidate = source_graph
            .node_aspect_merge_policy_bindings(*node)?
            .iter()
            .find(|binding| binding.aspect == aspect)
            .map(|binding| binding.policy_name.clone());
        let Some(candidate) = candidate else { continue };
        select_same_policy(
            &mut selected,
            candidate,
            format!(
                "merge candidate nodes declare conflicting aspect merge policy overrides for aspect {}",
                aspect.id()
            ),
        )?;
    }
    Ok(selected)
}

pub(super) fn unanimous_schema_aspect_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
    aspect: Aspect,
) -> Result<Option<AspectMergePolicyName>, SignalError> {
    let mut selected = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let descriptor = schema_registry.resolve_by_id(binding.schema_id()).ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during aspect merge policy selection",
                    node,
                    binding.schema_id().0
                ),
            )
        })?;
        let candidate = descriptor
            .default_aspect_merge_policy_bindings()
            .iter()
            .find(|binding| binding.aspect == aspect)
            .map(|binding| binding.policy_name.clone());
        let Some(candidate) = candidate else { continue };
        select_same_policy(
            &mut selected,
            candidate,
            format!(
                "merge candidate schemas declare conflicting default aspect merge policies for aspect {}",
                aspect.id()
            ),
        )?;
    }
    Ok(selected)
}

fn select_same_policy(
    selected: &mut Option<AspectMergePolicyName>,
    candidate: AspectMergePolicyName,
    message: String,
) -> Result<(), SignalError> {
    if let Some(existing) = selected.as_ref() {
        if existing != &candidate {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "{}: `{}` vs `{}`",
                    message,
                    existing.as_str(),
                    candidate.as_str()
                ),
            ));
        }
    } else {
        *selected = Some(candidate);
    }
    Ok(())
}
