use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::explanations::HadwigerReusableNegativeEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureScope {
    Artifact {
        artifact: HadwigerArtifactReference,
    },
    EdgeLocal {
        graph_version: HadwigerArtifactReference,
        left_vertex: String,
        right_vertex: String,
    },
}

impl FailureScope {
    pub fn artifact(artifact: HadwigerArtifactReference) -> Self {
        Self::Artifact { artifact }
    }

    pub fn edge_local(
        graph_version: HadwigerArtifactReference,
        left_vertex: impl Into<String>,
        right_vertex: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let left_vertex = require_non_empty(left_vertex, "left_vertex")?;
        let right_vertex = require_non_empty(right_vertex, "right_vertex")?;
        if left_vertex == right_vertex {
            return Err(HadwigerArtifactShapeError::SelfEdge {
                vertex_label: left_vertex,
            });
        }
        let (left_vertex, right_vertex) = if left_vertex <= right_vertex {
            (left_vertex, right_vertex)
        } else {
            (right_vertex, left_vertex)
        };
        Ok(Self::EdgeLocal {
            graph_version,
            left_vertex,
            right_vertex,
        })
    }

    pub fn stable_token(&self) -> String {
        match self {
            Self::Artifact { artifact } => format!("artifact_scope:{}", artifact.stable_token()),
            Self::EdgeLocal {
                graph_version,
                left_vertex,
                right_vertex,
            } => format!(
                "edge_local:{}:{left_vertex}:{right_vertex}",
                graph_version.stable_token()
            ),
        }
    }

    pub fn affected_artifact(&self) -> HadwigerArtifactReference {
        match self {
            Self::Artifact { artifact } => artifact.clone(),
            Self::EdgeLocal { graph_version, .. } => graph_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureBasisFingerprint {
    core: HadwigerArtifactCore,
    failure_basis: String,
    scope_token: String,
    evidence_digest_token: String,
}

impl FailureBasisFingerprint {
    pub(crate) fn from_negative_evidence(
        evidence: &HadwigerReusableNegativeEvidence,
        scope: &FailureScope,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let failure_basis = require_non_empty(evidence.failure_basis(), "failure_basis")?;
        let scope_token = require_non_empty(scope.stable_token(), "failure_scope")?;
        let evidence_digest_token = evidence.artifact_digest().stable_token().to_string();
        let core = artifact_core(
            HadwigerArtifactKind::FailureBasisFingerprint,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "failure_basis_fingerprint".to_string(),
            },
            vec![evidence.reference(), scope.affected_artifact()],
            vec![
                HadwigerArtifactPayloadEntry::text("failure_basis", failure_basis.clone()),
                HadwigerArtifactPayloadEntry::text("scope_token", scope_token.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "evidence_digest_token",
                    evidence_digest_token.clone(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            failure_basis,
            scope_token,
            evidence_digest_token,
        })
    }

    pub fn failure_basis(&self) -> &str {
        &self.failure_basis
    }

    pub fn scope_token(&self) -> &str {
        &self.scope_token
    }

    pub fn evidence_digest_token(&self) -> &str {
        &self.evidence_digest_token
    }
}

impl_hadwiger_artifact!(FailureBasisFingerprint, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphResidentFailure {
    core: HadwigerArtifactCore,
    failure_scope: FailureScope,
    failure_basis_fingerprint: FailureBasisFingerprint,
    reactivation_hint: String,
}

impl GraphResidentFailure {
    pub(crate) fn from_negative_evidence(
        evidence: &HadwigerReusableNegativeEvidence,
        failure_scope: FailureScope,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let failure_basis_fingerprint =
            FailureBasisFingerprint::from_negative_evidence(evidence, &failure_scope)?;
        let reactivation_hint = require_non_empty(
            evidence.reactivation_or_repair_hint(),
            "reactivation_or_repair_hint",
        )?;
        let core = artifact_core(
            HadwigerArtifactKind::GraphResidentFailure,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "graph_resident_failure".to_string(),
            },
            vec![
                evidence.reference(),
                failure_scope.affected_artifact(),
                failure_basis_fingerprint.reference(),
            ],
            vec![
                HadwigerArtifactPayloadEntry::text("failure_scope", failure_scope.stable_token()),
                HadwigerArtifactPayloadEntry::text(
                    "failure_basis_fingerprint",
                    failure_basis_fingerprint.artifact_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text("reactivation_hint", reactivation_hint.clone()),
            ],
        )?;
        Ok(Self {
            core,
            failure_scope,
            failure_basis_fingerprint,
            reactivation_hint,
        })
    }

    pub fn failure_scope(&self) -> &FailureScope {
        &self.failure_scope
    }

    pub fn failure_basis_fingerprint(&self) -> &FailureBasisFingerprint {
        &self.failure_basis_fingerprint
    }

    pub fn reactivation_hint(&self) -> &str {
        &self.reactivation_hint
    }

    pub fn stable_token(&self) -> String {
        self.reference().stable_token()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(GraphResidentFailure, core);
