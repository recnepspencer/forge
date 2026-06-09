use forge_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
};
use forge_proof::TransitionOutcome;

pub(crate) fn build_provenance(
    artifact_family: &str,
    canonical_digest: &str,
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(boundary_artifact_id(&[
                "forge-server.response.provenance".to_string(),
                artifact_family.to_string(),
                canonical_digest.to_string(),
            ])),
            BoundaryArtifactField::Basis,
        ));
    match FoundationalBoundaryEvidenceProvenanceFrontDoor
        .current(source_basis)
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => provenance,
        outcome => panic!("response provenance construction should be admitted: {outcome:?}"),
    }
}

pub(crate) fn boundary_artifact_id(parts: &[String]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x1f;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
