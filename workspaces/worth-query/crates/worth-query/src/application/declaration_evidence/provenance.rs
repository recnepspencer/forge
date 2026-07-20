use sha2::{Digest, Sha256};
use worth_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceCanonicalDigestBasis, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
};
use worth_proof::TransitionOutcome;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    class::WorthQueryDeclarationFoundationalEvidenceClass,
    denial::WorthQueryDeclarationFoundationalEvidenceDenial,
    subject::WorthQueryDeclarationFoundationalEvidenceInput,
};

pub(crate) fn build_provenance<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
) -> Result<
    FoundationalBoundaryEvidenceProvenanceArtifact,
    WorthQueryDeclarationFoundationalEvidenceDenial<D, I>,
> {
    let class = subject.class();
    let step = FoundationalBoundaryEvidenceProvenanceFrontDoor
        .current(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
            source_locator(subject),
        ))
        .canonical_digest_basis(FoundationalBoundaryEvidenceCanonicalDigestBasis::digest(
            subject.canonical_declaration().declaration_digest().clone(),
        ));

    match step.with_freshness(freshness_for(class)) {
        TransitionOutcome::Success(provenance) => Ok(provenance),
        TransitionOutcome::Denied(denial) => Err(
            WorthQueryDeclarationFoundationalEvidenceDenial::provenance(class, denial),
        ),
        outcome => panic!("unexpected provenance outcome: {outcome:?}"),
    }
}

pub(crate) fn source_locator<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        boundary_artifact_id(&[
            "worth-query.declaration-evidence".to_string(),
            format!("class:{:?}", subject.class()),
            format!("declaration:{}", subject.declaration_digest_string()),
            format!("family:{}", subject.declaration_family_key()),
        ]),
        BoundaryArtifactField::Basis,
    )
}

pub(crate) fn target_locator<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationFoundationalEvidenceInput<D, I>,
) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        boundary_artifact_id(&[
            "worth-query.declaration-evidence".to_string(),
            format!("class:{:?}", subject.class()),
            format!("declaration:{}", subject.declaration_digest_string()),
            format!("family:{}", subject.declaration_family_key()),
        ]),
        BoundaryArtifactField::Proofs,
    )
}

pub(crate) fn boundary_artifact_id(parts: &[String]) -> BoundaryArtifactId {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    BoundaryArtifactId::new(u64::from_be_bytes(bytes))
}

fn freshness_for(
    class: WorthQueryDeclarationFoundationalEvidenceClass,
) -> FoundationalBoundaryEvidenceFreshnessPosture {
    match class {
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale => {
            FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained
        }
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted
        | WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
        | WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => {
            FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
        }
    }
}
