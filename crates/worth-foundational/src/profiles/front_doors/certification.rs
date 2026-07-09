use super::super::{
    bridge_evidence_backed_proof_bearing_artifact_trust_boundary,
    bridge_production_certified_proof_bearing_artifact_trust_boundary,
    certify_evidence_backed_proof_bearing_artifact,
    certify_production_certified_proof_bearing_artifact,
    foundational_profile_certification_authority,
    foundational_profile_certification_readmission_authority,
    readmit_evidence_backed_proof_bearing_artifact_after_boundary,
    readmit_production_certified_proof_bearing_artifact_after_boundary,
    BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact,
    BoundaryBridgedProductionCertifiedProofBearingArtifact,
    EvidenceBackedCertifiedProofBearingArtifact, FoundationalProfileCertificationOutcome,
    ProductionCertifiedProofBearingArtifact, ProofBearingProfiledArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalProfileCertificationFrontDoor;

impl FoundationalProfileCertificationFrontDoor {
    pub fn evidence_backed<T>(
        self,
        artifact: ProofBearingProfiledArtifact<T>,
    ) -> FoundationalProfileCertificationOutcome<EvidenceBackedCertifiedProofBearingArtifact<T>>
    {
        certify_evidence_backed_proof_bearing_artifact(
            artifact,
            foundational_profile_certification_authority(),
        )
    }

    pub fn production_certified<T>(
        self,
        artifact: EvidenceBackedCertifiedProofBearingArtifact<T>,
    ) -> FoundationalProfileCertificationOutcome<ProductionCertifiedProofBearingArtifact<T>> {
        certify_production_certified_proof_bearing_artifact(
            artifact,
            foundational_profile_certification_authority(),
        )
    }

    pub fn bridge_evidence_backed<T>(
        self,
        artifact: EvidenceBackedCertifiedProofBearingArtifact<T>,
    ) -> BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T> {
        bridge_evidence_backed_proof_bearing_artifact_trust_boundary(artifact)
    }

    pub fn readmit_evidence_backed<T>(
        self,
        artifact: BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T>,
    ) -> EvidenceBackedCertifiedProofBearingArtifact<T> {
        readmit_evidence_backed_proof_bearing_artifact_after_boundary(
            artifact,
            foundational_profile_certification_readmission_authority(),
        )
    }

    pub fn bridge_production_certified<T>(
        self,
        artifact: ProductionCertifiedProofBearingArtifact<T>,
    ) -> BoundaryBridgedProductionCertifiedProofBearingArtifact<T> {
        bridge_production_certified_proof_bearing_artifact_trust_boundary(artifact)
    }

    pub fn readmit_production_certified<T>(
        self,
        artifact: BoundaryBridgedProductionCertifiedProofBearingArtifact<T>,
    ) -> ProductionCertifiedProofBearingArtifact<T> {
        readmit_production_certified_proof_bearing_artifact_after_boundary(
            artifact,
            foundational_profile_certification_readmission_authority(),
        )
    }
}
