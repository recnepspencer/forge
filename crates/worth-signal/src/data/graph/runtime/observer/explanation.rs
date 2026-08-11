use super::GraphObserver;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::trace::{
    ColdArtifactRecord, RetainedDiagnosticArtifact, RuntimeArtifactHot, RuntimeArtifactState,
    RuntimeArtifactWarm,
};
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::logic::explain::{dependency_chain_to, explain, NodeExplanation};
use crate::presentation::dot::to_dot;

impl<'a> GraphObserver<'a> {
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        explain(self.graph, node)
    }

    pub(crate) fn runtime_artifact_state(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RuntimeArtifactState>, SignalError> {
        self.graph.node_runtime_artifact_state(node)
    }

    pub fn runtime_artifact_hot(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RuntimeArtifactHot>, SignalError> {
        self.graph.node_runtime_artifact_hot(node)
    }

    pub fn runtime_artifact_warm(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RuntimeArtifactWarm>, SignalError> {
        self.graph.node_runtime_artifact_warm(node)
    }

    pub fn retained_diagnostic_artifact(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RetainedDiagnosticArtifact>, SignalError> {
        self.graph.node_retained_diagnostic_artifact(node)
    }

    pub fn cold_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a ColdArtifactRecord>, SignalError> {
        self.graph.node_cold_artifact_record(node)
    }

    pub fn dependency_chain_to(
        &self,
        root: NodeId,
        target: NodeId,
    ) -> Result<Option<Vec<NodeId>>, SignalError> {
        dependency_chain_to(self.graph, root, target)
    }

    pub fn explanation_fact(&self, node: NodeId) -> Option<&'a ExplanationFact> {
        self.graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node)
    }

    pub fn provenance_fact(&self, node: NodeId) -> Option<&'a ProvenanceFact> {
        self.graph
            .observation
            .diagnostics
            .provenance_facts()
            .get(&node)
    }

    pub fn to_dot(&self) -> String {
        to_dot(self.graph)
    }
}
