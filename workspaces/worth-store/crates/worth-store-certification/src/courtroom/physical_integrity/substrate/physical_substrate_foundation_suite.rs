use crate::{PhysicalFoundationEvidenceBundle, PhysicalSubstrateCertificationDenial};
use worth_store_contracts::{StableArtifactId, StableDigest};
use worth_store_readiness::{FoundationalVocabularyAdoptionMap, PhysicalFoundationEvidenceField};

pub(crate) fn foundation_bundle(
) -> Result<PhysicalFoundationEvidenceBundle, PhysicalSubstrateCertificationDenial> {
    let adoption = FoundationalVocabularyAdoptionMap::physical_format_all_public_lanes()
        .map_err(|_| PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected)?;
    let adoption_digest = adoption.proof_vocabulary().digest().clone();
    let mut builder = PhysicalFoundationEvidenceBundle::builder(adoption)
        .with_canonical_artifact_digest(
            StableArtifactId::new(PhysicalFoundationEvidenceField::ArtifactDigest.as_str())
                .map_err(|_| PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected)?,
            adoption_digest,
        );
    for field in PhysicalFoundationEvidenceField::required_for_physical_format() {
        if field != PhysicalFoundationEvidenceField::ArtifactDigest {
            builder = builder
                .with_report_evidence(
                    field,
                    StableArtifactId::new(field.as_str()).map_err(|_| {
                        PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected
                    })?,
                    digest(field.as_str())?,
                )
                .map_err(|_| PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected)?;
        }
    }
    builder
        .admit_without_byte_authority_promotion()
        .map_err(|_| PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected)
}

fn digest(name: &str) -> Result<StableDigest, PhysicalSubstrateCertificationDenial> {
    StableDigest::new(format!("sha256:{name}"))
        .map_err(|_| PhysicalSubstrateCertificationDenial::FoundationEvidenceRejected)
}
