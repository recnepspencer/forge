use forge_runtime_bridge::facade::{
    BridgeMutationAuthorityBundle, BridgeWritebackFailureClass, BridgeWritebackOutcomeClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationProvenanceEvidence {
    contract_digest: String,
    derived_effect_digest: String,
    proposed_effect_digest: String,
    feedback_provenance_digest: String,
    causality_digest: String,
    strategy_descriptor_digest: String,
    execution_record_digest: String,
    outcome_class: Option<BridgeWritebackOutcomeClass>,
    authoritative_artifact_digest: Option<String>,
    request_digest: Option<String>,
    receipt_digest: Option<String>,
    failure_class: Option<BridgeWritebackFailureClass>,
}

impl ForgeQueryMutationProvenanceEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeMutationAuthorityBundle) -> Self {
        let provenance = bundle.provenance();
        Self {
            contract_digest: provenance.contract_digest().to_string(),
            derived_effect_digest: provenance.derived_effect_digest().to_string(),
            proposed_effect_digest: provenance.proposed_effect_digest().to_string(),
            feedback_provenance_digest: provenance.feedback_provenance_digest().to_string(),
            causality_digest: provenance.causality_digest().to_string(),
            strategy_descriptor_digest: provenance.strategy_descriptor_digest().to_string(),
            execution_record_digest: provenance.execution_record_digest().to_string(),
            outcome_class: provenance.outcome_class(),
            authoritative_artifact_digest: provenance
                .authoritative_artifact_digest()
                .map(str::to_string),
            request_digest: provenance.request_digest().map(str::to_string),
            receipt_digest: provenance.receipt_digest().map(str::to_string),
            failure_class: provenance.failure_class(),
        }
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn derived_effect_digest(&self) -> &str {
        &self.derived_effect_digest
    }

    pub fn proposed_effect_digest(&self) -> &str {
        &self.proposed_effect_digest
    }

    pub fn feedback_provenance_digest(&self) -> &str {
        &self.feedback_provenance_digest
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn execution_record_digest(&self) -> &str {
        &self.execution_record_digest
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

    #[cfg(test)]
    pub(crate) fn test_only(execution_record_digest: impl Into<String>) -> Self {
        Self {
            contract_digest: "contract:test".to_string(),
            derived_effect_digest: "derived:test".to_string(),
            proposed_effect_digest: "proposed:test".to_string(),
            feedback_provenance_digest: "feedback:test".to_string(),
            causality_digest: "causality:test".to_string(),
            strategy_descriptor_digest: "strategy:test".to_string(),
            execution_record_digest: execution_record_digest.into(),
            outcome_class: None,
            authoritative_artifact_digest: None,
            request_digest: None,
            receipt_digest: None,
            failure_class: None,
        }
    }
}
