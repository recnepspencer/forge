use super::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use super::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use super::query_references::HadwigerQueryDeclarationReference;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerCheckerCausalEvidence {
    truth_view_basis_digest: String,
    route_identity: String,
    evaluation_identity: String,
    diagnostics_digest: String,
    replay_digest: String,
}

impl HadwigerCheckerCausalEvidence {
    pub fn new(
        truth_view_basis_digest: impl Into<String>,
        route_identity: impl Into<String>,
        evaluation_identity: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        replay_digest: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            truth_view_basis_digest: require_non_empty(
                truth_view_basis_digest,
                "truth_view_basis_digest",
            )?,
            route_identity: require_non_empty(route_identity, "route_identity")?,
            evaluation_identity: require_non_empty(evaluation_identity, "evaluation_identity")?,
            diagnostics_digest: require_non_empty(diagnostics_digest, "diagnostics_digest")?,
            replay_digest: require_non_empty(replay_digest, "replay_digest")?,
        })
    }

    pub(crate) fn payload_entries(&self) -> Vec<HadwigerArtifactPayloadEntry> {
        vec![
            HadwigerArtifactPayloadEntry::text(
                "truth_view_basis_digest",
                self.truth_view_basis_digest.clone(),
            ),
            HadwigerArtifactPayloadEntry::text("route_identity", self.route_identity.clone()),
            HadwigerArtifactPayloadEntry::text(
                "evaluation_identity",
                self.evaluation_identity.clone(),
            ),
            HadwigerArtifactPayloadEntry::text(
                "diagnostics_digest",
                self.diagnostics_digest.clone(),
            ),
            HadwigerArtifactPayloadEntry::text("replay_digest", self.replay_digest.clone()),
        ]
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerCheckerBoundaryKind {
    InProcess,
}

impl HadwigerCheckerBoundaryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InProcess => "in_process",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerCheckerPosture {
    Admitted,
    Rejected,
    Unsupported,
}

impl HadwigerCheckerPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Rejected => "rejected",
            Self::Unsupported => "unsupported",
        }
    }

    pub fn is_admitted(self) -> bool {
        self == Self::Admitted
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceVerification {
    core: HadwigerArtifactCore,
    checker_identity: String,
    checker_version: String,
    boundary_kind: HadwigerCheckerBoundaryKind,
    posture: HadwigerCheckerPosture,
    causal_evidence: HadwigerCheckerCausalEvidence,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl UnitDistanceVerification {
    pub(crate) fn checked(
        embedding_reference: HadwigerArtifactReference,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        checker_identity: impl Into<String>,
        checker_version: impl Into<String>,
        posture: HadwigerCheckerPosture,
        causal_evidence: HadwigerCheckerCausalEvidence,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let checker_identity = require_non_empty(checker_identity, "checker_identity")?;
        let checker_version = require_non_empty(checker_version, "checker_version")?;
        let core = checker_artifact(
            HadwigerArtifactKind::UnitDistanceVerification,
            "unit_distance_verification",
            embedding_reference,
            checker_identity.clone(),
            checker_version.clone(),
            posture,
            causal_evidence.clone(),
            query_declaration_reference.clone(),
        )?;
        Ok(Self {
            core,
            checker_identity,
            checker_version,
            boundary_kind: HadwigerCheckerBoundaryKind::InProcess,
            posture,
            causal_evidence,
            query_declaration_reference,
        })
    }

    pub fn is_admitted(&self) -> bool {
        self.posture.is_admitted()
    }

    pub fn posture(&self) -> HadwigerCheckerPosture {
        self.posture
    }

    pub fn checker_identity(&self) -> &str {
        &self.checker_identity
    }

    pub fn checker_version(&self) -> &str {
        &self.checker_version
    }

    pub fn boundary_kind(&self) -> HadwigerCheckerBoundaryKind {
        self.boundary_kind
    }

    pub fn causal_evidence(&self) -> &HadwigerCheckerCausalEvidence {
        &self.causal_evidence
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(UnitDistanceVerification, core);

pub(crate) fn checker_artifact(
    artifact_kind: HadwigerArtifactKind,
    affected_aspect: &'static str,
    input_reference: HadwigerArtifactReference,
    checker_identity: impl Into<String>,
    checker_version: impl Into<String>,
    posture: HadwigerCheckerPosture,
    causal_evidence: HadwigerCheckerCausalEvidence,
    query_declaration_reference: HadwigerQueryDeclarationReference,
) -> Result<HadwigerArtifactCore, HadwigerArtifactShapeError> {
    checker_artifact_with_entries(
        artifact_kind,
        affected_aspect,
        input_reference,
        checker_identity,
        checker_version,
        posture,
        causal_evidence,
        query_declaration_reference,
        Vec::new(),
    )
}

pub(crate) fn checker_artifact_with_entries(
    artifact_kind: HadwigerArtifactKind,
    affected_aspect: &'static str,
    input_reference: HadwigerArtifactReference,
    checker_identity: impl Into<String>,
    checker_version: impl Into<String>,
    posture: HadwigerCheckerPosture,
    causal_evidence: HadwigerCheckerCausalEvidence,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    extra_payload_entries: Vec<HadwigerArtifactPayloadEntry>,
) -> Result<HadwigerArtifactCore, HadwigerArtifactShapeError> {
    let checker_identity = require_non_empty(checker_identity, "checker_identity")?;
    let checker_version = require_non_empty(checker_version, "checker_version")?;
    let mut payload_entries = vec![
        HadwigerArtifactPayloadEntry::text("checker_identity", checker_identity.clone()),
        HadwigerArtifactPayloadEntry::text("checker_version", checker_version.clone()),
        HadwigerArtifactPayloadEntry::text(
            "boundary_kind",
            HadwigerCheckerBoundaryKind::InProcess.as_str(),
        ),
        HadwigerArtifactPayloadEntry::text("result_posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("affected_artifact", input_reference.stable_token()),
        HadwigerArtifactPayloadEntry::text("affected_aspect", affected_aspect),
        HadwigerArtifactPayloadEntry::text(
            "query_declaration_reference",
            query_declaration_reference.stable_token(),
        ),
    ];
    payload_entries.extend(extra_payload_entries);
    payload_entries.extend(causal_evidence.payload_entries());
    artifact_core(
        artifact_kind,
        HadwigerArtifactAuthorityOwner::Checker,
        HadwigerArtifactSourceReference::CheckerBoundary {
            checker_identity,
            checker_version,
        },
        vec![input_reference],
        payload_entries,
    )
}
