use worth_foundational::facade::CanonicalDerivedDigest;

use super::query_references::HadwigerQueryDeclarationReference;

mod artifact_kinds;

pub use artifact_kinds::HadwigerArtifactKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerArtifactShapeError {
    EmptyField { field: &'static str },
    DuplicateVertex { vertex_label: String },
    MissingEdgeEndpoint { vertex_label: String },
    SelfEdge { vertex_label: String },
    EmptyParentArtifacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerArtifactAuthorityOwner {
    QueryDeclaration,
    HadwigerArtifactBuilder,
    Checker,
    AIAdvisory,
    AgentAdvisory,
    ProofCandidate,
    TheoremAuthority,
}

impl HadwigerArtifactAuthorityOwner {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDeclaration => "query_declaration",
            Self::HadwigerArtifactBuilder => "hadwiger_artifact_builder",
            Self::Checker => "checker",
            Self::AIAdvisory => "ai_advisory",
            Self::AgentAdvisory => "agent_advisory",
            Self::ProofCandidate => "proof_candidate",
            Self::TheoremAuthority => "theorem_authority",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerArtifactSourceReference {
    QueryDeclaration(HadwigerQueryDeclarationReference),
    ArtifactConstruction {
        operation: String,
    },
    CheckerBoundary {
        checker_identity: String,
        checker_version: String,
    },
    AIAdvisory {
        advisory_source_digest: String,
    },
    AgentAdvisory {
        source_digest: String,
    },
}

impl HadwigerArtifactSourceReference {
    pub fn stable_token(&self) -> String {
        match self {
            Self::QueryDeclaration(reference) => {
                format!("query_declaration:{}", reference.stable_token())
            }
            Self::ArtifactConstruction { operation } => {
                format!("artifact_construction:{operation}")
            }
            Self::CheckerBoundary {
                checker_identity,
                checker_version,
            } => {
                format!("checker_boundary:{checker_identity}:{checker_version}")
            }
            Self::AIAdvisory {
                advisory_source_digest,
            } => {
                format!("ai_advisory:{advisory_source_digest}")
            }
            Self::AgentAdvisory { source_digest } => {
                format!("agent_advisory:{source_digest}")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerArtifactDigest {
    canonical: CanonicalDerivedDigest,
    stable_token: String,
}

impl HadwigerArtifactDigest {
    pub(crate) fn from_canonical(canonical: CanonicalDerivedDigest) -> Self {
        let stable_token = canonical_digest_token(&canonical);
        Self {
            canonical,
            stable_token,
        }
    }

    pub fn canonical(&self) -> &CanonicalDerivedDigest {
        &self.canonical
    }

    pub fn stable_token(&self) -> &str {
        &self.stable_token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerArtifactReference {
    artifact_kind: HadwigerArtifactKind,
    artifact_digest: HadwigerArtifactDigest,
}

impl HadwigerArtifactReference {
    pub(crate) fn new(
        artifact_kind: HadwigerArtifactKind,
        artifact_digest: HadwigerArtifactDigest,
    ) -> Self {
        Self {
            artifact_kind,
            artifact_digest,
        }
    }

    pub fn artifact_kind(&self) -> HadwigerArtifactKind {
        self.artifact_kind
    }

    pub fn artifact_digest(&self) -> &HadwigerArtifactDigest {
        &self.artifact_digest
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}",
            self.artifact_kind.as_str(),
            self.artifact_digest.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HadwigerArtifactCore {
    artifact_kind: HadwigerArtifactKind,
    artifact_digest: HadwigerArtifactDigest,
    authority_owner: HadwigerArtifactAuthorityOwner,
    source_reference: HadwigerArtifactSourceReference,
    parent_artifacts: Vec<HadwigerArtifactReference>,
}

impl HadwigerArtifactCore {
    pub(crate) fn new(
        artifact_kind: HadwigerArtifactKind,
        artifact_digest: HadwigerArtifactDigest,
        authority_owner: HadwigerArtifactAuthorityOwner,
        source_reference: HadwigerArtifactSourceReference,
        parent_artifacts: Vec<HadwigerArtifactReference>,
    ) -> Self {
        Self {
            artifact_kind,
            artifact_digest,
            authority_owner,
            source_reference,
            parent_artifacts,
        }
    }

    pub(crate) fn reference(&self) -> HadwigerArtifactReference {
        HadwigerArtifactReference::new(self.artifact_kind, self.artifact_digest.clone())
    }

    pub(crate) fn artifact_kind(&self) -> HadwigerArtifactKind {
        self.artifact_kind
    }

    pub(crate) fn artifact_digest(&self) -> &HadwigerArtifactDigest {
        &self.artifact_digest
    }

    pub(crate) fn authority_owner(&self) -> HadwigerArtifactAuthorityOwner {
        self.authority_owner
    }

    pub(crate) fn source_reference(&self) -> &HadwigerArtifactSourceReference {
        &self.source_reference
    }

    pub(crate) fn parent_artifacts(&self) -> &[HadwigerArtifactReference] {
        &self.parent_artifacts
    }
}

pub trait HadwigerCanonicalArtifact {
    fn artifact_kind(&self) -> HadwigerArtifactKind;
    fn artifact_digest(&self) -> &HadwigerArtifactDigest;
    fn authority_owner(&self) -> HadwigerArtifactAuthorityOwner;
    fn source_reference(&self) -> &HadwigerArtifactSourceReference;
    fn parent_artifacts(&self) -> &[HadwigerArtifactReference];
    fn reference(&self) -> HadwigerArtifactReference;
}

pub(crate) fn require_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerArtifactShapeError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(HadwigerArtifactShapeError::EmptyField { field })
    } else {
        Ok(value)
    }
}

pub(crate) fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) use super::canonical_artifact_impl::impl_hadwiger_artifact;
