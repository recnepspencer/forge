use crate::candidate_screening::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use crate::domain_artifacts::core_artifact::{
    canonical_digest_token, impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::generated_pattern_replay_suites::GeneratedPatternReplaySuite;
use super::periodic_quotient_cells::PeriodicQuotientCell;
use super::replay_counters::GeneratedPatternReplayCounters;
use super::replay_errors::GeneratedPatternReplayError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternReplayBlocker {
    reason: String,
    evidence_digest: String,
}

impl GeneratedPatternReplayBlocker {
    pub(crate) fn rejected(evaluation: &CandidateScreeningEvaluation) -> Self {
        Self {
            reason: evaluation.evidence().to_string(),
            evidence_digest: canonical_digest_token(evaluation.artifact_digest().canonical()),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternReplayReport {
    core: HadwigerArtifactCore,
    evaluations: Vec<CandidateScreeningEvaluation>,
    blockers: Vec<GeneratedPatternReplayBlocker>,
    counters: GeneratedPatternReplayCounters,
    query_declaration_digest: String,
}

impl GeneratedPatternReplayReport {
    pub(crate) fn checked(
        suite: &GeneratedPatternReplaySuite,
        evaluations: Vec<CandidateScreeningEvaluation>,
        counters: GeneratedPatternReplayCounters,
        query_declaration_digest: String,
    ) -> Result<Self, GeneratedPatternReplayError> {
        let blockers = replay_blockers(&evaluations);
        let evidence = replay_evidence(
            &evaluations,
            &blockers,
            &counters,
            &query_declaration_digest,
        );
        let mut parents = vec![suite.reference()];
        parents.extend(
            evaluations
                .iter()
                .map(CandidateScreeningEvaluation::reference),
        );
        let core = artifact_core(
            HadwigerArtifactKind::GeneratedPatternReplayReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "generated_pattern_replay_report".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "WORTH.hadwiger.generated_pattern_replay_report.v1",
                ),
                HadwigerArtifactPayloadEntry::text("suite", suite.reference().stable_token()),
                HadwigerArtifactPayloadEntry::text("evidence", evidence),
            ],
        )?;
        Ok(Self {
            core,
            evaluations,
            blockers,
            counters,
            query_declaration_digest,
        })
    }

    pub fn evaluations(&self) -> &[CandidateScreeningEvaluation] {
        &self.evaluations
    }

    pub fn blockers(&self) -> &[GeneratedPatternReplayBlocker] {
        &self.blockers
    }

    pub fn counters(&self) -> &GeneratedPatternReplayCounters {
        &self.counters
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn has_rejected_generated_rule(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn reusable_negative_evidence(&self) -> Option<&GeneratedPatternReplayBlocker> {
        self.blockers.first()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(GeneratedPatternReplayReport, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientReplayChecked {
    quotient_cell: PeriodicQuotientCell,
    report: GeneratedPatternReplayReport,
}

impl PeriodicQuotientReplayChecked {
    pub(crate) fn new(
        quotient_cell: PeriodicQuotientCell,
        report: GeneratedPatternReplayReport,
    ) -> Self {
        Self {
            quotient_cell,
            report,
        }
    }

    pub fn periodic_quotient_cell(&self) -> &PeriodicQuotientCell {
        &self.quotient_cell
    }

    pub fn periodic_quotient_report(&self) -> &GeneratedPatternReplayReport {
        &self.report
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.report.counters().query_declarations_performed()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternReplayChecked {
    suite: GeneratedPatternReplaySuite,
    report: GeneratedPatternReplayReport,
}

impl GeneratedPatternReplayChecked {
    pub(crate) fn new(
        suite: GeneratedPatternReplaySuite,
        report: GeneratedPatternReplayReport,
    ) -> Self {
        Self { suite, report }
    }

    pub fn suite(&self) -> &GeneratedPatternReplaySuite {
        &self.suite
    }

    pub fn report(&self) -> &GeneratedPatternReplayReport {
        &self.report
    }

    pub fn has_rejected_generated_rule(&self) -> bool {
        self.report.has_rejected_generated_rule()
    }

    pub fn reusable_negative_evidence(&self) -> Option<&GeneratedPatternReplayBlocker> {
        self.report.reusable_negative_evidence()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

fn replay_blockers(
    evaluations: &[CandidateScreeningEvaluation],
) -> Vec<GeneratedPatternReplayBlocker> {
    evaluations
        .iter()
        .filter(|evaluation| matches!(evaluation.verdict(), CandidateScreeningVerdict::Rejected))
        .map(GeneratedPatternReplayBlocker::rejected)
        .collect()
}

fn replay_evidence(
    evaluations: &[CandidateScreeningEvaluation],
    blockers: &[GeneratedPatternReplayBlocker],
    counters: &GeneratedPatternReplayCounters,
    query_declaration_digest: &str,
) -> String {
    let evaluation_tokens = evaluations
        .iter()
        .map(|evaluation| canonical_digest_token(evaluation.artifact_digest().canonical()))
        .collect::<Vec<_>>()
        .join("|");
    let blocker_tokens = blockers
        .iter()
        .map(GeneratedPatternReplayBlocker::evidence_digest)
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "query_declaration_digest={query_declaration_digest};evaluations={evaluation_tokens};blockers={blocker_tokens};counters={}",
        counters.stable_token()
    )
}
