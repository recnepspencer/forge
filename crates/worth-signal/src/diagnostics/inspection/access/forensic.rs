use crate::data::handle::NodeId;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::logic::explain::NodeExplanation;

use super::GraphForensicDiagnostics;

impl<'a> GraphForensicDiagnostics<'a> {
    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph
            .observe()
            .materialize()
            .retained_explanation_artifact(node)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, crate::data::error::SignalError> {
        self.graph
            .observe()
            .materialize()
            .reconstruct_explanation_artifact(node)
    }

    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), crate::data::error::SignalError>
    {
        self.graph
            .observe()
            .materialize()
            .materialize_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph
            .observe()
            .materialize()
            .retained_provenance_artifact(node)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, crate::data::error::SignalError> {
        self.graph
            .observe()
            .materialize()
            .reconstruct_provenance_artifact(node)
    }

    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), crate::data::error::SignalError>
    {
        self.graph
            .observe()
            .materialize()
            .materialize_provenance_artifact(node)
    }
}
