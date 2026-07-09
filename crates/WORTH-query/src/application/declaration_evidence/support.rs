use worth_foundational::facade::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceSupportAttachment,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportFrontDoor,
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
    FoundationalBoundaryEvidenceSupportResidualDebtSet,
};
use worth_proof::TransitionOutcome;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    class::WorthQueryDeclarationFoundationalEvidenceClass,
    denial::WorthQueryDeclarationFoundationalEvidenceDenial,
    receipt::build_support_publication_receipt,
    subject::WorthQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) fn build_support_attachment<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    completed_receipt: Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
) -> Result<
    FoundationalBoundaryEvidenceSupportAttachment,
    WorthQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    let class = subject.class();
    match class {
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
            let receipt = build_support_publication_receipt(subject, provenance);
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .published_evidence()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
                )
                .attested_by(receipt)
            {
                TransitionOutcome::Success(artifact) => Ok(
                    FoundationalBoundaryEvidenceSupportAttachment::Published(artifact),
                ),
                TransitionOutcome::Denied(denial) => Err(
                    WorthQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected support publication outcome: {outcome:?}"),
            }
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
            let receipt = build_support_publication_receipt(subject, provenance);
            let debt = FoundationalBoundaryEvidenceSupportResidualDebtSet::new(vec![
                FoundationalBoundaryEvidenceSupportResidualDebtKind::RebuildRequired,
            ])
            .expect("non-empty residual debt set should admit");
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .residual_debt_statement()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
                )
                .with_residual_debt(debt)
                .attested_by(receipt)
            {
                TransitionOutcome::Success(artifact) => Ok(
                    FoundationalBoundaryEvidenceSupportAttachment::Published(artifact),
                ),
                TransitionOutcome::Denied(denial) => Err(
                    WorthQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected deferred support outcome: {outcome:?}"),
            }
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale => {
            let receipt = build_support_publication_receipt(subject, provenance);
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .stale_basis_disclosure()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis,
                )
                .attested_by(receipt)
            {
                TransitionOutcome::Success(artifact) => Ok(
                    FoundationalBoundaryEvidenceSupportAttachment::Published(artifact),
                ),
                TransitionOutcome::Denied(denial) => Err(
                    WorthQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected stale support outcome: {outcome:?}"),
            }
        }
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
            let receipt = completed_receipt
                .cloned()
                .expect("closeout support requires a completed primary receipt");
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .degraded_recovery_report()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
                )
                .closed_out_by(receipt)
            {
                TransitionOutcome::Success(artifact) => Ok(
                    FoundationalBoundaryEvidenceSupportAttachment::Closeout(artifact),
                ),
                TransitionOutcome::Denied(denial) => Err(
                    WorthQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected degraded support outcome: {outcome:?}"),
            }
        }
    }
}
