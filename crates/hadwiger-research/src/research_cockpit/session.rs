use crate::agent_advisory::AgentExplorationAdmissionChecked;
use crate::discovery_loop::{DerivedFrontierState, DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::research_graph_invariants::HadwigerResearchInvariantCatalog;

use super::ResearchCockpitError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitInputDigest {
    stable_token: String,
}

impl ResearchCockpitInputDigest {
    fn new(stable_token: String) -> Self {
        Self { stable_token }
    }

    pub fn stable_token(&self) -> &str {
        &self.stable_token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitSession {
    core: HadwigerArtifactCore,
    session_id: String,
    input_digest: ResearchCockpitInputDigest,
    corpus: ResearchEvidenceCorpus,
    frontier: DiscoveryFrontier,
    invariant_catalog: HadwigerResearchInvariantCatalog,
    agent_admission: Option<AgentExplorationAdmissionChecked>,
    derived_frontier_state: Option<DerivedFrontierState>,
}

impl ResearchCockpitSession {
    pub fn builder(session_id: impl Into<String>) -> ResearchCockpitSessionBuilder {
        ResearchCockpitSessionBuilder {
            session_id: session_id.into(),
            corpus: None,
            frontier: None,
            invariant_catalog: None,
            agent_admission: None,
            derived_frontier_state: None,
        }
    }

    pub(crate) fn from_builder(
        builder: ResearchCockpitSessionBuilder,
        handle_digest: &str,
    ) -> Result<Self, ResearchCockpitError> {
        let session_id = require_non_empty(builder.session_id, "session_id")?;
        let corpus = builder
            .corpus
            .ok_or(ResearchCockpitError::MissingInput { field: "corpus" })?;
        let frontier = builder
            .frontier
            .ok_or(ResearchCockpitError::MissingInput { field: "frontier" })?;
        let invariant_catalog =
            builder
                .invariant_catalog
                .ok_or(ResearchCockpitError::MissingInput {
                    field: "invariant_catalog",
                })?;
        let agent_admission = builder.agent_admission;
        let derived_frontier_state = builder.derived_frontier_state;
        let input_digest = ResearchCockpitInputDigest::new(input_digest_token(
            handle_digest,
            &corpus,
            &frontier,
            &invariant_catalog,
            agent_admission.as_ref(),
            derived_frontier_state.as_ref(),
        ));
        let core = artifact_core(
            HadwigerArtifactKind::ResearchCockpitSession,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_cockpit_session".to_string(),
            },
            session_parents(
                &corpus,
                &frontier,
                &invariant_catalog,
                agent_admission.as_ref(),
                derived_frontier_state.as_ref(),
            ),
            session_payload(&session_id, &input_digest),
        )?;
        Ok(Self {
            core,
            session_id,
            input_digest,
            corpus,
            frontier,
            invariant_catalog,
            agent_admission,
            derived_frontier_state,
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn session_digest(&self) -> &str {
        self.artifact_digest().stable_token()
    }

    pub fn input_digest(&self) -> &ResearchCockpitInputDigest {
        &self.input_digest
    }

    pub fn corpus(&self) -> &ResearchEvidenceCorpus {
        &self.corpus
    }

    pub fn frontier(&self) -> &DiscoveryFrontier {
        &self.frontier
    }

    pub fn invariant_catalog(&self) -> &HadwigerResearchInvariantCatalog {
        &self.invariant_catalog
    }

    pub fn agent_admission(&self) -> Option<&AgentExplorationAdmissionChecked> {
        self.agent_admission.as_ref()
    }

    pub fn derived_frontier_state(&self) -> Option<&DerivedFrontierState> {
        self.derived_frontier_state.as_ref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchCockpitSession, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitSessionBuilder {
    session_id: String,
    corpus: Option<ResearchEvidenceCorpus>,
    frontier: Option<DiscoveryFrontier>,
    invariant_catalog: Option<HadwigerResearchInvariantCatalog>,
    agent_admission: Option<AgentExplorationAdmissionChecked>,
    derived_frontier_state: Option<DerivedFrontierState>,
}

impl ResearchCockpitSessionBuilder {
    pub fn with_corpus(mut self, corpus: ResearchEvidenceCorpus) -> Self {
        self.corpus = Some(corpus);
        self
    }

    pub fn with_frontier(mut self, frontier: DiscoveryFrontier) -> Self {
        self.frontier = Some(frontier);
        self
    }

    pub fn with_invariant_catalog(mut self, catalog: HadwigerResearchInvariantCatalog) -> Self {
        self.invariant_catalog = Some(catalog);
        self
    }

    pub fn with_agent_admission(mut self, checked: AgentExplorationAdmissionChecked) -> Self {
        self.agent_admission = Some(checked);
        self
    }

    pub fn with_derived_frontier_state(mut self, state: DerivedFrontierState) -> Self {
        self.derived_frontier_state = Some(state);
        self
    }

    pub fn finish(self) -> Result<Self, ResearchCockpitError> {
        require_non_empty(self.session_id.as_str(), "session_id")?;
        if self.corpus.is_none() {
            return Err(ResearchCockpitError::MissingInput { field: "corpus" });
        }
        if self.frontier.is_none() {
            return Err(ResearchCockpitError::MissingInput { field: "frontier" });
        }
        if self.invariant_catalog.is_none() {
            return Err(ResearchCockpitError::MissingInput {
                field: "invariant_catalog",
            });
        }
        Ok(self)
    }
}

fn session_parents(
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    catalog: &HadwigerResearchInvariantCatalog,
    agent_admission: Option<&AgentExplorationAdmissionChecked>,
    derived_frontier_state: Option<&DerivedFrontierState>,
) -> Vec<HadwigerArtifactReference> {
    let mut parents = vec![
        corpus.reference(),
        frontier.reference(),
        catalog.reference(),
    ];
    if let Some(agent_admission) = agent_admission {
        parents.push(agent_admission.batch().reference());
        parents.extend(
            agent_admission
                .advisory_artifacts()
                .iter()
                .map(HadwigerCanonicalArtifact::reference),
        );
    }
    if let Some(derived_frontier_state) = derived_frontier_state {
        parents.push(derived_frontier_state.reference());
    }
    parents
}

fn input_digest_token(
    handle_digest: &str,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    catalog: &HadwigerResearchInvariantCatalog,
    agent_admission: Option<&AgentExplorationAdmissionChecked>,
    derived_frontier_state: Option<&DerivedFrontierState>,
) -> String {
    let agent = agent_admission
        .map(|checked| checked.batch().artifact_digest().stable_token().to_string())
        .unwrap_or_else(|| "none".to_string());
    let derived = derived_frontier_state
        .map(|state| state.artifact_digest().stable_token().to_string())
        .unwrap_or_else(|| "none".to_string());
    format!(
        "handle={handle_digest};corpus={};frontier={};catalog={};agent={agent};derived={derived}",
        corpus.artifact_digest().stable_token(),
        frontier.artifact_digest().stable_token(),
        catalog.artifact_digest().stable_token()
    )
}

fn session_payload(
    session_id: &str,
    input_digest: &ResearchCockpitInputDigest,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("session_id", session_id),
        HadwigerArtifactPayloadEntry::text("input_digest", input_digest.stable_token()),
    ]
}
