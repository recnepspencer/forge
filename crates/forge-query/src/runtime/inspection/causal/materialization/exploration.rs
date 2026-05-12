use super::{
    CausalInspectionArtifactKind, CausalInspectionBoundaryEnvelopeCategory,
    CausalInspectionPerformanceEnvelope, CausalMaterializationReceipt,
    QueryCausalEvidenceReferenceArtifact, QueryCausalInspectionArtifact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalInspectionArtifactDecisionTrace<'a> {
    query_decision_digest: &'a str,
    bridge_envelope_digest: Option<&'a str>,
    bridge_denial_digest: Option<&'a str>,
}

impl<'a> CausalInspectionArtifactDecisionTrace<'a> {
    pub fn query_decision_digest(&self) -> &'a str {
        self.query_decision_digest
    }

    pub fn bridge_envelope_digest(&self) -> Option<&'a str> {
        self.bridge_envelope_digest
    }

    pub fn bridge_denial_digest(&self) -> Option<&'a str> {
        self.bridge_denial_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalInspectionArtifactIntegrity<'a> {
    artifact_digest: &'a str,
    causal_identity_digest: &'a str,
    bridge_readmission_proof_digest: Option<&'a str>,
}

impl<'a> CausalInspectionArtifactIntegrity<'a> {
    pub fn artifact_digest(&self) -> &'a str {
        self.artifact_digest
    }

    pub fn causal_identity_digest(&self) -> &'a str {
        self.causal_identity_digest
    }

    pub fn bridge_readmission_proof_digest(&self) -> Option<&'a str> {
        self.bridge_readmission_proof_digest
    }
}

impl QueryCausalInspectionArtifact {
    pub fn primary_result(&self) -> CausalInspectionArtifactKind {
        self.kind()
    }

    pub fn warnings(&self) -> Vec<&str> {
        match self {
            Self::Admitted(_) => Vec::new(),
            Self::Advisory(artifact) => vec![artifact.advisory_reason()],
            Self::Denied(artifact) => vec![artifact.denial_reason()],
        }
    }

    pub fn decision_trace(&self) -> CausalInspectionArtifactDecisionTrace<'_> {
        match self {
            Self::Admitted(artifact) => CausalInspectionArtifactDecisionTrace {
                query_decision_digest: artifact.query_admission_digest(),
                bridge_envelope_digest: Some(artifact.bridge_envelope_digest()),
                bridge_denial_digest: None,
            },
            Self::Advisory(artifact) => CausalInspectionArtifactDecisionTrace {
                query_decision_digest: artifact.query_advisory_digest(),
                bridge_envelope_digest: Some(artifact.bridge_envelope_digest()),
                bridge_denial_digest: None,
            },
            Self::Denied(artifact) => CausalInspectionArtifactDecisionTrace {
                query_decision_digest: artifact.query_denial_digest(),
                bridge_envelope_digest: None,
                bridge_denial_digest: artifact.bridge_denial_digest(),
            },
        }
    }

    pub fn authority_bindings(&self) -> &[QueryCausalEvidenceReferenceArtifact] {
        self.evidence()
    }

    pub fn evidence(&self) -> &[QueryCausalEvidenceReferenceArtifact] {
        match self {
            Self::Admitted(artifact) => artifact.evidence_references(),
            Self::Advisory(artifact) => artifact.evidence_references(),
            Self::Denied(_) => &[],
        }
    }

    pub fn integrity(&self) -> CausalInspectionArtifactIntegrity<'_> {
        CausalInspectionArtifactIntegrity {
            artifact_digest: self.artifact_digest(),
            causal_identity_digest: self.causal_identity_digest(),
            bridge_readmission_proof_digest: self.bridge_readmission_proof_digest(),
        }
    }

    pub fn receipt(&self) -> &CausalMaterializationReceipt {
        match self {
            Self::Admitted(artifact) => artifact.receipt(),
            Self::Advisory(artifact) => artifact.receipt(),
            Self::Denied(artifact) => artifact.receipt(),
        }
    }

    pub fn denial_reason(&self) -> Option<&str> {
        match self {
            Self::Denied(artifact) => Some(artifact.denial_reason()),
            Self::Admitted(_) | Self::Advisory(_) => None,
        }
    }

    pub fn advisory_reason(&self) -> Option<&str> {
        match self {
            Self::Advisory(artifact) => Some(artifact.advisory_reason()),
            Self::Admitted(_) | Self::Denied(_) => None,
        }
    }

    pub fn boundary_categories(&self) -> &[CausalInspectionBoundaryEnvelopeCategory] {
        match self {
            Self::Admitted(artifact) => artifact.boundary_categories(),
            Self::Advisory(artifact) => artifact.boundary_categories(),
            Self::Denied(artifact) => artifact.boundary_categories(),
        }
    }

    pub fn is_advisory(&self) -> bool {
        matches!(self, Self::Advisory(_))
    }

    pub fn performance_envelope(&self) -> &CausalInspectionPerformanceEnvelope {
        self.performance()
    }
}
