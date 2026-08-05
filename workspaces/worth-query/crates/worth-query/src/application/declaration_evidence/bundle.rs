use worth_foundational::facade::{
    derive_boundary_evidence_attachment_bundle_digest,
    prepare_boundary_evidence_attachment_bundle_for_canonical_basis, CanonicalDerivedDigest,
    FoundationalBoundaryEvidenceAttachmentFrontDoor,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};
use worth_proof::TransitionOutcome;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    denial::WorthQueryDeclarationFoundationalEvidenceDenial, provenance::target_locator,
    subject::WorthQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) fn build_bundle<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    planning_receipt: Option<
        worth_foundational::facade::FoundationalBoundaryEvidencePlanningReceiptArtifact,
    >,
    completed_receipt: Option<
        worth_foundational::facade::FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    >,
    support_attachment: worth_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> Result<
    (
        FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
        CanonicalDerivedDigest,
    ),
    WorthQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    let class = subject.class();
    let mut bundle = FoundationalBoundaryEvidenceAttachmentFrontDoor
        .for_boundary_artifact(target_locator(subject))
        .with_provenance_attachment(provenance);
    if let Some(receipt) = completed_receipt.clone() {
        bundle = bundle.with_receipt_attachment(receipt);
    }
    bundle = match support_attachment.clone() {
        worth_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::Published(artifact) => {
            if completed_receipt.is_none() {
                bundle = bundle.with_receipt_attachment(
                    artifact.support_publication_receipt().completed_receipt().clone(),
                );
            }
            bundle.with_published_support(artifact)
        }
        worth_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::Closeout(artifact) => {
            bundle.with_support_closeout(artifact)
        }
        worth_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::TransientLifecycle(artifact) => {
            bundle.with_transient_lifecycle_support(artifact)
        }
    };
    let materialized = bundle.materialize_under(profile);
    let version = subject
        .canonical_declaration()
        .version()
        .foundational()
        .clone();
    match prepare_boundary_evidence_attachment_bundle_for_canonical_basis(
        version.clone(),
        &materialized,
    ) {
        TransitionOutcome::Success(_) => {}
        TransitionOutcome::Denied(denial) => {
            return Err(
                WorthQueryDeclarationFoundationalEvidenceDenial::attachment_canonical_basis(
                    class, denial,
                ),
            )
        }
        outcome => panic!("unexpected bundle basis outcome: {outcome:?}"),
    }
    let digest = match derive_boundary_evidence_attachment_bundle_digest(
        version,
        &materialized,
        worth_foundational::facade::CanonicalDigestAlgorithmId::sha256(),
    ) {
        TransitionOutcome::Success(digest) => digest,
        TransitionOutcome::Denied(denial) => {
            return Err(
                WorthQueryDeclarationFoundationalEvidenceDenial::attachment_digest(class, denial),
            )
        }
        outcome => panic!("unexpected bundle digest outcome: {outcome:?}"),
    };
    let _ = planning_receipt;
    Ok((materialized, digest))
}
