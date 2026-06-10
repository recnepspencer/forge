use crate::ForgeServerResponseEnvelope;

use super::{ForgeServerExternalCounterSet, ForgeServerExternalEvidenceRecord};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityCertificationBundle {
    support_posture_label: String,
    policy_digest: String,
    provenance_digest: String,
    response_digest: String,
    external_counters: ForgeServerExternalCounterSet,
    operator_evidence_record: ForgeServerExternalEvidenceRecord,
    canonical_digest: String,
}

impl ForgeServerCompatibilityCertificationBundle {
    pub(crate) fn new(
        support_posture_label: String,
        policy_digest: String,
        provenance_digest: String,
        response: &ForgeServerResponseEnvelope,
        external_counters: ForgeServerExternalCounterSet,
        operator_evidence_record: ForgeServerExternalEvidenceRecord,
    ) -> Self {
        let response_digest = response.canonical_digest().to_string();
        let canonical_digest = format!(
            "forge-server-compat-certification-v1|support={support_posture_label}|policy={policy_digest}|provenance={provenance_digest}|response={response_digest}|external={}|evidence={}",
            external_counters.canonical_digest(),
            operator_evidence_record.canonical_digest(),
        );
        Self {
            support_posture_label,
            policy_digest,
            provenance_digest,
            response_digest,
            external_counters,
            operator_evidence_record,
            canonical_digest,
        }
    }

    pub fn support_posture_label(&self) -> &str {
        &self.support_posture_label
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }

    pub fn response_digest(&self) -> &str {
        &self.response_digest
    }

    pub fn external_counters(&self) -> &ForgeServerExternalCounterSet {
        &self.external_counters
    }

    pub fn operator_evidence_record(&self) -> &ForgeServerExternalEvidenceRecord {
        &self.operator_evidence_record
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
