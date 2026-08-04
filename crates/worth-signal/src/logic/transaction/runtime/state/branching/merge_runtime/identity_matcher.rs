use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergeRequest, IdentityMatchPolicy, IdentityMatcherDescriptor,
    IdentityMatcherName, IdentityMatcherSelectionBasis,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::FrozenIdentityMatcherRegistry;

#[derive(Debug, Clone)]
pub(super) struct ResolvedIdentityMatcherSelection {
    pub(super) descriptor: IdentityMatcherDescriptor,
    pub(super) basis: IdentityMatcherSelectionBasis,
}

pub(super) fn resolve_identity_matcher(
    registry: &FrozenIdentityMatcherRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: IdentityMatchPolicy,
) -> Result<ResolvedIdentityMatcherSelection, SignalError> {
    if let Some(matcher_name) = request.identity_matcher_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(matcher_name)
            .cloned()
            .ok_or_else(|| unknown_matcher(matcher_name.as_str()))?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::RequestNamed,
        });
    }
    if let Some(matcher_name) = unanimous_node_identity_matcher_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&matcher_name)
            .cloned()
            .ok_or_else(|| unknown_matcher(matcher_name.as_str()))?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::NodeOverride,
        });
    }
    if let Some(matcher_name) =
        unanimous_schema_identity_matcher_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&matcher_name)
            .cloned()
            .ok_or_else(|| unknown_matcher(matcher_name.as_str()))?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "identity matcher {:?} has no registered descriptor in the frozen identity matcher registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedIdentityMatcherSelection {
        descriptor,
        basis: IdentityMatcherSelectionBasis::BuiltInDefault,
    })
}

fn unknown_matcher(name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "identity matcher `{}` is not registered in the frozen identity matcher registry",
            name
        ),
    )
}

fn unanimous_node_identity_matcher_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<IdentityMatcherName>, SignalError> {
    let mut selected: Option<IdentityMatcherName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_identity_matcher_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting identity matcher overrides: `{}` vs `{}`",
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

fn unanimous_schema_identity_matcher_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<IdentityMatcherName>, SignalError> {
    let mut selected: Option<IdentityMatcherName> = None;
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
                    "node {} references unknown schema id `{}` during identity matcher selection",
                    node,
                    binding.schema_id().0
                ),
                )
            })?;
        let Some(candidate) = descriptor.default_identity_matcher_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default identity matchers: `{}` vs `{}`",
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
