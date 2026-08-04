use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeBase, BranchMergeFailureKind, BranchMergeRequest, BranchMergeStrategy,
    LoweredMergeBasePlan, MergeBaseSelectionBasis, MergeBaseSelectionPolicy,
    MergeBaseStrategyDescriptor, MergeStrategyDescriptor, MergeStrategyName,
    MergeStrategySelectionBasis,
};
use crate::schema::data::SignalSchemaRegistry;

use super::super::super::merge::{FrozenMergeBaseStrategyRegistry, FrozenMergeStrategyRegistry};
use super::candidates::BranchStateDiscovery;

#[derive(Debug, Clone)]
pub(super) struct ResolvedMergeBaseSelection {
    pub(super) descriptor: MergeBaseStrategyDescriptor,
    pub(super) basis: MergeBaseSelectionBasis,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedMergeStrategySelection {
    pub(super) descriptor: MergeStrategyDescriptor,
    pub(super) basis: MergeStrategySelectionBasis,
}

pub(super) struct MergeBaseResolution {
    pub(super) base: BranchMergeBase,
    pub(super) lowered: LoweredMergeBasePlan,
}

pub(super) fn resolve_base<D, I, T>(
    registry: &FrozenMergeBaseStrategyRegistry,
    request: &BranchMergeRequest,
    states: &BranchStateDiscovery<D, I, T>,
) -> Result<MergeBaseResolution, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let resolved = resolve_merge_base_descriptor(
        registry,
        request,
        MergeBaseSelectionPolicy::ForkPointSnapshot,
    )?;
    let forked_from_snapshot_id = match resolved.descriptor.policy() {
        MergeBaseSelectionPolicy::ForkPointSnapshot => {
            states.source_state.ancestry().forked_from_snapshot_id()
        }
    };
    let base = BranchMergeBase {
        source_branch_id: request.source_branch.id,
        target_branch_id: request.target_branch.id,
        forked_from_snapshot_id,
        source_snapshot_id: states.source_snapshot_id,
        target_snapshot_id_before: states.target_snapshot_id_before,
    };
    let lowered = LoweredMergeBasePlan {
        resolved_base: base.clone(),
        selected_merge_base_name: resolved.descriptor.semantic_name().clone(),
        selected_merge_base_digest: resolved.descriptor.digest().to_string(),
        selected_merge_base_basis: resolved.basis,
    };
    Ok(MergeBaseResolution { base, lowered })
}

pub(super) fn resolve_merge_strategy(
    registry: &FrozenMergeStrategyRegistry,
    schema_registry: &SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_strategy: BranchMergeStrategy,
) -> Result<ResolvedMergeStrategySelection, SignalError> {
    if let Some(strategy_name) = request.strategy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(strategy_name)
            .cloned()
            .ok_or_else(|| unknown_strategy("merge strategy", strategy_name.as_str()))?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::RequestNamed,
        });
    }
    if let Some(strategy_hint) = request.strategy_hint {
        let descriptor = registry
            .first_matching_strategy(strategy_hint)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge strategy {:?} has no registered descriptor in the frozen merge strategy registry",
                        strategy_hint
                    ),
                )
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::RequestHint,
        });
    }
    if let Some(strategy_name) = unanimous_node_override_name(source_graph, candidate_nodes)? {
        let descriptor = registry
            .resolve_by_name(&strategy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_strategy("node merge strategy override", strategy_name.as_str())
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::NodeOverride,
        });
    }
    if let Some(strategy_name) =
        unanimous_schema_default_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&strategy_name)
            .cloned()
            .ok_or_else(|| {
                unknown_strategy("schema default merge strategy", strategy_name.as_str())
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .first_matching_strategy(default_strategy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "merge strategy {:?} has no registered descriptor in the frozen merge strategy registry",
                    default_strategy
                ),
            )
        })?;
    Ok(ResolvedMergeStrategySelection {
        descriptor,
        basis: MergeStrategySelectionBasis::DivergenceDefault,
    })
}

pub(super) fn resolve_merge_base_descriptor(
    registry: &FrozenMergeBaseStrategyRegistry,
    request: &BranchMergeRequest,
    default_policy: MergeBaseSelectionPolicy,
) -> Result<ResolvedMergeBaseSelection, SignalError> {
    if let Some(strategy_name) = request.merge_base_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(strategy_name)
            .cloned()
            .ok_or_else(|| unknown_strategy("merge-base strategy", strategy_name.as_str()))?;
        return Ok(ResolvedMergeBaseSelection {
            descriptor,
            basis: MergeBaseSelectionBasis::RequestNamed,
        });
    }
    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::MissingMergeBase,
                "no built-in merge-base strategy matches the default selection policy",
            )
        })?;
    Ok(ResolvedMergeBaseSelection {
        descriptor,
        basis: MergeBaseSelectionBasis::BuiltInDefault,
    })
}

fn unknown_strategy(family: &str, name: &str) -> SignalError {
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "{} `{}` is not registered in the frozen merge strategy registry",
            family, name
        ),
    )
}

fn unanimous_node_override_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<MergeStrategyName>, SignalError> {
    let mut selected: Option<MergeStrategyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_merge_strategy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting merge strategy overrides: `{}` vs `{}`",
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

fn unanimous_schema_default_name(
    source_graph: &SignalGraph,
    schema_registry: &SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<MergeStrategyName>, SignalError> {
    let mut selected: Option<MergeStrategyName> = None;
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
                        "node {} references unknown schema id `{}` during merge strategy selection",
                        node,
                        binding.schema_id().0
                    ),
                )
            })?;
        let Some(candidate) = descriptor.default_merge_strategy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default merge strategies: `{}` vs `{}`",
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
