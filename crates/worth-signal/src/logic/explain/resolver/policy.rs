use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;

use super::super::types::NodeExplanation;

pub(super) struct ExplanationDiagnosticPolicy {
    allow_retained_fast_path: bool,
}

impl ExplanationDiagnosticPolicy {
    pub(super) fn retained_or_reconstruct() -> Self {
        Self {
            allow_retained_fast_path: true,
        }
    }

    pub(super) fn reconstruct_only() -> Self {
        Self {
            allow_retained_fast_path: false,
        }
    }

    pub(super) fn retained_fast_path_allowed(&self) -> bool {
        self.allow_retained_fast_path
    }
}

pub(super) fn retained_explanation(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<Option<NodeExplanation>, SignalError> {
    let entry = graph.get_entry(node)?;
    let Some(fact) = graph.explanation_fact(node) else {
        return Ok(None);
    };
    let current_record = entry.historical_artifact_record(node);
    if (!fact.compact_projection || fact.explanation.rewiring.is_some())
        && fact.explanation.state == *entry.get_state()
        && fact.explanation.historical_artifact_record == current_record
    {
        return Ok(Some(fact.explanation.clone()));
    }
    Ok(None)
}
