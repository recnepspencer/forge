use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerArtifactDigest, HadwigerCanonicalArtifact};

use super::corpus::ResearchEvidenceCorpus;
use super::experiments::{ExperimentBatch, ExperimentPlan};
use super::hypotheses::InvariantHypothesis;
use super::patterns::MotifObservation;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct HadwigerDiscoveryCounters {
    candidate_breadth: usize,
    motif_observations: usize,
    suppression_hits: usize,
    retired_hypotheses: usize,
    reactivation_hits: usize,
    skipped_unsupported_work: usize,
    checker_budget_blocks: usize,
    query_readiness_checks: usize,
}

impl HadwigerDiscoveryCounters {
    pub(crate) fn new(
        candidate_breadth: usize,
        motif_observations: usize,
        suppression_hits: usize,
        query_readiness_checks: usize,
    ) -> Self {
        Self {
            candidate_breadth,
            motif_observations,
            suppression_hits,
            query_readiness_checks,
            ..Self::default()
        }
    }

    pub fn candidate_breadth(&self) -> usize {
        self.candidate_breadth
    }

    pub fn motif_observations(&self) -> usize {
        self.motif_observations
    }

    pub fn suppression_hits(&self) -> usize {
        self.suppression_hits
    }

    pub fn retired_hypotheses(&self) -> usize {
        self.retired_hypotheses
    }

    pub fn reactivation_hits(&self) -> usize {
        self.reactivation_hits
    }

    pub fn skipped_unsupported_work(&self) -> usize {
        self.skipped_unsupported_work
    }

    pub fn checker_budget_blocks(&self) -> usize {
        self.checker_budget_blocks
    }

    pub fn query_readiness_checks(&self) -> usize {
        self.query_readiness_checks
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryScorecard {
    counters: HadwigerDiscoveryCounters,
}

impl DiscoveryScorecard {
    pub(crate) fn new(counters: HadwigerDiscoveryCounters) -> Self {
        Self { counters }
    }

    pub fn counters(&self) -> &HadwigerDiscoveryCounters {
        &self.counters
    }

    pub fn suppression_hits(&self) -> usize {
        self.counters.suppression_hits()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryFrontier {
    core: HadwigerArtifactCore,
    scorecard: DiscoveryScorecard,
    experiment_batch: ExperimentBatch,
}

impl DiscoveryFrontier {
    pub(crate) fn new(
        corpus: &ResearchEvidenceCorpus,
        motif_observations: &[MotifObservation],
        hypotheses: &[InvariantHypothesis],
        experiment_batch: ExperimentBatch,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let counters = HadwigerDiscoveryCounters::new(
            hypotheses.len(),
            motif_observations.len(),
            experiment_batch.suppression_proofs().len(),
            experiment_batch.query_readiness_checks(),
        );
        let scorecard = DiscoveryScorecard::new(counters);
        let core = artifact_core(
            HadwigerArtifactKind::DiscoveryFrontier,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "discovery_frontier".to_string(),
            },
            vec![corpus.reference(), experiment_batch.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "corpus_digest",
                    corpus.corpus_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "motif_observations",
                    motif_observations.len() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned("hypotheses", hypotheses.len() as u128),
                HadwigerArtifactPayloadEntry::unsigned(
                    "suppression_hits",
                    scorecard.suppression_hits() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "query_readiness_checks",
                    scorecard.counters().query_readiness_checks() as u128,
                ),
            ],
        )?;
        Ok(Self {
            core,
            scorecard,
            experiment_batch,
        })
    }

    pub fn scorecard(&self) -> &DiscoveryScorecard {
        &self.scorecard
    }

    pub fn experiment_plans(&self) -> &[ExperimentPlan] {
        self.experiment_batch.experiment_plans()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(DiscoveryFrontier, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DerivedFrontierState {
    core: HadwigerArtifactCore,
    source_corpus_digest: HadwigerArtifactDigest,
    counters: HadwigerDiscoveryCounters,
    experiment_plans: Vec<ExperimentPlan>,
    rejected_evidence_available: bool,
}

impl DerivedFrontierState {
    pub(crate) fn new(
        corpus: &ResearchEvidenceCorpus,
        experiment_batch: ExperimentBatch,
        motif_count: usize,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let counters = HadwigerDiscoveryCounters::new(
            experiment_batch.experiment_plans().len(),
            motif_count,
            experiment_batch.suppression_proofs().len(),
            experiment_batch.query_readiness_checks(),
        );
        let rejected_evidence_available = corpus.rejected_evidence_available();
        let experiment_plans = experiment_batch.experiment_plans().to_vec();
        let core = artifact_core(
            HadwigerArtifactKind::DerivedFrontierState,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "derived_discovery_frontier_state".to_string(),
            },
            vec![corpus.reference(), experiment_batch.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "source_corpus_digest",
                    corpus.corpus_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "candidate_breadth",
                    counters.candidate_breadth() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "suppression_hits",
                    counters.suppression_hits() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "query_readiness_checks",
                    counters.query_readiness_checks() as u128,
                ),
                HadwigerArtifactPayloadEntry::text(
                    "rejected_evidence_available",
                    rejected_evidence_available.to_string(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            source_corpus_digest: corpus.corpus_digest().clone(),
            counters,
            experiment_plans,
            rejected_evidence_available,
        })
    }

    pub fn source_corpus_digest(&self) -> &HadwigerArtifactDigest {
        &self.source_corpus_digest
    }

    pub fn counters(&self) -> &HadwigerDiscoveryCounters {
        &self.counters
    }

    pub fn experiment_plans(&self) -> &[ExperimentPlan] {
        &self.experiment_plans
    }

    pub fn rejected_evidence_available(&self) -> bool {
        self.rejected_evidence_available
    }
}

impl_hadwiger_artifact!(DerivedFrontierState, core);
