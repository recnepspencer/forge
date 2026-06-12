use crate::evidence_identity::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceTag};

use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use super::super::identity::{
    compose_bridge_causal_envelope_identity, CausalInspectionOutcomeIdentity,
};
use super::super::observation_identity::CausalObservationAnchorDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalBridgeReadmissionProof {
    query_admission_identity: CausalInspectionOutcomeIdentity,
    anchor_identity: CausalObservationAnchorDigest,
    bridge_admission_summary_digest: String,
    bridge_envelope_identity: ForgeQueryEvidenceIdentity,
    readmission_proof_identity: ForgeQueryEvidenceIdentity,
}

impl CausalBridgeReadmissionProof {
    pub(super) fn from_readmitted_bridge_envelope(
        query_admission_identity: &CausalInspectionOutcomeIdentity,
        anchor_identity: &CausalObservationAnchorDigest,
        envelope: &BridgeCausalExplanationEnvelope,
    ) -> Self {
        let bridge_admission_summary_digest = envelope.admission_summary_digest().to_string();
        let bridge_envelope_identity = compose_bridge_causal_envelope_identity(envelope.identity());
        let readmission_proof_identity = ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::CausalInspectionArtifact,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "bridge-readmission-proof",
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("query_admission"),
            query_admission_identity.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("anchor"),
            anchor_identity.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("bridge_summary"),
            &bridge_admission_summary_digest,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_envelope"),
            &bridge_envelope_identity,
        )
        .seal();
        Self {
            query_admission_identity: query_admission_identity.clone(),
            anchor_identity: anchor_identity.clone(),
            bridge_admission_summary_digest,
            bridge_envelope_identity,
            readmission_proof_identity,
        }
    }

    pub fn query_admission_digest(&self) -> &str {
        self.query_admission_identity.as_str()
    }

    pub fn anchor_digest(&self) -> &str {
        self.anchor_identity.as_str()
    }

    pub fn bridge_admission_summary_digest(&self) -> &str {
        &self.bridge_admission_summary_digest
    }

    pub fn bridge_envelope_digest(&self) -> &str {
        self.bridge_envelope_identity.as_str()
    }

    pub fn readmission_proof_digest(&self) -> &str {
        self.readmission_proof_identity.as_str()
    }

    pub(in crate::runtime) fn readmission_proof_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.readmission_proof_identity
    }
}
