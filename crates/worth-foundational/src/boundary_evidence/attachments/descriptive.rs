use crate::diagnostics::FoundationalDiagnosticLocator;

use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::super::receipts::FoundationalBoundaryEvidenceCompletedReceiptArtifact;
use super::super::support::{
    FoundationalBoundaryEvidencePublishedSupportArtifact,
    FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceSupportAttachment {
    Published(FoundationalBoundaryEvidencePublishedSupportArtifact),
    Closeout(FoundationalBoundaryEvidenceSupportCloseoutArtifact),
    TransientLifecycle(FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact),
}

impl FoundationalBoundaryEvidenceSupportAttachment {
    pub(crate) fn canonical_fragment(&self) -> String {
        match self {
            Self::Published(artifact) => {
                format!("support:published:{:?}", artifact.support_truth_kind())
            }
            Self::Closeout(artifact) => {
                format!("support:closeout:{:?}", artifact.support_truth_kind())
            }
            Self::TransientLifecycle(artifact) => {
                format!("support:transient:{:?}", artifact.support_truth_kind())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceDiagnosticAttachment {
    SupportReport(FoundationalDiagnosticLocator),
    ExplanationBundle(FoundationalDiagnosticLocator),
}

impl FoundationalBoundaryEvidenceDiagnosticAttachment {
    pub(crate) fn canonical_fragment(&self) -> String {
        match self {
            Self::SupportReport(locator) => {
                format!("diagnostic:support:{}", locator.canonical_key_fragment())
            }
            Self::ExplanationBundle(locator) => {
                format!(
                    "diagnostic:explanation:{}",
                    locator.canonical_key_fragment()
                )
            }
        }
    }
}

pub(crate) fn canonical_fragment_for_provenance_attachment(
    provenance: &FoundationalBoundaryEvidenceProvenanceArtifact,
) -> String {
    format!(
        "provenance:{:?}:{:?}:source={:?}:authority={}:strategy={}:profile={}:comparison={}:digest={}:support_context={}",
        provenance.locality(),
        provenance.freshness_posture(),
        provenance.source_basis().kind(),
        provenance.authority_path().is_some(),
        provenance.strategy_basis().is_some(),
        provenance.profile_basis().is_some(),
        provenance.comparison_basis().is_some(),
        provenance.canonical_digest_basis().is_some(),
        provenance.support_context_attachments().len()
    )
}

pub(crate) fn canonical_fragment_for_receipt_attachment(
    receipt: &FoundationalBoundaryEvidenceCompletedReceiptArtifact,
) -> String {
    format!(
        "receipt:{:?}:{:?}:executed={}",
        receipt.receipt_kind(),
        receipt.closeout_disposition(),
        receipt.did_execute()
    )
}
