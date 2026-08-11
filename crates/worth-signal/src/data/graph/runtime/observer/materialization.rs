use super::GraphMaterializer;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::trace::{HistoricalArtifactRecord, TraceSummary};
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::logic::explain::NodeExplanation;

impl<'a> GraphMaterializer<'a> {
    pub fn materialize_historical_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<HistoricalArtifactRecord>, SignalError> {
        crate::data::access_counters::note_reconstructed_artifact_read();
        let entry = self.graph.get_entry(node)?;
        Ok(entry.historical_artifact_record(node))
    }

    pub fn materialize_trace_summary(
        &self,
        node: NodeId,
    ) -> Result<Option<TraceSummary>, SignalError> {
        crate::data::access_counters::note_reconstructed_artifact_read();
        let entry = self.graph.get_entry(node)?;
        Ok(entry.trace_summary())
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        let fact = self
            .graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node)?;
        let mut explanation = if fact.compact_projection {
            self.graph
                .reconstruct_explanation_artifact_without_retained_fast_path(node)
                .ok()?
        } else {
            fact.explanation.clone()
        };
        explanation.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
        self.graph.record_retained_forensic_read();
        self.graph.record_retained_artifact_read();
        Some(explanation)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph.reconstruct_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        let explanation_fact = self
            .graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node);
        let mut fact = match (
            explanation_fact.map(|fact| fact.compact_projection),
            self.graph
                .observation
                .diagnostics
                .provenance_facts()
                .get(&node)
                .cloned(),
        ) {
            (Some(true), _) => self
                .graph
                .reconstruct_provenance_artifact_without_retained_fast_path(node)
                .ok()?,
            (_, Some(fact)) => fact,
            _ => return None,
        };
        fact.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
        self.graph.record_retained_forensic_read();
        self.graph.record_retained_artifact_read();
        Some(fact)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph.reconstruct_provenance_artifact(node)
    }

    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.graph.materialize_explanation_artifact(node)
    }

    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), SignalError> {
        self.graph.materialize_provenance_artifact(node)
    }
}
