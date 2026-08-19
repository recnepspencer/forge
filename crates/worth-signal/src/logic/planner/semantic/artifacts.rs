use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::logic::explain::RewiringSummary;

pub(super) fn record_semantic_artifacts(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    rewiring: Option<&RewiringSummary>,
) -> Result<(), SignalError> {
    let policy = graph.installed_runtime_policy();
    if !policy.retains_explanation_facts() && !policy.retains_provenance_facts() {
        return Ok(());
    }

    graph.record_operational_diagnostic_facts(node, rewiring.cloned())
}
