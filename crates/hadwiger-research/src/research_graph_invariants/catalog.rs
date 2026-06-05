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
    registration_blocked_rules: usize,
    breadth_inspected: usize,
}

impl ResearchGraphInvariantCounters {
    pub(crate) fn new(
        checked_rules: usize,
        violation_count: usize,
        denied_runtime_targets: usize,
        registration_blocked_rules: usize,
        breadth_inspected: usize,
    ) -> Self {
        Self {
            checked_rules,
            violation_count,
            denied_runtime_targets,
            registration_blocked_rules,
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

    pub fn registration_blocked_rules(&self) -> usize {
        self.registration_blocked_rules
    }

    pub fn breadth_inspected(&self) -> usize {
        self.breadth_inspected
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "checked={};violations={};denied={};blocked={};breadth={}",
            self.checked_rules,
            self.violation_count,
            self.denied_runtime_targets,
            self.registration_blocked_rules,
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
                HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.research_graph.v1"),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantCompatibilitySurface {
    surface: &'static str,
}

impl ResearchGraphInvariantCompatibilitySurface {
    pub(crate) fn new(surface: &'static str) -> Self {
        Self { surface }
    }

    pub fn as_str(&self) -> &'static str {
        self.surface
    }
}

impl PartialEq<&str> for ResearchGraphInvariantCompatibilitySurface {
    fn eq(&self, other: &&str) -> bool {
        self.surface == *other
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantCompatibilitySurfaces {
    surfaces: Vec<ResearchGraphInvariantCompatibilitySurface>,
}

impl ResearchGraphInvariantCompatibilitySurfaces {
    pub(crate) fn registration_targets() -> Self {
        Self {
            surfaces: vec![
                ResearchGraphInvariantCompatibilitySurface::new(
                    "ForgeQueryRuntime::builder().invariant_catalog(...)",
                ),
                ResearchGraphInvariantCompatibilitySurface::new(
                    "ForgeQueryRuntime::builder().custom_invariant(...)",
                ),
                ResearchGraphInvariantCompatibilitySurface::new(
                    "ForgeQueryRuntime::builder().register_invariant(...)",
                ),
                ResearchGraphInvariantCompatibilitySurface::new(
                    "ForgeQueryRuntime::builder().invariant_registration_artifact(...)",
                ),
            ],
        }
    }

    pub fn contains(&self, surface: &str) -> bool {
        self.surfaces
            .iter()
            .any(|candidate| candidate.as_str() == surface)
    }

    pub fn rows(&self) -> &[ResearchGraphInvariantCompatibilitySurface] {
        &self.surfaces
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResearchGraphInvariantRegistrationPosture {
    BlockedDraft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantRegistrationPlan {
    core: HadwigerArtifactCore,
    posture: ResearchGraphInvariantRegistrationPosture,
    compatible_query_surfaces: ResearchGraphInvariantCompatibilitySurfaces,
}

impl ResearchGraphInvariantRegistrationPlan {
    pub(crate) fn blocked_draft(
        catalog: &HadwigerResearchInvariantCatalog,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let compatible_query_surfaces =
            ResearchGraphInvariantCompatibilitySurfaces::registration_targets();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantRegistrationPlan,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_registration_plan".to_string(),
            },
            vec![catalog.reference()],
            registration_plan_payload(catalog, &compatible_query_surfaces),
        )?;
        Ok(Self {
            core,
            posture: ResearchGraphInvariantRegistrationPosture::BlockedDraft,
            compatible_query_surfaces,
        })
    }

    pub fn posture(&self) -> ResearchGraphInvariantRegistrationPosture {
        self.posture
    }

    pub fn compatible_query_surfaces(&self) -> &ResearchGraphInvariantCompatibilitySurfaces {
        &self.compatible_query_surfaces
    }

    pub fn registers_runtime_invariants(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchGraphInvariantRegistrationPlan, core);

fn catalog_payload(
    rules: &[ResearchGraphInvariantRule],
    counters: &ResearchGraphInvariantCounters,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.research_graph.catalog.v1"),
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

fn registration_plan_payload(
    catalog: &HadwigerResearchInvariantCatalog,
    surfaces: &ResearchGraphInvariantCompatibilitySurfaces,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("posture", "blocked_draft"),
        HadwigerArtifactPayloadEntry::text("catalog", catalog.artifact_digest().stable_token()),
    ];
    for surface in surfaces.rows() {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "compatible_query_surface",
            surface.as_str(),
        ));
    }
    payload
}
