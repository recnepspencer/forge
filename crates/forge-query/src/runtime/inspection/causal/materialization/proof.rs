use crate::identity::hash_parts;

use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalBridgeReadmissionProof {
    query_admission_digest: String,
    anchor_digest: String,
    bridge_admission_summary_digest: String,
    bridge_envelope_digest: String,
    readmission_proof_digest: String,
}

impl CausalBridgeReadmissionProof {
    pub(super) fn from_readmitted_bridge_envelope(
        query_admission_digest: &str,
        anchor_digest: &str,
        envelope: &BridgeCausalExplanationEnvelope,
    ) -> Self {
        let bridge_admission_summary_digest = envelope.admission_summary_digest().to_string();
        let bridge_envelope_digest = envelope.envelope_digest().to_string();
        let readmission_proof_digest = hash_parts(&[
            "causal_bridge_readmission_proof_v1".to_string(),
            format!("query-admission:{query_admission_digest}"),
            format!("anchor:{anchor_digest}"),
            format!("bridge-summary:{bridge_admission_summary_digest}"),
            format!("bridge-envelope:{bridge_envelope_digest}"),
        ]);
        Self {
            query_admission_digest: query_admission_digest.to_string(),
            anchor_digest: anchor_digest.to_string(),
            bridge_admission_summary_digest,
            bridge_envelope_digest,
            readmission_proof_digest,
        }
    }

    pub fn query_admission_digest(&self) -> &str {
        &self.query_admission_digest
    }

    pub fn anchor_digest(&self) -> &str {
        &self.anchor_digest
    }

    pub fn bridge_admission_summary_digest(&self) -> &str {
        &self.bridge_admission_summary_digest
    }

    pub fn bridge_envelope_digest(&self) -> &str {
        &self.bridge_envelope_digest
    }

    pub fn readmission_proof_digest(&self) -> &str {
        &self.readmission_proof_digest
    }
}
