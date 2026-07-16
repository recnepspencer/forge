use crate::discovery_loop::{DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchGraphInvariantFamily {
    FailureResidency,
    SuppressionRelation,
    HypothesisLifecycle,
    BranchPromotion,
    ExecutableExperimentAdmission,
}
impl ResearchGraphInvariantFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailureResidency => "failure_residency",
            Self::SuppressionRelation => "suppression_relation",
            Self::HypothesisLifecycle => "hypothesis_lifecycle",
            Self::BranchPromotion => "branch_promotion",
            Self::ExecutableExperimentAdmission => "executable_experiment_admission",
        }
    }

    pub fn query_invariant_family(self) -> &'static str {
        match self {
            Self::FailureResidency => "hadwiger.research_graph.failure_residency",
            Self::SuppressionRelation => "hadwiger.research_graph.suppression_relation",
            Self::HypothesisLifecycle => "hadwiger.research_graph.hypothesis_lifecycle",
            Self::BranchPromotion => "hadwiger.research_graph.branch_promotion",
            Self::ExecutableExperimentAdmission => {
                "hadwiger.research_graph.executable_experiment_admission"
            }
        }
    }

    pub(crate) fn all() -> [Self; 5] {
        [
            Self::FailureResidency,
            Self::SuppressionRelation,
            Self::HypothesisLifecycle,
            Self::BranchPromotion,
            Self::ExecutableExperimentAdmission,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResearchGraphInvariantScope {
    EvidenceCorpus,
    DiscoveryFrontier,
    ExperimentBatch,
    LowerRuntimeBoundary,
}

impl ResearchGraphInvariantScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EvidenceCorpus => "evidence_corpus",
            Self::DiscoveryFrontier => "discovery_frontier",
            Self::ExperimentBatch => "experiment_batch",
            Self::LowerRuntimeBoundary => "lower_runtime_boundary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantCounters {
    checked_rules: usize,
    violation_count: usize,
    denied_runtime_targets: usize,
    registration_ready_rules: usize,
    breadth_inspected: usize,
}

impl ResearchGraphInvariantCounters {
    pub(crate) fn new(
        checked_rules: usize,
        violation_count: usize,
        denied_runtime_targets: usize,
        registration_ready_rules: usize,
        breadth_inspected: usize,
    ) -> Self {
        Self {
            checked_rules,
            violation_count,
            denied_runtime_targets,
            registration_ready_rules,
            breadth_inspected,
        }
    }

    pub fn checked_rules(&self) -> usize {
        self.checked_rules
    }

    pub fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn denied_runtime_targets(&self) -> usize {
        self.denied_runtime_targets
    }

    pub fn registration_ready_rules(&self) -> usize {
        self.registration_ready_rules
    }

    pub fn breadth_inspected(&self) -> usize {
        self.breadth_inspected
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "checked={};violations={};denied={};ready={};breadth={}",
            self.checked_rules,
            self.violation_count,
            self.denied_runtime_targets,
            self.registration_ready_rules,
            self.breadth_inspected
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantRule {
    core: HadwigerArtifactCore,
    family: ResearchGraphInvariantFamily,
    scope: ResearchGraphInvariantScope,
}

impl ResearchGraphInvariantRule {
    pub(crate) fn new(
        family: ResearchGraphInvariantFamily,
        scope: ResearchGraphInvariantScope,
        parents: Vec<HadwigerArtifactReference>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantRule,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_rule".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.research_graph.v1"),
                HadwigerArtifactPayloadEntry::text("family", family.as_str()),
                HadwigerArtifactPayloadEntry::text("scope", scope.as_str()),
                HadwigerArtifactPayloadEntry::text("query_family", family.query_invariant_family()),
            ],
        )?;
        Ok(Self {
            core,
            family,
            scope,
        })
    }

    pub fn family(&self) -> ResearchGraphInvariantFamily {
        self.family
    }

    pub fn scope(&self) -> &ResearchGraphInvariantScope {
        &self.scope
    }
}

impl_hadwiger_artifact!(ResearchGraphInvariantRule, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerResearchInvariantCatalog {
    core: HadwigerArtifactCore,
    rules: Vec<ResearchGraphInvariantRule>,
    counters: ResearchGraphInvariantCounters,
}

impl HadwigerResearchInvariantCatalog {
    pub(crate) fn new(
        corpus: &ResearchEvidenceCorpus,
        frontier: &DiscoveryFrontier,
        mut rules: Vec<ResearchGraphInvariantRule>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        rules.sort_by_key(|rule| (rule.family(), rule.reference().stable_token()));
        let counters = ResearchGraphInvariantCounters::new(
            rules.len(),
            0,
            0,
            rules.len(),
            corpus.evidence_references().len() + frontier.experiment_plans().len(),
        );
        let mut parents = vec![corpus.reference(), frontier.reference()];
        parents.extend(rules.iter().map(ResearchGraphInvariantRule::reference));
        let core = artifact_core(
            HadwigerArtifactKind::HadwigerResearchInvariantCatalog,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hadwiger_research_invariant_catalog".to_string(),
            },
            parents,
            catalog_payload(&rules, &counters),
        )?;
        Ok(Self {
            core,
            rules,
            counters,
        })
    }

    pub fn rules(&self) -> &[ResearchGraphInvariantRule] {
        &self.rules
    }

    pub fn counters(&self) -> &ResearchGraphInvariantCounters {
        &self.counters
    }

    pub fn has_rule_family(&self, family: ResearchGraphInvariantFamily) -> bool {
        self.rules.iter().any(|rule| rule.family() == family)
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerResearchInvariantCatalog, core);

fn catalog_payload(
    rules: &[ResearchGraphInvariantRule],
    counters: &ResearchGraphInvariantCounters,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.research_graph.catalog.v1"),
        HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
    ];
    for rule in rules {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "rule",
            rule.reference().stable_token(),
        ));
    }
    payload
}
