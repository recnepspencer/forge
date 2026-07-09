use worth_foundational::facade::{
    BoundaryHandle, EquivalenceBasisId, FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidencePlanningReceiptArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceReceiptFrontDoor,
    FoundationalBranchId, FoundationalCommitId, FoundationalCommitParentBasis,
    FoundationalCommitParentageLocator, FoundationalTransitionLocator,
};

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    class::WorthQueryDeclarationFoundationalEvidenceClass, provenance::boundary_artifact_id,
    subject::WorthQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) enum WorthQueryDeclarationFoundationalPrimaryReceipt {
    Planning(FoundationalBoundaryEvidencePlanningReceiptArtifact),
    Completed(FoundationalBoundaryEvidenceCompletedReceiptArtifact),
}

impl WorthQueryDeclarationFoundationalPrimaryReceipt {
    pub(crate) fn planning(&self) -> Option<&FoundationalBoundaryEvidencePlanningReceiptArtifact> {
        match self {
            Self::Planning(receipt) => Some(receipt),
            Self::Completed(_) => None,
        }
    }

    pub(crate) fn completed(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact> {
        match self {
            Self::Planning(_) => None,
            Self::Completed(receipt) => Some(receipt),
        }
    }
}

pub(crate) fn build_primary_receipt<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
) -> WorthQueryDeclarationFoundationalPrimaryReceipt {
    let boundary = receipt_boundary(subject);
    match subject.class() {
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted => {
            WorthQueryDeclarationFoundationalPrimaryReceipt::Planning(
                FoundationalBoundaryEvidenceReceiptFrontDoor
                    .planning(boundary)
                    .with_provenance(provenance),
            )
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
            WorthQueryDeclarationFoundationalPrimaryReceipt::Completed(
                FoundationalBoundaryEvidenceReceiptFrontDoor
                    .admission(boundary)
                    .with_provenance(provenance)
                    .completed_receipt()
                    .clone(),
            )
        }
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied => {
            WorthQueryDeclarationFoundationalPrimaryReceipt::Completed(
                FoundationalBoundaryEvidenceReceiptFrontDoor
                    .denied_closeout(boundary)
                    .with_provenance(provenance),
            )
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
            WorthQueryDeclarationFoundationalPrimaryReceipt::Completed(
                FoundationalBoundaryEvidenceReceiptFrontDoor
                    .blocked_closeout(boundary)
                    .with_provenance(provenance),
            )
        }
    }
}

pub(crate) fn build_support_publication_receipt<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
) -> worth_foundational::facade::FoundationalBoundaryEvidenceExecutedReceiptArtifact {
    FoundationalBoundaryEvidenceReceiptFrontDoor
        .support_publication(receipt_boundary(subject))
        .with_provenance(provenance)
}

fn receipt_boundary<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
) -> FoundationalBoundaryEvidenceReceiptBoundary {
    let branch_id = FoundationalBranchId::new(format!(
        "worth-query.declaration.{}",
        normalize_fragment(subject.declaration_family_key())
    ))
    .expect("static declaration evidence branch ids should be valid");
    let commit_id = FoundationalCommitId::new(BoundaryHandle::new(
        boundary_artifact_id(&[
            format!("receipt.commit:{:?}", subject.class()),
            format!("declaration:{}", subject.declaration_digest_string()),
        ])
        .get(),
    ));
    let parent_basis = FoundationalCommitParentBasis::new(EquivalenceBasisId::new(
        boundary_artifact_id(&[
            format!("receipt.parent:{:?}", subject.class()),
            format!("support:{}", subject.support_digest()),
            format!("legality:{:?}", subject.legality_digest()),
        ])
        .get(),
    ));
    let locator = FoundationalTransitionLocator::CommitParentage(
        FoundationalCommitParentageLocator::new(commit_id, parent_basis),
    );
    let _ = branch_id;
    FoundationalBoundaryEvidenceReceiptBoundary::transition(locator)
}

fn normalize_fragment(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            normalized.push(ch);
        } else if !normalized.ends_with('-') {
            normalized.push('-');
        }
    }
    normalized.trim_matches('-').to_string()
}
