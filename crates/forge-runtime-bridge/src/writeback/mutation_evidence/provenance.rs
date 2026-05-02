use std::sync::Arc;

use super::super::{
    BridgeDerivedWritebackEffect, BridgeWritebackAuthorityOutcome, BridgeWritebackCausalityBasis,
    BridgeWritebackExecutionRecord, BridgeWritebackFailureClass, BridgeWritebackFeedbackProvenance,
    BridgeWritebackOutcomeClass,
};

/// Bridge-owned causality breadcrumbs for one authoritative mutation crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMutationCausalityBundle {
    causality_digest: Arc<str>,
    truth_trigger_digest: Arc<str>,
    route_digest: Arc<str>,
    evaluation_surface_digest: Arc<str>,
    truth_view_digest: Arc<str>,
}

impl BridgeMutationCausalityBundle {
    pub fn from_writeback_causality(causality: &BridgeWritebackCausalityBasis) -> Self {
        Self {
            causality_digest: Arc::from(causality.digest().to_owned()),
            truth_trigger_digest: Arc::from(causality.truth_trigger_digest().to_owned()),
            route_digest: Arc::from(causality.route_digest().to_owned()),
            evaluation_surface_digest: Arc::from(causality.evaluation_surface_digest().to_owned()),
            truth_view_digest: Arc::from(causality.truth_view_digest().to_owned()),
        }
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn truth_trigger_digest(&self) -> &str {
        self.truth_trigger_digest.as_ref()
    }

    pub fn route_digest(&self) -> &str {
        self.route_digest.as_ref()
    }

    pub fn evaluation_surface_digest(&self) -> &str {
        self.evaluation_surface_digest.as_ref()
    }

    pub fn truth_view_digest(&self) -> &str {
        self.truth_view_digest.as_ref()
    }
}

/// Bridge-owned provenance breadcrumbs for one authoritative mutation crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMutationProvenanceBundle {
    contract_digest: Arc<str>,
    derived_effect_digest: Arc<str>,
    proposed_effect_digest: Arc<str>,
    feedback_provenance_digest: Arc<str>,
    causality_digest: Arc<str>,
    strategy_descriptor_digest: Arc<str>,
    execution_record_digest: Arc<str>,
    outcome_class: Option<BridgeWritebackOutcomeClass>,
    authoritative_artifact_digest: Option<Arc<str>>,
    request_digest: Option<Arc<str>>,
    receipt_digest: Option<Arc<str>>,
    failure_class: Option<BridgeWritebackFailureClass>,
}

impl BridgeMutationProvenanceBundle {
    pub fn from_writeback_artifacts(
        effect: &BridgeDerivedWritebackEffect,
        feedback: &BridgeWritebackFeedbackProvenance,
        execution_record: &BridgeWritebackExecutionRecord,
        outcome: Option<&BridgeWritebackAuthorityOutcome>,
    ) -> Self {
        Self {
            contract_digest: Arc::from(execution_record.contract_digest().to_owned()),
            derived_effect_digest: Arc::from(execution_record.derived_effect_digest().to_owned()),
            proposed_effect_digest: Arc::from(execution_record.proposed_effect_digest().to_owned()),
            feedback_provenance_digest: Arc::from(feedback.digest().to_owned()),
            causality_digest: Arc::from(execution_record.causality_digest().to_owned()),
            strategy_descriptor_digest: Arc::from(effect.strategy_descriptor_digest().to_owned()),
            execution_record_digest: Arc::from(execution_record.digest().to_owned()),
            outcome_class: execution_record.outcome_class(),
            authoritative_artifact_digest: outcome
                .map(|value| Arc::from(value.authoritative_artifact_digest().to_owned())),
            request_digest: execution_record
                .request_digest()
                .map(|value| Arc::from(value.to_owned())),
            receipt_digest: execution_record
                .receipt_digest()
                .map(|value| Arc::from(value.to_owned())),
            failure_class: execution_record.failure_class(),
        }
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn derived_effect_digest(&self) -> &str {
        self.derived_effect_digest.as_ref()
    }

    pub fn proposed_effect_digest(&self) -> &str {
        self.proposed_effect_digest.as_ref()
    }

    pub fn feedback_provenance_digest(&self) -> &str {
        self.feedback_provenance_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn execution_record_digest(&self) -> &str {
        self.execution_record_digest.as_ref()
    }

    pub fn outcome_class(&self) -> Option<BridgeWritebackOutcomeClass> {
        self.outcome_class
    }

    pub fn authoritative_artifact_digest(&self) -> Option<&str> {
        self.authoritative_artifact_digest.as_deref()
    }

    pub fn request_digest(&self) -> Option<&str> {
        self.request_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }

    pub fn failure_class(&self) -> Option<BridgeWritebackFailureClass> {
        self.failure_class
    }
}
