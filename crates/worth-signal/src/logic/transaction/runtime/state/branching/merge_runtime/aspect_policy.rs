use std::collections::BTreeMap;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    AspectMergeDecisionOutcome, AspectMergePolicy, AspectMergePolicyDescriptor,
    AspectMergePolicySelectionBasis, BranchMergeFailureKind, BranchMergeRequest,
    FrozenAspectMergePolicyRegistry, LoweredAspectMergeDecisionPlan,
    LoweredAspectMergeDecisionRecord, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::{LoweredAspectMergePolicyPlan, LoweredAspectMergePolicyRecord};

#[derive(Debug, Clone)]
pub(super) struct ResolvedAspectPolicySelection {
    pub(super) descriptor: AspectMergePolicyDescriptor,
    pub(super) basis: AspectMergePolicySelectionBasis,
}

pub(super) fn lower_aspect_policy_plan(
    registry: &FrozenAspectMergePolicyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
) -> Result<LoweredAspectMergePolicyPlan, SignalError> {
    let mut nodes_by_aspect: BTreeMap<u8, Vec<NodeId>> = BTreeMap::new();
    for node in candidate_nodes {
        let produces = source_graph
            .node_eval_config(*node)?
            .contract
            .semantics
            .produces;
        if produces == AspectMask::ALL || produces.is_empty() {
            continue;
        }
        for aspect in iter_declared_aspects(produces) {
            nodes_by_aspect.entry(aspect.id()).or_default().push(*node);
        }
    }
    let mut records = Vec::new();
    for (aspect_id, affected_source_nodes) in nodes_by_aspect {
        let aspect = Aspect::new(aspect_id);
        let resolved = resolve_aspect_policy(
            registry,
            schema_registry,
            source_graph,
            &affected_source_nodes,
            request,
            aspect,
            AspectMergePolicy::RequireConflict,
        )?;
        records.push(LoweredAspectMergePolicyRecord {
            aspect,
            selected_policy_name: resolved.descriptor.semantic_name().clone(),
            selected_policy_digest: resolved.descriptor.digest().to_string(),
            selected_policy_basis: resolved.basis,
            affected_source_nodes,
        });
    }
    Ok(LoweredAspectMergePolicyPlan { records })
}

pub(super) fn lower_aspect_decision_plan(
    aspect_policy_plan: &LoweredAspectMergePolicyPlan,
    node_plan: &[NodeMergePlan],
) -> LoweredAspectMergeDecisionPlan {
    let node_plan_by_source = node_plan
        .iter()
        .map(|plan| (plan.source_node(), plan))
        .collect::<BTreeMap<_, _>>();
    let mut records = Vec::new();
    for policy_record in &aspect_policy_plan.records {
        for source_node in &policy_record.affected_source_nodes {
            let Some(node_plan) = node_plan_by_source.get(source_node) else {
                continue;
            };
            let target_node = match node_plan.shape() {
                NodeReconciliationShape::ExistingTargetNode { target_node } => Some(target_node),
                NodeReconciliationShape::SourceOnlyIntroduction => None,
            };
            let outcome = match node_plan.decision() {
                NodeReconciliationDecision::AdoptSourceAuthority => {
                    if matches!(
                        node_plan.shape(),
                        NodeReconciliationShape::SourceOnlyIntroduction
                    ) {
                        AspectMergeDecisionOutcome::SourceIntroducedIntoTarget
                    } else {
                        AspectMergeDecisionOutcome::SourceAuthorityAdopted
                    }
                }
                NodeReconciliationDecision::MarkEquivalentUnchanged => {
                    AspectMergeDecisionOutcome::EquivalentUnchanged
                }
                NodeReconciliationDecision::PreserveTarget => {
                    AspectMergeDecisionOutcome::TargetPreserved
                }
                NodeReconciliationDecision::SkipNonAdoptableSource => {
                    AspectMergeDecisionOutcome::SourceSkippedNonAdoptable
                }
                NodeReconciliationDecision::ReplaceTargetAuthority => {
                    AspectMergeDecisionOutcome::SourceAuthorityAdopted
                }
                NodeReconciliationDecision::RejectRequiresConflictResolution => {
                    AspectMergeDecisionOutcome::ConflictRequired
                }
            };
            records.push(LoweredAspectMergeDecisionRecord {
                aspect: policy_record.aspect,
                source_node: *source_node,
                target_node,
                selected_policy_name: policy_record.selected_policy_name.clone(),
                selected_policy_digest: policy_record.selected_policy_digest.clone(),
                selected_policy_basis: policy_record.selected_policy_basis,
                outcome,
            });
        }
    }
    LoweredAspectMergeDecisionPlan { records }
}

pub(super) fn iter_declared_aspects(mask: AspectMask) -> impl Iterator<Item = Aspect> {
    (0..crate::data::aspect::MAX_ASPECTS)
        .map(|index| Aspect::new(index as u8))
        .filter(move |aspect| mask.contains(AspectMask::from_aspect(*aspect)))
}

fn resolve_aspect_policy(
    registry: &FrozenAspectMergePolicyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    aspect: Aspect,
    default_policy: AspectMergePolicy,
) -> Result<ResolvedAspectPolicySelection, SignalError> {
    let request_bindings = request
        .aspect_policy_bindings
        .iter()
        .filter(|binding| binding.aspect() == aspect)
        .map(|binding| binding.policy_name().clone())
        .collect::<Vec<_>>();
    if request_bindings.len() > 1 {
        return Err(SignalError::branch_merge_failed(
            BranchMergeFailureKind::UnsupportedMergeStrategy,
            format!(
                "request declared multiple aspect merge policies for aspect {}",
                aspect.id()
            ),
        ));
    }
    if let Some(policy_name) = request_bindings.into_iter().next() {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_aspect_policy("request aspect merge policy", policy_name.as_str())
            })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) = super::aspect_policy_selection::unanimous_node_aspect_policy_name(
        source_graph,
        candidate_nodes,
        aspect,
    )? {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_aspect_policy("node aspect merge policy override", policy_name.as_str())
            })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) = super::aspect_policy_selection::unanimous_schema_aspect_policy_name(
        source_graph,
        schema_registry,
        candidate_nodes,
        aspect,
    )? {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_aspect_policy("schema default aspect merge policy", policy_name.as_str())
            })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::SchemaDefault,
        });
    }
    let policy_name =
        crate::logic::transaction::runtime::AspectMergePolicyName::new(match default_policy {
            AspectMergePolicy::RequireConflict => "signal.aspect.require-conflict",
            AspectMergePolicy::PreferSource => "signal.aspect.prefer-source",
            AspectMergePolicy::PreferTarget => "signal.aspect.prefer-target",
        });
    let descriptor = registry
        .resolve_by_name(&policy_name)
        .cloned()
        .ok_or_else(|| {
            unknown_aspect_policy("aspect merge policy", &format!("{:?}", default_policy))
        })?;
    Ok(ResolvedAspectPolicySelection {
        descriptor,
        basis: AspectMergePolicySelectionBasis::BuiltInDefault,
    })
}

fn unknown_aspect_policy(family: &str, name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "{} `{}` is not registered in the frozen aspect merge policy registry",
            family, name
        ),
    )
}
