use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
    HadwigerCanonicalArtifact,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

use super::session::ResearchCockpitSession;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ResearchCockpitCounters {
    corpus_width: usize,
    frontier_width: usize,
    action_count: usize,
    suppression_hits: usize,
    tile_equivalence_hits: usize,
    query_readiness_checks: usize,
    blocked_action_count: usize,
}

impl ResearchCockpitCounters {
    pub(crate) fn new(
        corpus_width: usize,
        frontier_width: usize,
        action_count: usize,
        suppression_hits: usize,
        tile_equivalence_hits: usize,
        query_readiness_checks: usize,
        blocked_action_count: usize,
    ) -> Self {
        Self {
            corpus_width,
            frontier_width,
            action_count,
            suppression_hits,
            tile_equivalence_hits,
            query_readiness_checks,
            blocked_action_count,
        }
    }

    pub fn corpus_width(&self) -> usize {
        self.corpus_width
    }

    pub fn frontier_width(&self) -> usize {
        self.frontier_width
    }

    pub fn action_count(&self) -> usize {
        self.action_count
    }

    pub fn suppression_hits(&self) -> usize {
        self.suppression_hits
    }

    pub fn tile_equivalence_hits(&self) -> usize {
        self.tile_equivalence_hits
    }

    pub fn query_readiness_checks(&self) -> usize {
        self.query_readiness_checks
    }

    pub fn blocked_action_count(&self) -> usize {
        self.blocked_action_count
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "corpus={};frontier={};actions={};suppression={};tile={};readiness={};blocked={}",
            self.corpus_width,
            self.frontier_width,
            self.action_count,
            self.suppression_hits,
            self.tile_equivalence_hits,
            self.query_readiness_checks,
            self.blocked_action_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchCockpitReport {
    core: HadwigerArtifactCore,
    counters: ResearchCockpitCounters,
}

impl ResearchCockpitReport {
    pub(crate) fn new(
        session: &ResearchCockpitSession,
        counters: ResearchCockpitCounters,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::ResearchCockpitReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_cockpit_report".to_string(),
            },
            vec![session.reference()],
            vec![HadwigerArtifactPayloadEntry::text(
                "counters",
                counters.stable_token(),
            )],
        )?;
        Ok(Self { core, counters })
    }

    pub fn counters(&self) -> &ResearchCockpitCounters {
        &self.counters
    }
}

impl_hadwiger_artifact!(ResearchCockpitReport, core);
