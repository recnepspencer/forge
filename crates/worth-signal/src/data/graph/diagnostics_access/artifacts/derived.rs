use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::logic::explain::{explain, NodeExplanation};

impl SignalGraph {
    pub(crate) fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.record_explicit_cold_materialization_request();
        if let Some(fact) = self.explanation_fact(node) {
            let mut explanation = fact.explanation.clone();
            explanation.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
            self.record_retained_artifact_read();
            return Ok((
                Some(explanation),
                DiagnosticsAvailability::RetainedAvailable,
            ));
        }

        match self.explanation_reconstruction_availability() {
            DiagnosticsAvailability::ReconstructedAvailable => Ok((
                Some(self.reconstruct_explanation_artifact(node)?),
                DiagnosticsAvailability::ReconstructedAvailable,
            )),
            DiagnosticsAvailability::OmittedByTier => {
                self.record_denied_reconstruction_by_tier(true);
                Ok((None, DiagnosticsAvailability::OmittedByTier))
            }
            DiagnosticsAvailability::DeniedByBudget => {
                self.record_denied_reconstruction_by_budget(true);
                Ok((None, DiagnosticsAvailability::DeniedByBudget))
            }
            availability => Ok((None, availability)),
        }
    }

    pub(crate) fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), SignalError> {
        self.record_explicit_cold_materialization_request();
        if let Some(fact) = self.provenance_fact(node) {
            let mut fact = fact.clone();
            fact.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
            self.record_retained_artifact_read();
            return Ok((Some(fact), DiagnosticsAvailability::RetainedAvailable));
        }

        match self.provenance_reconstruction_availability() {
            DiagnosticsAvailability::ReconstructedAvailable => Ok((
                Some(self.reconstruct_provenance_artifact(node)?),
                DiagnosticsAvailability::ReconstructedAvailable,
            )),
            DiagnosticsAvailability::OmittedByTier => {
                self.record_denied_reconstruction_by_tier(false);
                Ok((None, DiagnosticsAvailability::OmittedByTier))
            }
            DiagnosticsAvailability::DeniedByBudget => {
                self.record_denied_reconstruction_by_budget(false);
                Ok((None, DiagnosticsAvailability::DeniedByBudget))
            }
            availability => Ok((None, availability)),
        }
    }

    pub(crate) fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_explanation_reconstruction();
        Ok(explanation)
    }

    pub(crate) fn reconstruct_explanation_artifact_without_retained_fast_path(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        let mut explanation = crate::logic::explain::explain_reconstructing_with_policy_resolver(
            self, node, &resolver,
        )?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_explanation_reconstruction();
        Ok(explanation)
    }

    pub(crate) fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_provenance_reconstruction();
        Ok(ProvenanceFact::from_explanation(&explanation))
    }

    pub(crate) fn reconstruct_provenance_artifact_without_retained_fast_path(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        let mut explanation = crate::logic::explain::explain_reconstructing_with_policy_resolver(
            self, node, &resolver,
        )?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_provenance_reconstruction();
        Ok(ProvenanceFact::from_explanation(&explanation))
    }
}
