use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::reuse::{ReuseBoundaryContext, ReuseSemanticRegionIdentity};

pub(crate) fn resolve_reuse_boundary_context(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<ReuseBoundaryContext, SignalError> {
    let entry = graph.get_entry(node)?;
    let eval = entry.get_eval_config();
    let contract = &eval.contract;
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
    })
}
