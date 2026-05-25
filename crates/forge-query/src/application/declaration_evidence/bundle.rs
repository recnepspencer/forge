use forge_foundational::facade::{
    derive_boundary_evidence_attachment_bundle_digest,
    prepare_boundary_evidence_attachment_bundle_for_canonical_basis, CanonicalDerivedDigest,
    FoundationalBoundaryEvidenceAttachmentFrontDoor,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};
use forge_proof::TransitionOutcome;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{
    denial::ForgeQueryDeclarationFoundationalEvidenceDenial, provenance::target_locator,
    subject::ForgeQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) fn build_bundle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    subject: &ForgeQueryDeclarationFoundationalEvidenceInput<D, I>,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    planning_receipt: Option<
        forge_foundational::facade::FoundationalBoundaryEvidencePlanningReceiptArtifact,
    >,
    completed_receipt: Option<
        forge_foundational::facade::FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    >,
    support_attachment: forge_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> Result<
    (
        FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
        CanonicalDerivedDigest,
    ),
    ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    let class = subject.class();
    let mut bundle = FoundationalBoundaryEvidenceAttachmentFrontDoor
        .for_boundary_artifact(target_locator(subject))
        .with_provenance_attachment(provenance);
    if let Some(receipt) = completed_receipt.clone() {
        bundle = bundle.with_receipt_attachment(receipt);
    }
    bundle = match support_attachment.clone() {
        forge_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::Published(artifact) => {
            if completed_receipt.is_none() {
                bundle = bundle.with_receipt_attachment(
                    artifact.support_publication_receipt().completed_receipt().clone(),
                );
            }
            bundle.with_published_support(artifact)
        }
        forge_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::Closeout(artifact) => {
            bundle.with_support_closeout(artifact)
        }
        forge_foundational::facade::FoundationalBoundaryEvidenceSupportAttachment::TransientLifecycle(artifact) => {
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
                ForgeQueryDeclarationFoundationalEvidenceDenial::attachment_canonical_basis(
                    class, denial,
                ),
            )
        }
        outcome => panic!("unexpected bundle basis outcome: {outcome:?}"),
    }
    let digest = match derive_boundary_evidence_attachment_bundle_digest(
        version,
        &materialized,
        forge_foundational::facade::CanonicalDigestAlgorithmId::test_stable_fixture(),
    ) {
        TransitionOutcome::Success(digest) => digest,
        TransitionOutcome::Denied(denial) => {
            return Err(
                ForgeQueryDeclarationFoundationalEvidenceDenial::attachment_digest(class, denial),
            )
        }
        outcome => panic!("unexpected bundle digest outcome: {outcome:?}"),
    };
    let _ = planning_receipt;
    Ok((materialized, digest))
}
