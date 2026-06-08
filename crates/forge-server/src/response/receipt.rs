use forge_foundational::facade::{
    claim_receipt_evidence_boundary_surface, BoundaryHandle, EquivalenceBasisId,
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceReceiptFrontDoor,
    FoundationalBoundaryReceiptSurface, FoundationalCommitId, FoundationalCommitParentBasis,
    FoundationalCommitParentageLocator, FoundationalTransitionLocator,
};

use super::provenance::boundary_artifact_id;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ForgeServerResponseReceipt {
    Executed(FoundationalBoundaryEvidenceExecutedReceiptArtifact),
    Completed(FoundationalBoundaryEvidenceCompletedReceiptArtifact),
}

impl ForgeServerResponseReceipt {
    pub fn executed(&self) -> Option<&FoundationalBoundaryEvidenceExecutedReceiptArtifact> {
        match self {
            Self::Executed(receipt) => Some(receipt),
            Self::Completed(_) => None,
        }
    }

    pub fn completed(&self) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        match self {
            Self::Executed(receipt) => receipt.completed_receipt(),
            Self::Completed(receipt) => receipt,
        }
    }
}

pub(crate) fn build_success_receipt(
    boundary_label: &str,
    canonical_digest: &str,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
) -> ForgeServerResponseReceipt {
    let boundary = receipt_boundary("success", canonical_digest);
    let surface = FoundationalBoundaryReceiptSurface::new(boundary_label, 1)
        .expect("response success receipts should carry completed boundary text");
    let _claim = claim_receipt_evidence_boundary_surface(surface);
    ForgeServerResponseReceipt::Executed(
        FoundationalBoundaryEvidenceReceiptFrontDoor
            .publication(boundary)
            .with_provenance(provenance),
    )
}

pub(crate) fn build_denial_receipt(
    boundary_label: &str,
    canonical_digest: &str,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
) -> ForgeServerResponseReceipt {
    let boundary = receipt_boundary("denial", canonical_digest);
    let surface = FoundationalBoundaryReceiptSurface::new(boundary_label, 1)
        .expect("response denial receipts should carry completed boundary text");
    let _claim = claim_receipt_evidence_boundary_surface(surface);
    ForgeServerResponseReceipt::Completed(
        FoundationalBoundaryEvidenceReceiptFrontDoor
            .denied_closeout(boundary)
            .with_provenance(provenance),
    )
}

fn receipt_boundary(
    boundary_family: &str,
    canonical_digest: &str,
) -> FoundationalBoundaryEvidenceReceiptBoundary {
    let commit_id = FoundationalCommitId::new(BoundaryHandle::new(boundary_artifact_id(&[
        "forge-server.response.receipt.commit".to_string(),
        boundary_family.to_string(),
        canonical_digest.to_string(),
    ])));
    let parent_basis =
        FoundationalCommitParentBasis::new(EquivalenceBasisId::new(boundary_artifact_id(&[
            "forge-server.response.receipt.parent".to_string(),
            boundary_family.to_string(),
            canonical_digest.to_string(),
        ])));
    FoundationalBoundaryEvidenceReceiptBoundary::transition(
        FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
            commit_id,
            parent_basis,
        )),
    )
}
