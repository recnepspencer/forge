use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergeRequest, ConflictMergePolicy, ConflictPolicyDescriptor,
    ConflictPolicyName, ConflictPolicySelectionBasis,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::FrozenConflictPolicyRegistry;

#[derive(Debug, Clone)]
pub(super) struct ResolvedConflictPolicySelection {
    pub(super) descriptor: ConflictPolicyDescriptor,
    pub(super) basis: ConflictPolicySelectionBasis,
}

pub(super) fn resolve_conflict_policy(
    registry: &FrozenConflictPolicyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: ConflictMergePolicy,
) -> Result<ResolvedConflictPolicySelection, SignalError> {
    if let Some(policy_name) = request.conflict_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy("conflict policy", policy_name.as_str()))?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) = unanimous_node_conflict_policy_name(source_graph, candidate_nodes)? {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy("node conflict policy override", policy_name.as_str()))?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) =
        unanimous_schema_conflict_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_policy("schema default conflict policy", policy_name.as_str())
            })?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "conflict policy {:?} has no registered descriptor in the frozen conflict policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedConflictPolicySelection {
        descriptor,
        basis: ConflictPolicySelectionBasis::BuiltInDefault,
    })
}

fn unknown_policy(family: &str, name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "{} `{}` is not registered in the frozen conflict policy registry",
            family, name
        ),
    )
}

fn unanimous_node_conflict_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictPolicyName>, SignalError> {
    let mut selected: Option<ConflictPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_conflict_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting conflict policy overrides: `{}` vs `{}`",
                        existing.as_str(), candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_conflict_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictPolicyName>, SignalError> {
    let mut selected: Option<ConflictPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let descriptor = schema_registry
            .resolve_by_id(binding.schema_id())
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                    "node {} references unknown schema id `{}` during conflict policy selection",
                    node,
                    binding.schema_id().0
                ),
                )
            })?;
        let Some(candidate) = descriptor.default_conflict_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default conflict policies: `{}` vs `{}`",
                        existing.as_str(), candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}
