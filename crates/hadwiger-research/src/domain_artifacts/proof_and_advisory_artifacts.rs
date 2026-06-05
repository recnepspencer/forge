use super::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use super::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use super::query_references::HadwigerQueryDeclarationReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerProofClaimPosture {
    Candidate,
    Blocked,
    Admitted,
}

impl HadwigerProofClaimPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Blocked => "blocked",
            Self::Admitted => "admitted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerProofClaimKind {
    PlaneLowerBound { color_count: u32 },
    PlaneUpperBound { color_count: u32 },
    PlaneExactValue { color_count: u32 },
}

impl HadwigerProofClaimKind {
    pub fn color_count(self) -> u32 {
        match self {
            Self::PlaneLowerBound { color_count }
            | Self::PlaneUpperBound { color_count }
            | Self::PlaneExactValue { color_count } => color_count,
        }
    }

    pub fn claim_statement(self) -> String {
        match self {
            Self::PlaneLowerBound { color_count } => format!("chi(plane) >= {color_count}"),
            Self::PlaneUpperBound { color_count } => format!("chi(plane) <= {color_count}"),
            Self::PlaneExactValue { color_count } => format!("chi(plane) = {color_count}"),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::PlaneLowerBound { .. } => "plane_lower_bound",
            Self::PlaneUpperBound { .. } => "plane_upper_bound",
            Self::PlaneExactValue { .. } => "plane_exact_value",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofClaim {
    core: HadwigerArtifactCore,
    claim_id: String,
    claim_kind: HadwigerProofClaimKind,
    posture: HadwigerProofClaimPosture,
}

impl ProofClaim {
    pub fn candidate_lower_bound(
        witness_reference: HadwigerArtifactReference,
        claim_id: impl Into<String>,
        color_count: u32,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        proof_claim(
            witness_reference,
            claim_id,
            HadwigerProofClaimKind::PlaneLowerBound { color_count },
            HadwigerProofClaimPosture::Candidate,
            HadwigerArtifactAuthorityOwner::ProofCandidate,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "proof_claim".to_string(),
            },
        )
    }

    pub fn blocked_lower_bound(
        witness_reference: HadwigerArtifactReference,
        claim_id: impl Into<String>,
        color_count: u32,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        proof_claim(
            witness_reference,
            claim_id,
            HadwigerProofClaimKind::PlaneLowerBound { color_count },
            HadwigerProofClaimPosture::Blocked,
            HadwigerArtifactAuthorityOwner::ProofCandidate,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "proof_claim".to_string(),
            },
        )
    }

    pub fn admits_theorem_authority(&self) -> bool {
        self.posture == HadwigerProofClaimPosture::Admitted
    }

    pub fn posture(&self) -> HadwigerProofClaimPosture {
        self.posture
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn claim_kind(&self) -> HadwigerProofClaimKind {
        self.claim_kind
    }

    pub fn claim_statement(&self) -> String {
        self.claim_kind.claim_statement()
    }

    pub fn color_count(&self) -> u32 {
        self.claim_kind.color_count()
    }
}

impl_hadwiger_artifact!(ProofClaim, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerBoundWitnessArtifact {
    core: HadwigerArtifactCore,
    witness_id: String,
    forbidden_color_count: u32,
    lower_bound_color_count: u32,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl LowerBoundWitnessArtifact {
    pub(crate) fn admitted(
        witness_id: impl Into<String>,
        graph_version_reference: HadwigerArtifactReference,
        unit_distance_verification_reference: HadwigerArtifactReference,
        colorability_verification_reference: HadwigerArtifactReference,
        forbidden_color_count: u32,
        query_declaration_reference: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let witness_id = require_non_empty(witness_id, "witness_id")?;
        if forbidden_color_count == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "forbidden_color_count",
            });
        }
        let lower_bound_color_count = forbidden_color_count + 1;
        let core = artifact_core(
            HadwigerArtifactKind::LowerBoundWitnessArtifact,
            HadwigerArtifactAuthorityOwner::TheoremAuthority,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            vec![
                graph_version_reference,
                unit_distance_verification_reference,
                colorability_verification_reference,
            ],
            vec![
                HadwigerArtifactPayloadEntry::text("witness_id", witness_id.clone()),
                HadwigerArtifactPayloadEntry::unsigned(
                    "forbidden_color_count",
                    forbidden_color_count as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "lower_bound_color_count",
                    lower_bound_color_count as u128,
                ),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration_reference",
                    query_declaration_reference.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            witness_id,
            forbidden_color_count,
            lower_bound_color_count,
            query_declaration_reference,
        })
    }

    pub fn witness_id(&self) -> &str {
        &self.witness_id
    }

    pub fn forbidden_color_count(&self) -> u32 {
        self.forbidden_color_count
    }

    pub fn lower_bound_color_count(&self) -> u32 {
        self.lower_bound_color_count
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }
}

impl_hadwiger_artifact!(LowerBoundWitnessArtifact, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedBackgroundTheorem {
    core: HadwigerArtifactCore,
    theorem_id: String,
    theorem_statement: String,
    source: String,
    provenance_digest: String,
    authority_note: String,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl RetainedBackgroundTheorem {
    pub(crate) fn admitted_plane_seven_upper_bound(
        theorem_id: impl Into<String>,
        source: impl Into<String>,
        provenance_digest: impl Into<String>,
        authority_note: impl Into<String>,
        query_declaration_reference: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let theorem_id = require_non_empty(theorem_id, "theorem_id")?;
        let source = require_non_empty(source, "source")?;
        let provenance_digest = require_non_empty(provenance_digest, "provenance_digest")?;
        let authority_note = require_non_empty(authority_note, "authority_note")?;
        let theorem_statement = "chi(plane) <= 7".to_string();
        let core = artifact_core(
            HadwigerArtifactKind::RetainedBackgroundTheorem,
            HadwigerArtifactAuthorityOwner::TheoremAuthority,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            Vec::new(),
            vec![
                HadwigerArtifactPayloadEntry::text("theorem_id", theorem_id.clone()),
                HadwigerArtifactPayloadEntry::text("theorem_statement", theorem_statement.clone()),
                HadwigerArtifactPayloadEntry::text("source", source.clone()),
                HadwigerArtifactPayloadEntry::text("provenance_digest", provenance_digest.clone()),
                HadwigerArtifactPayloadEntry::text("authority_note", authority_note.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration_reference",
                    query_declaration_reference.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            theorem_id,
            theorem_statement,
            source,
            provenance_digest,
            authority_note,
            query_declaration_reference,
        })
    }

    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }

    pub fn theorem_statement(&self) -> &str {
        &self.theorem_statement
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }

    pub fn authority_note(&self) -> &str {
        &self.authority_note
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }
}

impl_hadwiger_artifact!(RetainedBackgroundTheorem, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AIAdvisoryArtifact {
    core: HadwigerArtifactCore,
    advisory_id: String,
}

impl AIAdvisoryArtifact {
    pub fn new(
        parent_reference: HadwigerArtifactReference,
        advisory_id: impl Into<String>,
        advisory_source_digest: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let advisory_id = require_non_empty(advisory_id, "advisory_id")?;
        let advisory_source_digest =
            require_non_empty(advisory_source_digest, "advisory_source_digest")?;
        let core = artifact_core(
            HadwigerArtifactKind::AIAdvisoryArtifact,
            HadwigerArtifactAuthorityOwner::AIAdvisory,
            HadwigerArtifactSourceReference::AIAdvisory {
                advisory_source_digest: advisory_source_digest.clone(),
            },
            vec![parent_reference],
            vec![
                HadwigerArtifactPayloadEntry::text("advisory_id", advisory_id.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "advisory_source_digest",
                    advisory_source_digest,
                ),
            ],
        )?;
        Ok(Self { core, advisory_id })
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn advisory_id(&self) -> &str {
        &self.advisory_id
    }
}

impl_hadwiger_artifact!(AIAdvisoryArtifact, core);

fn proof_claim(
    witness_reference: HadwigerArtifactReference,
    claim_id: impl Into<String>,
    claim_kind: HadwigerProofClaimKind,
    posture: HadwigerProofClaimPosture,
    authority_owner: HadwigerArtifactAuthorityOwner,
    source_reference: HadwigerArtifactSourceReference,
) -> Result<ProofClaim, HadwigerArtifactShapeError> {
    let claim_id = require_non_empty(claim_id, "claim_id")?;
    let color_count = claim_kind.color_count();
    if color_count == 0 {
        return Err(HadwigerArtifactShapeError::EmptyField {
            field: "color_count",
        });
    }
    let core = artifact_core(
        HadwigerArtifactKind::ProofClaim,
        authority_owner,
        source_reference,
        vec![witness_reference],
        vec![
            HadwigerArtifactPayloadEntry::text("claim_id", claim_id.clone()),
            HadwigerArtifactPayloadEntry::text("claim_kind", claim_kind.as_str()),
            HadwigerArtifactPayloadEntry::text("claim_statement", claim_kind.claim_statement()),
            HadwigerArtifactPayloadEntry::unsigned("color_count", color_count as u128),
            HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        ],
    )?;
    Ok(ProofClaim {
        core,
        claim_id,
        claim_kind,
        posture,
    })
}

pub(crate) fn admitted_proof_claim(
    parent_reference: HadwigerArtifactReference,
    claim_id: impl Into<String>,
    claim_kind: HadwigerProofClaimKind,
    query_declaration_reference: HadwigerQueryDeclarationReference,
) -> Result<ProofClaim, HadwigerArtifactShapeError> {
    proof_claim(
        parent_reference,
        claim_id,
        claim_kind,
        HadwigerProofClaimPosture::Admitted,
        HadwigerArtifactAuthorityOwner::TheoremAuthority,
        HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference),
    )
}

pub(crate) fn blocked_proof_claim(
    parent_reference: HadwigerArtifactReference,
    claim_id: impl Into<String>,
    claim_kind: HadwigerProofClaimKind,
) -> Result<ProofClaim, HadwigerArtifactShapeError> {
    proof_claim(
        parent_reference,
        claim_id,
        claim_kind,
        HadwigerProofClaimPosture::Blocked,
        HadwigerArtifactAuthorityOwner::ProofCandidate,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "blocked_proof_claim".to_string(),
        },
    )
}
