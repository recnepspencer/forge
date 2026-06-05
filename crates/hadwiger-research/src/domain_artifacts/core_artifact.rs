use forge_foundational::facade::CanonicalDerivedDigest;

use super::query_references::{HadwigerQueryDeclarationReference, HadwigerQueryEnvelopeReference};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerArtifactShapeError {
    EmptyField { field: &'static str },
    DuplicateVertex { vertex_label: String },
    MissingEdgeEndpoint { vertex_label: String },
    SelfEdge { vertex_label: String },
    EmptyParentArtifacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerArtifactKind {
    GraphIdentity,
    GraphVersion,
    VertexIdentity,
    EdgeIdentity,
    EmbeddingCandidate,
    UnitDistanceVerification,
    ColorabilityEncoding,
    SolverRun,
    ColorabilityVerification,
    UnsatCoreArtifact,
    GadgetDefinition,
    GadgetContract,
    GraphComposition,
    ReductionTrace,
    ProofClaim,
    AIAdvisoryArtifact,
    AgentAdvisoryArtifact,
    AgentExplorationBatch,
    AgentAdvisoryContributionRecord,
    AgentExperimentProposalScreening,
    WholePlaneColoringConstruction,
    WholePlaneColoringVerification,
    LowerBoundWitnessArtifact,
    RetainedBackgroundTheorem,
    RejectionExplanation,
    PartialAdmissionExplanation,
    QueryRecoveryExplanation,
    RepairObligation,
    ReusableNegativeEvidence,
    ConservativeEscalationExplanation,
    ResearchEvidenceCorpus,
    GraphResidentFailure,
    FailureBasisFingerprint,
    PatternSignature,
    MotifObservation,
    InvariantHypothesis,
    InvariantCandidate,
    CounterexampleObligation,
    DeadEndSignature,
    ExperimentSuppressionProof,
    ExperimentPlan,
    ExperimentBatch,
    ExperimentResult,
    DiscoveryFrontier,
    DerivedFrontierState,
    RetiredHypothesisRecord,
    ReactivationCondition,
    HadwigerResearchInvariantCatalog,
    ResearchGraphInvariantRule,
    ResearchGraphInvariantViolation,
    ResearchGraphInvariantDenial,
    ResearchGraphInvariantRegistrationPlan,
    ResearchCockpitSession,
    ResearchCockpitActionPacket,
    ResearchCockpitEquivalenceClass,
    ResearchCockpitReport,
    HadwigerCertificationBundle,
    TileEquivalenceWitness,
    CandidateScreeningInvariantCatalog,
    CandidateScreeningInvariantNode,
}

impl HadwigerArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphIdentity => "graph_identity",
            Self::GraphVersion => "graph_version",
            Self::VertexIdentity => "vertex_identity",
            Self::EdgeIdentity => "edge_identity",
            Self::EmbeddingCandidate => "embedding_candidate",
            Self::UnitDistanceVerification => "unit_distance_verification",
            Self::ColorabilityEncoding => "colorability_encoding",
            Self::SolverRun => "solver_run",
            Self::ColorabilityVerification => "colorability_verification",
            Self::UnsatCoreArtifact => "unsat_core_artifact",
            Self::GadgetDefinition => "gadget_definition",
            Self::GadgetContract => "gadget_contract",
            Self::GraphComposition => "graph_composition",
            Self::ReductionTrace => "reduction_trace",
            Self::ProofClaim => "proof_claim",
            Self::AIAdvisoryArtifact => "ai_advisory_artifact",
            Self::AgentAdvisoryArtifact => "agent_advisory_artifact",
            Self::AgentExplorationBatch => "agent_exploration_batch",
            Self::AgentAdvisoryContributionRecord => "agent_advisory_contribution_record",
            Self::AgentExperimentProposalScreening => "agent_experiment_proposal_screening",
            Self::WholePlaneColoringConstruction => "whole_plane_coloring_construction",
            Self::WholePlaneColoringVerification => "whole_plane_coloring_verification",
            Self::LowerBoundWitnessArtifact => "lower_bound_witness_artifact",
            Self::RetainedBackgroundTheorem => "retained_background_theorem",
            Self::RejectionExplanation => "rejection_explanation",
            Self::PartialAdmissionExplanation => "partial_admission_explanation",
            Self::QueryRecoveryExplanation => "query_recovery_explanation",
            Self::RepairObligation => "repair_obligation",
            Self::ReusableNegativeEvidence => "reusable_negative_evidence",
            Self::ConservativeEscalationExplanation => "conservative_escalation_explanation",
            Self::ResearchEvidenceCorpus => "research_evidence_corpus",
            Self::GraphResidentFailure => "graph_resident_failure",
            Self::FailureBasisFingerprint => "failure_basis_fingerprint",
            Self::PatternSignature => "pattern_signature",
            Self::MotifObservation => "motif_observation",
            Self::InvariantHypothesis => "invariant_hypothesis",
            Self::InvariantCandidate => "invariant_candidate",
            Self::CounterexampleObligation => "counterexample_obligation",
            Self::DeadEndSignature => "dead_end_signature",
            Self::ExperimentSuppressionProof => "experiment_suppression_proof",
            Self::ExperimentPlan => "experiment_plan",
            Self::ExperimentBatch => "experiment_batch",
            Self::ExperimentResult => "experiment_result",
            Self::DiscoveryFrontier => "discovery_frontier",
            Self::DerivedFrontierState => "derived_frontier_state",
            Self::RetiredHypothesisRecord => "retired_hypothesis_record",
            Self::ReactivationCondition => "reactivation_condition",
            Self::HadwigerResearchInvariantCatalog => "hadwiger_research_invariant_catalog",
            Self::ResearchGraphInvariantRule => "research_graph_invariant_rule",
            Self::ResearchGraphInvariantViolation => "research_graph_invariant_violation",
            Self::ResearchGraphInvariantDenial => "research_graph_invariant_denial",
            Self::ResearchGraphInvariantRegistrationPlan => {
                "research_graph_invariant_registration_plan"
            }
            Self::ResearchCockpitSession => "research_cockpit_session",
            Self::ResearchCockpitActionPacket => "research_cockpit_action_packet",
            Self::ResearchCockpitEquivalenceClass => "research_cockpit_equivalence_class",
            Self::ResearchCockpitReport => "research_cockpit_report",
            Self::HadwigerCertificationBundle => "hadwiger_certification_bundle",
            Self::TileEquivalenceWitness => "tile_equivalence_witness",
            Self::CandidateScreeningInvariantCatalog => "candidate_screening_invariant_catalog",
            Self::CandidateScreeningInvariantNode => "candidate_screening_invariant_node",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerArtifactAuthorityOwner {
    QueryDeclaration,
    QueryEnvelope,
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
            Self::QueryEnvelope => "query_envelope",
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
    QueryEnvelope(HadwigerQueryEnvelopeReference),
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
            Self::QueryEnvelope(reference) => {
                format!("query_envelope:{}", reference.stable_token())
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

macro_rules! impl_hadwiger_artifact {
    ($type:ty, $core:ident) => {
        impl $crate::domain_artifacts::core_artifact::HadwigerCanonicalArtifact for $type {
            fn artifact_kind(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactKind {
                self.$core.artifact_kind()
            }

            fn artifact_digest(
                &self,
            ) -> &$crate::domain_artifacts::core_artifact::HadwigerArtifactDigest {
                self.$core.artifact_digest()
            }

            fn authority_owner(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactAuthorityOwner {
                self.$core.authority_owner()
            }

            fn source_reference(
                &self,
            ) -> &$crate::domain_artifacts::core_artifact::HadwigerArtifactSourceReference {
                self.$core.source_reference()
            }

            fn parent_artifacts(
                &self,
            ) -> &[$crate::domain_artifacts::core_artifact::HadwigerArtifactReference] {
                self.$core.parent_artifacts()
            }

            fn reference(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactReference {
                self.$core.reference()
            }
        }
    };
}

pub(crate) use impl_hadwiger_artifact;
