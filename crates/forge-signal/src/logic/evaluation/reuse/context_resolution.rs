use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::CanonicalChangedRegions;
use crate::data::output::NodeEvaluationResult;
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::{ReuseBoundaryContext, ReuseSemanticRegionIdentity};
use crate::logic::prepared::PreparedKeyedContext;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn resolve_reuse_boundary_context(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
    result: Option<&NodeEvaluationResult>,
    keyed: Option<&PreparedKeyedContext>,
) -> Result<ReuseBoundaryContext, SignalError> {
    let entry = graph.get_entry(node)?;
    let eval = entry.get_eval_config();
    let contract = &eval.contract;
    let dependency_snapshot = graph.get_dep_snapshot(node)?;
    let partition_region_basis = PartitionScopeSet::from(
        contract
            .semantics
            .partition_scope
            .as_deref()
            .unwrap_or(&[]),
    );
    Ok(ReuseBoundaryContext {
        topology_regime: entry.get_dependencies_id().raw(),
        tolerance_regime: comparator_resolver.policy_for_node(node, eval.comparator.as_ref()),
        semantic_region: ReuseSemanticRegionIdentity::new(
            node,
            eval.partitioned_output,
            contract
                .semantics
                .partition_scope
                .clone()
                .unwrap_or_default(),
            contract.semantics.required_context,
        ),
        authority_policy: contract.authority.policy,
        artifact_family: keyed
            .and_then(|prepared| prepared.family.as_ref())
            .map(|family| family.as_str().to_owned()),
        structural_dependency_basis: compact_hash(dependency_snapshot),
        partition_region_basis: partition_region_basis.clone(),
        persistent_correspondence: keyed.and_then(|prepared| prepared.persistent_correspondence.clone()),
        composition_regions: result
            .map(|output| {
                PartitionScopeSet::from_changed_regions(&CanonicalChangedRegions::from(
                    output.changed_regions.as_slice(),
                ))
            })
            .filter(|regions| !regions.is_empty())
            .or_else(|| {
                keyed.and_then(|prepared| {
                    (!prepared.composition_regions.is_empty())
                        .then_some(prepared.composition_regions.clone())
                })
            })
            .unwrap_or(partition_region_basis),
    })
}

fn compact_hash<T: Hash>(value: &T) -> u32 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & u32::MAX as u64) as u32
}
