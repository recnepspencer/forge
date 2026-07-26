use std::sync::Arc;

use crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding;

pub struct WorthQueryRetainedConvergenceCandidateEvidence {
    occurrence_identity: Arc<str>,
    state_identity: Arc<str>,
    report_evidence_identity: Arc<str>,
    domain_evidence: WorthQueryDomainEvidenceExecutionBinding,
}

impl WorthQueryRetainedConvergenceCandidateEvidence {
    pub(super) fn new(
        occurrence_identity: impl Into<Arc<str>>,
        state_identity: impl Into<Arc<str>>,
        report_evidence_identity: impl Into<Arc<str>>,
        domain_evidence: WorthQueryDomainEvidenceExecutionBinding,
    ) -> Self {
        Self {
            occurrence_identity: occurrence_identity.into(),
            state_identity: state_identity.into(),
            report_evidence_identity: report_evidence_identity.into(),
            domain_evidence,
        }
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.occurrence_identity
    }

    pub fn state_identity(&self) -> &str {
        &self.state_identity
    }

    pub fn report_evidence_identity(&self) -> &str {
        &self.report_evidence_identity
    }

    pub fn domain_evidence(&self) -> &WorthQueryDomainEvidenceExecutionBinding {
        &self.domain_evidence
    }
}
