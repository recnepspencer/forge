use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::catalog::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantCounters, ResearchGraphInvariantFamily,
    ResearchGraphInvariantScope,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResearchGraphInvariantViolationKind {
    MissingRetainedNegativeEvidence,
    SuppressedExperimentMissingProof,
    HypothesisReactivationMissingEvidence,
    DiscoveryArtifactClaimsAuthority,
    ExecutableExperimentReadinessDrift,
}

impl ResearchGraphInvariantViolationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRetainedNegativeEvidence => "missing_retained_negative_evidence",
            Self::SuppressedExperimentMissingProof => "suppressed_experiment_missing_proof",
            Self::HypothesisReactivationMissingEvidence => {
                "hypothesis_reactivation_missing_evidence"
            }
            Self::DiscoveryArtifactClaimsAuthority => "discovery_artifact_claims_authority",
            Self::ExecutableExperimentReadinessDrift => "executable_experiment_readiness_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantViolation {
    core: HadwigerArtifactCore,
    violation_kind: ResearchGraphInvariantViolationKind,
    rule_family: ResearchGraphInvariantFamily,
    scope: ResearchGraphInvariantScope,
    detail: String,
    counters: ResearchGraphInvariantCounters,
}

impl ResearchGraphInvariantViolation {
    pub(crate) fn new(
        catalog: &HadwigerResearchInvariantCatalog,
        violation_kind: ResearchGraphInvariantViolationKind,
        rule_family: ResearchGraphInvariantFamily,
        scope: ResearchGraphInvariantScope,
        detail: impl Into<String>,
        parents: Vec<HadwigerArtifactReference>,
        breadth_inspected: usize,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let detail = detail.into();
        let counters = ResearchGraphInvariantCounters::new(
            catalog.rules().len(),
            1,
            0,
            catalog.rules().len(),
            breadth_inspected,
        );
        let mut parent_artifacts = vec![catalog.reference()];
        parent_artifacts.extend(parents);
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantViolation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_violation".to_string(),
            },
            parent_artifacts,
            vec![
                HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.violation.v1"),
                HadwigerArtifactPayloadEntry::text("kind", violation_kind.as_str()),
                HadwigerArtifactPayloadEntry::text("family", rule_family.as_str()),
                HadwigerArtifactPayloadEntry::text("scope", scope.as_str()),
                HadwigerArtifactPayloadEntry::text("detail", detail.clone()),
                HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
            ],
        )?;
        Ok(Self {
            core,
            violation_kind,
            rule_family,
            scope,
            detail,
            counters,
        })
    }

    pub fn violation_kind(&self) -> ResearchGraphInvariantViolationKind {
        self.violation_kind
    }

    pub fn rule_family(&self) -> ResearchGraphInvariantFamily {
        self.rule_family
    }

    pub fn scope(&self) -> &ResearchGraphInvariantScope {
        &self.scope
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> &ResearchGraphInvariantCounters {
        &self.counters
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchGraphInvariantViolation, core);
