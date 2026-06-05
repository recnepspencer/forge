use crate::domain_artifacts::HadwigerArtifactAuthorityOwner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerAspectAuthorityError {
    EmptyField {
        field: &'static str,
    },
    DuplicateAspect {
        aspect_kind: HadwigerAspectKind,
    },
    DuplicateDependency {
        required_by: HadwigerAspectKind,
        required: HadwigerAspectKind,
    },
    SelfDependency {
        aspect_kind: HadwigerAspectKind,
    },
    CyclicDependency {
        aspect_kind: HadwigerAspectKind,
    },
    MathematicalAuthorityNotAdmitted {
        aspect_kind: HadwigerAspectKind,
    },
    MissingDependencyRoot {
        aspect_kind: HadwigerAspectKind,
    },
    EmptyClosureGraph,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerAspectKind {
    AbstractGraphStructure,
    GraphVersionShape,
    EmbeddingCandidate,
    UnitDistanceEmbedding,
    KColorabilityEncoding,
    SolverRunEvidence,
    NotKColorable,
    UnsatCore,
    GadgetContract,
    ReductionTrace,
    GraphComposition,
    LowerBoundWitness,
    AIAdvisory,
    FailureEvidence,
}

impl HadwigerAspectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbstractGraphStructure => "abstract_graph_structure",
            Self::GraphVersionShape => "graph_version_shape",
            Self::EmbeddingCandidate => "embedding_candidate",
            Self::UnitDistanceEmbedding => "unit_distance_embedding",
            Self::KColorabilityEncoding => "k_colorability_encoding",
            Self::SolverRunEvidence => "solver_run_evidence",
            Self::NotKColorable => "not_k_colorable",
            Self::UnsatCore => "unsat_core",
            Self::GadgetContract => "gadget_contract",
            Self::ReductionTrace => "reduction_trace",
            Self::GraphComposition => "graph_composition",
            Self::LowerBoundWitness => "lower_bound_witness",
            Self::AIAdvisory => "ai_advisory",
            Self::FailureEvidence => "failure_evidence",
        }
    }

    pub fn query_aspect_path(self) -> &'static str {
        match self {
            Self::AbstractGraphStructure => "hadwiger.graph.abstract_structure",
            Self::GraphVersionShape => "hadwiger.graph.version_shape",
            Self::EmbeddingCandidate => "hadwiger.embedding.candidate",
            Self::UnitDistanceEmbedding => "hadwiger.embedding.unit_distance",
            Self::KColorabilityEncoding => "hadwiger.colorability.encoding",
            Self::SolverRunEvidence => "hadwiger.colorability.solver_run",
            Self::NotKColorable => "hadwiger.colorability.not_k_colorable",
            Self::UnsatCore => "hadwiger.colorability.unsat_core",
            Self::GadgetContract => "hadwiger.gadget.contract",
            Self::ReductionTrace => "hadwiger.reduction.trace",
            Self::GraphComposition => "hadwiger.graph.composition",
            Self::LowerBoundWitness => "hadwiger.lower_bound.witness",
            Self::AIAdvisory => "hadwiger.ai.advisory",
            Self::FailureEvidence => "hadwiger.failure.evidence",
        }
    }

    pub fn requires_external_math_authority(self) -> bool {
        matches!(
            self,
            Self::UnitDistanceEmbedding | Self::NotKColorable | Self::LowerBoundWitness
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerAspectPosture {
    Admitted,
    Rejected,
    Conflict,
    Stale,
    Missing,
    Unsupported,
    Deferred,
    Advisory,
}

impl HadwigerAspectPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Rejected => "rejected",
            Self::Conflict => "conflict",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Unsupported => "unsupported",
            Self::Deferred => "deferred",
            Self::Advisory => "advisory",
        }
    }

    pub(crate) fn severity_rank(self) -> u8 {
        match self {
            Self::Rejected => 7,
            Self::Conflict => 6,
            Self::Stale => 5,
            Self::Missing => 4,
            Self::Unsupported => 3,
            Self::Deferred => 2,
            Self::Advisory => 1,
            Self::Admitted => 0,
        }
    }

    pub fn satisfies_mathematical_dependency(self) -> bool {
        self == Self::Admitted
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HadwigerAspectScope {
    stable_token: String,
}

impl HadwigerAspectScope {
    pub fn artifact(stable_token: impl Into<String>) -> Result<Self, HadwigerAspectAuthorityError> {
        let stable_token = require_non_empty(stable_token, "aspect_scope")?;
        Ok(Self { stable_token })
    }

    pub fn stable_token(&self) -> &str {
        &self.stable_token
    }
}

pub(crate) fn require_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerAspectAuthorityError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(HadwigerAspectAuthorityError::EmptyField { field })
    } else {
        Ok(value)
    }
}

pub(crate) fn authority_token(authority_owner: HadwigerArtifactAuthorityOwner) -> &'static str {
    authority_owner.as_str()
}
