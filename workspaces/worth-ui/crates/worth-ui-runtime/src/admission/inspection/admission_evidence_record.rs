use crate::admission::UiAdmissionReport;
use crate::evidence::{
    evidence_authority_binding, evidence_handle, evidence_identity, evidence_ref,
    UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind, UiEvidenceFamily, UiEvidenceIdentity,
    UiEvidenceMaterializationPosture, UiEvidenceRef, UiEvidenceRetentionPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct UiAdmissionEvidenceRecord {
    identity: UiEvidenceIdentity,
    identity_digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
}

impl UiAdmissionEvidenceRecord {
    pub(crate) fn for_report(report: &UiAdmissionReport) -> Self {
        Self {
            identity: evidence_identity(UiEvidenceFamily::Admission, report.identity_digest()),
            identity_digest: report.identity_digest(),
            authority_generation: report.authority_generation(),
        }
    }

    pub(crate) fn reference(&self) -> UiEvidenceRef {
        let authority_binding = evidence_authority_binding(
            UiEvidenceAuthorityKind::AdmissionReport,
            self.identity_digest,
            self.authority_generation,
            None,
        );
        let handle = evidence_handle(
            UiEvidenceFamily::Admission,
            self.identity,
            self.identity_digest,
        );

        evidence_ref(
            UiEvidenceFamily::Admission,
            self.identity,
            authority_binding,
            UiEvidenceMaterializationPosture::RefsOnly,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            handle,
        )
    }
}
