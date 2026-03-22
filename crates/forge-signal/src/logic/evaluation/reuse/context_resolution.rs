use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::CanonicalChangedRegions;
use crate::data::output::NodeEvaluationResult;
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::{
    ArtifactFamilyId, ReuseBoundaryContext, ReuseSemanticRegionIdentity,
    ReuseStrategyBoundaryContext,
};
use crate::logic::prepared::PreparedKeyedContext;

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
    let partition_region_basis = PartitionScopeSet::from(
        contract
            .semantics
            .partition_scope
            .as_deref()
            .unwrap_or(&[]),
    );
    let composition_regions = keyed
        .and_then(|prepared| {
            (!prepared.composition_regions.is_empty()).then_some(prepared.composition_regions.clone())
        })
        .or_else(|| {
            result
                .map(|output| {
                    PartitionScopeSet::from_changed_regions(&CanonicalChangedRegions::from(
                        output.changed_regions.as_slice(),
                    ))
                })
                .filter(|regions| !regions.is_empty())
        });
    let strategy_detail = keyed
        .and_then(|prepared| {
            prepared
                .persistent_correspondence
                .clone()
                .map(|persistent_correspondence| {
                    ReuseStrategyBoundaryContext::CrossIdentity {
                        persistent_correspondence,
                    }
                })
        })
        .or_else(|| {
            composition_regions
                .clone()
                .map(|composition_regions| ReuseStrategyBoundaryContext::PartialArtifactSplice {
                    composition_regions,
                })
        })
        .unwrap_or_default();

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
            .map(|family| ArtifactFamilyId::new(family.as_str())),
        structural_dependency_basis: entry.get_dep_snapshot_id(),
        partition_region_basis: partition_region_basis.clone(),
        strategy_detail,
    })
}
