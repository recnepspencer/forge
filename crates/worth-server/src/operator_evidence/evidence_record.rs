use worth_foundational::facade::{
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceSupportAttachment, FoundationalBoundaryEvidenceSupportTruthKind,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};

use super::{
    classification::WorthServerOperatorEvidenceClass,
    counter_receipt::WorthServerOperatorCounterReceipt, transform::WorthServerEvidenceTransform,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthServerOperatorEvidenceRecord {
    transform: WorthServerEvidenceTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    response_digest: String,
    classification: WorthServerOperatorEvidenceClass,
    counter_receipt: WorthServerOperatorCounterReceipt,
    attachment_bundle: FoundationalBoundaryEvidenceAttachmentBundle,
    materialized_attachment_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
}

impl WorthServerOperatorEvidenceRecord {
    pub(crate) fn new(
        transform: WorthServerEvidenceTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        response_digest: String,
        classification: WorthServerOperatorEvidenceClass,
        counter_receipt: WorthServerOperatorCounterReceipt,
        attachment_bundle: FoundationalBoundaryEvidenceAttachmentBundle,
        materialized_attachment_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    ) -> Self {
        Self {
            transform,
            diagnostics_profile,
            response_digest,
            classification,
            counter_receipt,
            attachment_bundle,
            materialized_attachment_bundle,
        }
    }

    pub fn transform(&self) -> WorthServerEvidenceTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn response_digest(&self) -> &str {
        &self.response_digest
    }

    pub fn classification(&self) -> &WorthServerOperatorEvidenceClass {
        &self.classification
    }

    pub fn counter_receipt(&self) -> &WorthServerOperatorCounterReceipt {
        &self.counter_receipt
    }

    pub fn attachment_bundle(&self) -> &FoundationalBoundaryEvidenceAttachmentBundle {
        &self.attachment_bundle
    }

    pub fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        match self
            .attachment_bundle
            .support()
            .expect("operator evidence records always carry support truth")
        {
            FoundationalBoundaryEvidenceSupportAttachment::Published(artifact) => {
                artifact.support_truth_kind()
            }
            FoundationalBoundaryEvidenceSupportAttachment::Closeout(artifact) => {
                artifact.support_truth_kind()
            }
            FoundationalBoundaryEvidenceSupportAttachment::TransientLifecycle(artifact) => {
                artifact.support_truth_kind()
            }
        }
    }

    pub fn materialized_attachment_bundle(
        &self,
    ) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        &self.materialized_attachment_bundle
    }
}
