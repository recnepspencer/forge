use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergeRequest, SourceOnlyMergePolicy, SourceOnlyPolicyDescriptor,
    SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::FrozenSourceOnlyPolicyRegistry;

#[derive(Debug, Clone)]
pub(super) struct ResolvedSourceOnlyPolicySelection {
    pub(super) descriptor: SourceOnlyPolicyDescriptor,
    pub(super) basis: SourceOnlyPolicySelectionBasis,
}

pub(super) fn resolve_source_only_policy(
    registry: &FrozenSourceOnlyPolicyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: SourceOnlyMergePolicy,
) -> Result<ResolvedSourceOnlyPolicySelection, SignalError> {
    if let Some(policy_name) = request.source_only_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy("source-only policy", policy_name.as_str()))?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) =
        unanimous_node_source_only_policy_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_policy("node source-only policy override", policy_name.as_str())
            })?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) =
        unanimous_schema_source_only_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_policy("schema default source-only policy", policy_name.as_str())
            })?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "source-only policy {:?} has no registered descriptor in the frozen source-only policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedSourceOnlyPolicySelection {
        descriptor,
        basis: SourceOnlyPolicySelectionBasis::BuiltInDefault,
    })
}

fn unknown_policy(family: &str, name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "{} `{}` is not registered in the frozen source-only policy registry",
            family, name
        ),
    )
}

fn unanimous_node_source_only_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<SourceOnlyPolicyName>, SignalError> {
    let mut selected: Option<SourceOnlyPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_source_only_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting source-only policy overrides: `{}` vs `{}`",
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

fn unanimous_schema_source_only_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<SourceOnlyPolicyName>, SignalError> {
    let mut selected: Option<SourceOnlyPolicyName> = None;
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
                    "node {} references unknown schema id `{}` during source-only policy selection",
                    node,
                    binding.schema_id().0
                ),
                )
            })?;
        let Some(candidate) = descriptor.default_source_only_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default source-only policies: `{}` vs `{}`",
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
