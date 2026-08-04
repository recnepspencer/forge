use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeConflictRecord, BranchMergeFailureKind, BranchMergeRequest,
    ConflictIsolationGranularity, ConflictIsolationPolicyDescriptor, ConflictIsolationPolicyName,
    ConflictIsolationSelectionBasis,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::{
    ConflictIsolationWitness, ConservativeIsolationExpansion, FrozenConflictIsolationRegistry,
    LoweredConflictIsolationPlan, LoweredConflictIsolationRecord, RegionIsolationSummary,
};

#[derive(Debug, Clone)]
pub(super) struct ResolvedConflictIsolationSelection {
    pub(super) descriptor: ConflictIsolationPolicyDescriptor,
    pub(super) basis: ConflictIsolationSelectionBasis,
}

pub(super) fn resolve_conflict_isolation(
    registry: &FrozenConflictIsolationRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_granularity: ConflictIsolationGranularity,
) -> Result<ResolvedConflictIsolationSelection, SignalError> {
    if let Some(policy_name) = request.conflict_isolation_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| unknown_isolation(policy_name.as_str()))?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) =
        unanimous_node_conflict_isolation_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| unknown_isolation(policy_name.as_str()))?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) =
        unanimous_schema_conflict_isolation_name(schema_registry, source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| unknown_isolation(policy_name.as_str()))?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::SchemaDefault,
        });
    }
    let policy_name = match default_granularity {
        ConflictIsolationGranularity::PerNode
        | ConflictIsolationGranularity::HostDeclaredRegion => {
            ConflictIsolationPolicyName::new("signal.conflict-isolation.per-node")
        }
        ConflictIsolationGranularity::PerAspect => {
            ConflictIsolationPolicyName::new("signal.conflict-isolation.per-aspect")
        }
    };
    let descriptor = registry
        .resolve_by_name(&policy_name)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "conflict isolation default {:?} has no registered descriptor",
                    default_granularity
                ),
            )
        })?;
    Ok(ResolvedConflictIsolationSelection {
        descriptor,
        basis: ConflictIsolationSelectionBasis::BuiltInDefault,
    })
}

pub(super) fn lower_conflict_isolation_plan(
    selection: &ResolvedConflictIsolationSelection,
    source_graph: &SignalGraph,
    conflict_records: &[BranchMergeConflictRecord],
) -> Result<LoweredConflictIsolationPlan, SignalError> {
    let granularity = selection.descriptor.granularity();
    let mut records = Vec::new();
    for record in conflict_records {
        let isolated_aspects = match granularity {
            ConflictIsolationGranularity::PerAspect => {
                let produces = source_graph
                    .node_eval_config(record.source_node)?
                    .contract
                    .semantics
                    .produces;
                if produces == AspectMask::ALL || produces.is_empty() {
                    Vec::new()
                } else {
                    super::aspect_policy::iter_declared_aspects(produces).collect()
                }
            }
            ConflictIsolationGranularity::PerNode
            | ConflictIsolationGranularity::HostDeclaredRegion => Vec::new(),
        };
        records.push(LoweredConflictIsolationRecord {
            source_node: record.source_node,
            target_node: Some(record.target_node),
            granularity,
            isolated_aspects,
        });
    }
    Ok(LoweredConflictIsolationPlan {
        selected_policy_name: Some(selection.descriptor.semantic_name().clone()),
        selected_policy_digest: Some(selection.descriptor.digest().to_string()),
        selected_policy_basis: Some(selection.basis),
        expansion_breadth: 0,
        witness: Some(ConflictIsolationWitness {
            granularity,
            conflict_record_count: conflict_records.len() as u64,
        }),
        region_summary: RegionIsolationSummary {
            isolated_region_count: records.len() as u64,
            host_declared_region_count: u64::from(matches!(
                granularity,
                ConflictIsolationGranularity::HostDeclaredRegion
            )),
        },
        conservative_expansion: ConservativeIsolationExpansion {
            expanded_node_count: 0,
        },
        records,
    })
}

fn unknown_isolation(name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "branch merge request references unknown conflict isolation policy `{}`",
            name
        ),
    )
}

fn unanimous_node_conflict_isolation_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictIsolationPolicyName>, SignalError> {
    let mut selected: Option<ConflictIsolationPolicyName> = None;
    for node in candidate_nodes {
        let Some(name) = source_graph.node_conflict_isolation_policy_name(*node)? else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != name {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    "candidate nodes disagree on per-node conflict isolation policy",
                ));
            }
        } else {
            selected = Some(name.clone());
        }
    }
    Ok(selected)
}

fn unanimous_schema_conflict_isolation_name(
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictIsolationPolicyName>, SignalError> {
    let mut selected: Option<ConflictIsolationPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let descriptor = schema_registry.resolve_by_id(binding.schema_id()).ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during conflict isolation resolution",
                    node,
                    binding.schema_id().0
                ),
            )
        })?;
        let Some(name) = descriptor.default_conflict_isolation_policy_name() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != name {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    "candidate nodes disagree on schema-owned conflict isolation policy",
                ));
            }
        } else {
            selected = Some(name.clone());
        }
    }
    Ok(selected)
}
