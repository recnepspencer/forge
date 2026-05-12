use crate::identity::hash_parts;

use forge_runtime_bridge::facade::{BridgeCausalEvidenceBinding, BridgeCausalExplanationEnvelope};

use super::denied_artifact::DeniedQueryCausalInspectionArtifact;
use super::{
    BuiltBridgeBackedArtifact, CausalBridgeReadmissionProof, CausalInspectionArtifactKind,
    CausalInspectionBoundaryEnvelopeCategory, CausalInspectionPerformanceEnvelope,
    CausalInspectionRedactionPolicy, CausalMaterializationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCausalEvidenceReferenceArtifact {
    owner: String,
    family: String,
    reference_identity: String,
    binding_digest: String,
    retained_record_digest: Option<String>,
    detail_redacted: bool,
    reference_digest: String,
}

impl QueryCausalEvidenceReferenceArtifact {
    pub(super) fn from_bridge_binding(
        binding: &BridgeCausalEvidenceBinding,
        redaction_policy: CausalInspectionRedactionPolicy,
    ) -> Self {
        let detail_redacted = redaction_policy == CausalInspectionRedactionPolicy::DigestOnly;
        let retained_record_digest = if detail_redacted {
            None
        } else {
            binding.retained_record_digest().map(str::to_string)
        };
        let reference_digest = hash_parts(&[
            "query_causal_evidence_reference_artifact_v1".to_string(),
            format!("owner:{}", binding.owner().as_str()),
            format!("family:{}", binding.family().as_str()),
            format!("reference:{}", binding.reference_identity()),
            format!("binding:{}", binding.binding_digest()),
            format!(
                "retained:{}",
                retained_record_digest.as_deref().unwrap_or("redacted")
            ),
            format!("redacted:{detail_redacted}"),
        ]);
        Self {
            owner: binding.owner().as_str().to_string(),
            family: binding.family().as_str().to_string(),
            reference_identity: binding.reference_identity().to_string(),
            binding_digest: binding.binding_digest().to_string(),
            retained_record_digest,
            detail_redacted,
            reference_digest,
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn reference_identity(&self) -> &str {
        &self.reference_identity
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn retained_record_digest(&self) -> Option<&str> {
        self.retained_record_digest.as_deref()
    }

    pub fn detail_redacted(&self) -> bool {
        self.detail_redacted
    }

    pub fn reference_digest(&self) -> &str {
        &self.reference_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedQueryCausalInspectionArtifact {
    query_admission_digest: String,
    query_observation_digest: String,
    result_shape_context_digest: String,
    bridge_envelope_identity_digest: String,
    bridge_envelope_digest: String,
    bridge_receipt_digest: String,
    boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    evidence_references: Vec<QueryCausalEvidenceReferenceArtifact>,
    performance: CausalInspectionPerformanceEnvelope,
    receipt: CausalMaterializationReceipt,
    readmission_proof: CausalBridgeReadmissionProof,
    causal_identity_digest: String,
    artifact_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryQueryCausalInspectionArtifact {
    query_advisory_digest: String,
    query_observation_digest: String,
    result_shape_context_digest: String,
    advisory_reason: String,
    bridge_envelope_identity_digest: String,
    bridge_envelope_digest: String,
    bridge_receipt_digest: String,
    boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    evidence_references: Vec<QueryCausalEvidenceReferenceArtifact>,
    performance: CausalInspectionPerformanceEnvelope,
    receipt: CausalMaterializationReceipt,
    readmission_proof: CausalBridgeReadmissionProof,
    causal_identity_digest: String,
    artifact_digest: String,
}

impl AdmittedQueryCausalInspectionArtifact {
    pub(super) fn from_parts(
        query_admission_digest: &str,
        query_observation_digest: &str,
        result_shape_context_digest: &str,
        envelope: &BridgeCausalExplanationEnvelope,
        built: BuiltBridgeBackedArtifact,
    ) -> Self {
        Self {
            query_admission_digest: query_admission_digest.to_string(),
            query_observation_digest: query_observation_digest.to_string(),
            result_shape_context_digest: result_shape_context_digest.to_string(),
            bridge_envelope_identity_digest: envelope.identity().identity_digest().to_string(),
            bridge_envelope_digest: envelope.envelope_digest().to_string(),
            bridge_receipt_digest: envelope.receipt().receipt_digest().to_string(),
            boundary_categories: built.boundary_categories,
            evidence_references: built.evidence_references,
            performance: built.performance,
            receipt: built.receipt,
            readmission_proof: built.readmission_proof,
            causal_identity_digest: built.causal_identity_digest,
            artifact_digest: built.artifact_digest,
        }
    }

    pub fn query_admission_digest(&self) -> &str {
        &self.query_admission_digest
    }

    pub fn query_observation_digest(&self) -> &str {
        &self.query_observation_digest
    }

    pub fn result_shape_context_digest(&self) -> &str {
        &self.result_shape_context_digest
    }

    pub fn bridge_envelope_identity_digest(&self) -> &str {
        &self.bridge_envelope_identity_digest
    }
}

impl AdvisoryQueryCausalInspectionArtifact {
    pub(super) fn from_parts(
        query_advisory_digest: &str,
        query_observation_digest: &str,
        result_shape_context_digest: &str,
        advisory_reason: String,
        envelope: &BridgeCausalExplanationEnvelope,
        built: BuiltBridgeBackedArtifact,
    ) -> Self {
        Self {
            query_advisory_digest: query_advisory_digest.to_string(),
            query_observation_digest: query_observation_digest.to_string(),
            result_shape_context_digest: result_shape_context_digest.to_string(),
            advisory_reason,
            bridge_envelope_identity_digest: envelope.identity().identity_digest().to_string(),
            bridge_envelope_digest: envelope.envelope_digest().to_string(),
            bridge_receipt_digest: envelope.receipt().receipt_digest().to_string(),
            boundary_categories: built.boundary_categories,
            evidence_references: built.evidence_references,
            performance: built.performance,
            receipt: built.receipt,
            readmission_proof: built.readmission_proof,
            causal_identity_digest: built.causal_identity_digest,
            artifact_digest: built.artifact_digest,
        }
    }

    pub fn query_advisory_digest(&self) -> &str {
        &self.query_advisory_digest
    }

    pub fn query_observation_digest(&self) -> &str {
        &self.query_observation_digest
    }

    pub fn result_shape_context_digest(&self) -> &str {
        &self.result_shape_context_digest
    }

    pub fn advisory_reason(&self) -> &str {
        &self.advisory_reason
    }

    pub fn bridge_envelope_identity_digest(&self) -> &str {
        &self.bridge_envelope_identity_digest
    }
}

macro_rules! bridge_backed_accessors {
    ($ty:ty) => {
        impl $ty {
            pub fn evidence_references(&self) -> &[QueryCausalEvidenceReferenceArtifact] {
                &self.evidence_references
            }

            pub fn boundary_categories(&self) -> &[CausalInspectionBoundaryEnvelopeCategory] {
                &self.boundary_categories
            }

            pub fn bridge_envelope_digest(&self) -> &str {
                &self.bridge_envelope_digest
            }

            pub fn bridge_receipt_digest(&self) -> &str {
                &self.bridge_receipt_digest
            }

            pub fn performance(&self) -> &CausalInspectionPerformanceEnvelope {
                &self.performance
            }

            pub fn receipt(&self) -> &CausalMaterializationReceipt {
                &self.receipt
            }

            pub fn readmission_proof(&self) -> &CausalBridgeReadmissionProof {
                &self.readmission_proof
            }

            pub fn bridge_readmission_proof_digest(&self) -> &str {
                self.readmission_proof.readmission_proof_digest()
            }

            pub fn causal_identity_digest(&self) -> &str {
                &self.causal_identity_digest
            }

            pub fn artifact_digest(&self) -> &str {
                &self.artifact_digest
            }
        }
    };
}

bridge_backed_accessors!(AdmittedQueryCausalInspectionArtifact);
bridge_backed_accessors!(AdvisoryQueryCausalInspectionArtifact);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryCausalInspectionArtifact {
    Admitted(AdmittedQueryCausalInspectionArtifact),
    Advisory(AdvisoryQueryCausalInspectionArtifact),
    Denied(DeniedQueryCausalInspectionArtifact),
}

impl QueryCausalInspectionArtifact {
    pub fn kind(&self) -> CausalInspectionArtifactKind {
        match self {
            Self::Admitted(_) => CausalInspectionArtifactKind::Admitted,
            Self::Advisory(_) => CausalInspectionArtifactKind::Advisory,
            Self::Denied(_) => CausalInspectionArtifactKind::Denied,
        }
    }

    pub fn artifact_digest(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.artifact_digest(),
            Self::Advisory(artifact) => artifact.artifact_digest(),
            Self::Denied(artifact) => artifact.artifact_digest(),
        }
    }

    pub fn causal_identity_digest(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.causal_identity_digest(),
            Self::Advisory(artifact) => artifact.causal_identity_digest(),
            Self::Denied(artifact) => artifact.causal_identity_digest(),
        }
    }

    pub fn query_observation_digest(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.query_observation_digest(),
            Self::Advisory(artifact) => artifact.query_observation_digest(),
            Self::Denied(artifact) => artifact.query_observation_digest(),
        }
    }

    pub fn bridge_envelope_digest(&self) -> Option<&str> {
        match self {
            Self::Admitted(artifact) => Some(artifact.bridge_envelope_digest()),
            Self::Advisory(artifact) => Some(artifact.bridge_envelope_digest()),
            Self::Denied(_) => None,
        }
    }

    pub fn evidence_reference_count(&self) -> usize {
        match self {
            Self::Admitted(artifact) => artifact.evidence_references().len(),
            Self::Advisory(artifact) => artifact.evidence_references().len(),
            Self::Denied(_) => 0,
        }
    }

    pub fn performance(&self) -> &CausalInspectionPerformanceEnvelope {
        match self {
            Self::Admitted(artifact) => artifact.performance(),
            Self::Advisory(artifact) => artifact.performance(),
            Self::Denied(artifact) => artifact.performance(),
        }
    }

    pub fn bridge_readmission_proof_digest(&self) -> Option<&str> {
        match self {
            Self::Admitted(artifact) => Some(artifact.bridge_readmission_proof_digest()),
            Self::Advisory(artifact) => Some(artifact.bridge_readmission_proof_digest()),
            Self::Denied(_) => None,
        }
    }

    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted(_))
    }

    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied(_))
    }
}
