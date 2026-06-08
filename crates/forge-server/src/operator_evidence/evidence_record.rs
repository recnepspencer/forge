use forge_foundational::facade::{
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceSupportAttachment, FoundationalBoundaryEvidenceSupportTruthKind,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};

use super::{
    classification::ForgeServerOperatorEvidenceClass,
    counter_receipt::ForgeServerOperatorCounterReceipt, transform::ForgeServerEvidenceTransform,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeServerOperatorEvidenceRecord {
    transform: ForgeServerEvidenceTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    response_digest: String,
    classification: ForgeServerOperatorEvidenceClass,
    counter_receipt: ForgeServerOperatorCounterReceipt,
    attachment_bundle: FoundationalBoundaryEvidenceAttachmentBundle,
    materialized_attachment_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
}

impl ForgeServerOperatorEvidenceRecord {
    pub(crate) fn new(
        transform: ForgeServerEvidenceTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        response_digest: String,
        classification: ForgeServerOperatorEvidenceClass,
        counter_receipt: ForgeServerOperatorCounterReceipt,
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

    pub fn transform(&self) -> ForgeServerEvidenceTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn response_digest(&self) -> &str {
        &self.response_digest
    }

    pub fn classification(&self) -> &ForgeServerOperatorEvidenceClass {
        &self.classification
    }

    pub fn counter_receipt(&self) -> &ForgeServerOperatorCounterReceipt {
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
