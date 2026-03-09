use crate::data::comparator::VersionComparatorResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::evaluation::{ConditionResolver, EvaluationRequestMode};

/// Transitional callback-era test adapter.
///
/// This module exists only to keep a small set of legacy condition/comparator tests alive while
/// the suite finishes migrating to prepared execution. Do not use it in new tests.
pub fn evaluate<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: crate::data::output::IntoNodeEvaluationResult,
{
    crate::logic::evaluation::evaluate(graph, node, compute)
}

pub fn evaluate_on_demand<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: crate::data::output::IntoNodeEvaluationResult,
{
    crate::logic::evaluation::evaluate_on_demand(graph, node, compute)
}

pub fn evaluate_with_resolver<F, O, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    custom_resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: crate::data::output::IntoNodeEvaluationResult,
    R: VersionComparatorResolver,
{
    crate::logic::evaluation::evaluate_with_resolver(graph, node, compute, custom_resolver)
}

pub fn evaluate_with_resolvers<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    custom_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: crate::data::output::IntoNodeEvaluationResult,
    R: VersionComparatorResolver,
    C: ConditionResolver,
{
    crate::logic::evaluation::evaluate_with_resolvers(
        graph,
        node,
        compute,
        custom_resolver,
        condition_resolver,
        request_mode,
    )
}
