use forge_foundational::facade::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceSupportAttachment,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportFrontDoor,
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
    FoundationalBoundaryEvidenceSupportResidualDebtSet,
};
use forge_proof::TransitionOutcome;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{
    class::ForgeQueryDeclarationFoundationalEvidenceClass,
    denial::ForgeQueryDeclarationFoundationalEvidenceDenial,
    receipt::build_support_publication_receipt,
    subject::ForgeQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) fn build_support_attachment<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: &ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    completed_receipt: Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
) -> Result<
    FoundationalBoundaryEvidenceSupportAttachment,
    ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    let class = subject.class();
    match class {
        ForgeQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted
        | ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
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
                    ForgeQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected support publication outcome: {outcome:?}"),
            }
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
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
                    ForgeQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected deferred support outcome: {outcome:?}"),
            }
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionStale => {
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
                    ForgeQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected stale support outcome: {outcome:?}"),
            }
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::LegalityDenied
        | ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDenied
        | ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
        | ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
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
                    ForgeQueryDeclarationFoundationalEvidenceDenial::support(class, denial),
                ),
                outcome => panic!("unexpected degraded support outcome: {outcome:?}"),
            }
        }
    }
}
