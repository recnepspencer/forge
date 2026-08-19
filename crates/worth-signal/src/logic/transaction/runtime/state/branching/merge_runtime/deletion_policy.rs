use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergeRequest, DeletionMergePolicy, DeletionPolicyDescriptor,
    DeletionPolicyName, DeletionPolicySelectionBasis, LoweredDeletionPolicyPlan,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::FrozenDeletionPolicyRegistry;
use super::super::super::merge::{
    deny_selected_target_rejected_by_declaration, scoped_admission_outcome_to_signal_error,
};
use super::super::super::runtime_state::SignalRuntime;

#[derive(Debug, Clone)]
pub(super) struct ResolvedDeletionPolicySelection {
    pub(super) descriptor: DeletionPolicyDescriptor,
    pub(super) basis: DeletionPolicySelectionBasis,
}

pub(super) fn lower_deletion_plan<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    lowered_request: &crate::logic::transaction::runtime::LoweredFoundationalMergeRequest,
    target_only_nodes: Vec<NodeId>,
    selection: &ResolvedDeletionPolicySelection,
    denial_anchor: Option<NodeId>,
) -> Result<LoweredDeletionPolicyPlan, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let rejected_target_only = matches!(
        selection.descriptor.policy(),
        DeletionMergePolicy::RejectTargetOnlyConflict
    ) && !target_only_nodes.is_empty();
    let plan = LoweredDeletionPolicyPlan {
        target_only_count: target_only_nodes.len() as u64,
        rejected_target_only_count: u64::from(rejected_target_only),
        target_only_nodes: target_only_nodes.clone(),
    };
    if !rejected_target_only {
        return Ok(plan);
    }
    if !matches!(
        lowered_request
            .normalized_request()
            .normalized_scope()
            .family(),
        crate::logic::transaction::BranchMergeRequestScopeFamily::FullBranch
    ) {
        runtime.with_telemetry(|telemetry| telemetry.transaction.scoped_merge_denial_count += 1);
        let denied_node = denial_anchor.or_else(|| target_only_nodes.first().copied()).ok_or_else(
            || {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::DivergenceRequiresConflictResolution,
                    "deletion policy rejection has no source node available for scoped denial evidence",
                )
            },
        )?;
        return Err(scoped_admission_outcome_to_signal_error(
            deny_selected_target_rejected_by_declaration(lowered_request, denied_node),
        ));
    }
    Err(SignalError::branch_merge_failed_with_evidence(
        BranchMergeFailureKind::DivergenceRequiresConflictResolution,
        format!(
            "deletion policy `{}` rejects {} target-only branch delta node(s)",
            selection.descriptor.semantic_name().as_str(),
            target_only_nodes.len()
        ),
        crate::logic::transaction::runtime::BranchMergeFailureEvidence::Deletion(
            crate::logic::transaction::BranchMergeDeletionFailureEvidence {
                deletion_policy_name: selection.descriptor.semantic_name().clone(),
                target_only_nodes,
                deletion_plan: plan,
            },
        ),
    ))
}

pub(super) fn resolve_deletion_policy(
    registry: &FrozenDeletionPolicyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: DeletionMergePolicy,
) -> Result<ResolvedDeletionPolicySelection, SignalError> {
    if let Some(policy_name) = request.deletion_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy(policy_name.as_str()))?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) = unanimous_node_deletion_policy_name(source_graph, candidate_nodes)? {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy(policy_name.as_str()))?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) =
        unanimous_schema_deletion_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| unknown_policy(policy_name.as_str()))?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "deletion policy {:?} has no registered descriptor in the frozen deletion policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedDeletionPolicySelection {
        descriptor,
        basis: DeletionPolicySelectionBasis::BuiltInDefault,
    })
}

fn unknown_policy(name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "deletion policy `{}` is not registered in the frozen deletion policy registry",
            name
        ),
    )
}

fn unanimous_node_deletion_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<DeletionPolicyName>, SignalError> {
    let mut selected: Option<DeletionPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_deletion_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting deletion policy overrides: `{}` vs `{}`",
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

fn unanimous_schema_deletion_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<DeletionPolicyName>, SignalError> {
    let mut selected: Option<DeletionPolicyName> = None;
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
                    "node {} references unknown schema id `{}` during deletion policy selection",
                    node,
                    binding.schema_id().0
                ),
                )
            })?;
        let Some(candidate) = descriptor.default_deletion_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default deletion policies: `{}` vs `{}`",
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
