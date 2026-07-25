use worth_foundational::facade::FoundationalPerformanceCounterName;
use worth_query_installation::facade::{
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchPosture, WorthQueryDecisionKind,
    WorthQuerySourceOutputCorrespondence, WorthQueryTransformationDisposition,
    WorthQueryTransformationErrorPosture, WorthQueryTransformationLossPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryStructuralCounterObservation {
    name: FoundationalPerformanceCounterName,
    initial: u64,
    observed: u64,
    provider_certification: Option<String>,
}

impl WorthQueryStructuralCounterObservation {
    pub fn new(name: FoundationalPerformanceCounterName, initial: u64, observed: u64) -> Self {
        Self {
            name,
            initial,
            observed,
            provider_certification: None,
        }
    }

    pub fn with_provider_certification(mut self, identity: impl Into<String>) -> Self {
        self.provider_certification = Some(identity.into());
        self
    }

    pub fn name(&self) -> &FoundationalPerformanceCounterName {
        &self.name
    }

    pub const fn initial(&self) -> u64 {
        self.initial
    }

    pub const fn observed(&self) -> u64 {
        self.observed
    }

    pub fn provider_certification(&self) -> Option<&str> {
        self.provider_certification.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDecisionSummaryCounts {
    occurrence_count: u64,
    causal_parent_count: u64,
    affected_artifact_count: u64,
    recovery_relevant_count: u64,
}

impl WorthQueryDecisionSummaryCounts {
    pub const fn new(
        occurrence_count: u64,
        causal_parent_count: u64,
        affected_artifact_count: u64,
        recovery_relevant_count: u64,
    ) -> Self {
        Self {
            occurrence_count,
            causal_parent_count,
            affected_artifact_count,
            recovery_relevant_count,
        }
    }

    pub const fn occurrence_count(self) -> u64 {
        self.occurrence_count
    }

    pub const fn causal_parent_count(self) -> u64 {
        self.causal_parent_count
    }

    pub const fn affected_artifact_count(self) -> u64 {
        self.affected_artifact_count
    }

    pub const fn recovery_relevant_count(self) -> u64 {
        self.recovery_relevant_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDecisionSummary {
    kind: WorthQueryDecisionKind,
    counts: WorthQueryDecisionSummaryCounts,
}

impl WorthQueryDecisionSummary {
    pub const fn new(
        kind: WorthQueryDecisionKind,
        counts: WorthQueryDecisionSummaryCounts,
    ) -> Self {
        Self { kind, counts }
    }

    pub fn kind(&self) -> &WorthQueryDecisionKind {
        &self.kind
    }

    pub const fn counts(&self) -> WorthQueryDecisionSummaryCounts {
        self.counts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceValue {
    family: String,
    value: String,
}

impl WorthQueryDomainEvidenceValue {
    pub fn new(family: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            value: value.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateTerminationClass {
    Completed,
    Exhausted,
    BoundReached,
    SampleCompleted,
    HeuristicStop,
    Interrupted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateFeasibilityClass {
    NotApplicable,
    NoFeasibleCandidate,
    FeasibleCandidateFound,
    AllConsideredFeasible,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateIncumbentDisposition {
    NotApplicable,
    None,
    Selected,
    Reused,
    RejectedAll,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCandidateSearchSummaryParts {
    pub universe: WorthQueryDomainEvidenceValue,
    pub considered_count: u64,
    pub termination_family: String,
    pub termination: WorthQueryCandidateTerminationClass,
    pub completeness: WorthQueryCandidateSearchPosture,
    pub feasibility_family: String,
    pub feasibility: WorthQueryCandidateFeasibilityClass,
    pub comparison_authority: WorthQueryDomainEvidenceValue,
    pub optimality: WorthQueryCandidateOptimalityPosture,
    pub rejected_count: u64,
    pub incumbent_family: String,
    pub incumbent: WorthQueryCandidateIncumbentDisposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCandidateSearchSummary {
    parts: WorthQueryCandidateSearchSummaryParts,
}

impl WorthQueryCandidateSearchSummary {
    pub const fn from_parts(parts: WorthQueryCandidateSearchSummaryParts) -> Self {
        Self { parts }
    }

    pub const fn parts(&self) -> &WorthQueryCandidateSearchSummaryParts {
        &self.parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTransformationSummaryParts {
    pub source_occurrence: WorthQueryDomainEvidenceValue,
    pub output_occurrence_identity: String,
    pub transformation_family: String,
    pub transformation_version: u32,
    pub correspondence: WorthQuerySourceOutputCorrespondence,
    pub disposition: WorthQueryTransformationDisposition,
    pub error: WorthQueryTransformationErrorPosture,
    pub loss: WorthQueryTransformationLossPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTransformationSummary {
    parts: WorthQueryTransformationSummaryParts,
}

impl WorthQueryTransformationSummary {
    pub const fn from_parts(parts: WorthQueryTransformationSummaryParts) -> Self {
        Self { parts }
    }

    pub const fn parts(&self) -> &WorthQueryTransformationSummaryParts {
        &self.parts
    }
}
