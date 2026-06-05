use crate::domain_artifacts::{
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};
use crate::domain_declarations::RejectionExplanationDeclaration;
use crate::explanations::HadwigerReusableNegativeEvidence;
use crate::query_entry::HadwigerResearchHandle;

use super::corpus::{HadwigerDiscoveryEvidenceReference, ResearchEvidenceCorpus};
use super::experiments::{
    DeadEndSignature, ExperimentBatch, ExperimentPlan, ExperimentSuppressionProof,
};
use super::frontier::{DerivedFrontierState, DiscoveryFrontier};
use super::graph_memory::{FailureScope, GraphResidentFailure};
use super::hypotheses::{InvariantCandidate, InvariantHypothesis};
use super::patterns::MotifObservation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerDiscoveryError {
    Shape(HadwigerArtifactShapeError),
    EvidenceNotInCorpus,
    NoMotifsAvailable,
}

impl From<HadwigerArtifactShapeError> for HadwigerDiscoveryError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

pub fn attach_failure_to_research_graph(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    negative_evidence: &HadwigerReusableNegativeEvidence,
    failure_scope: FailureScope,
) -> Result<GraphResidentFailure, HadwigerDiscoveryError> {
    if !corpus.has_reference(&negative_evidence.reference()) {
        return Err(HadwigerDiscoveryError::EvidenceNotInCorpus);
    }
    GraphResidentFailure::from_negative_evidence(negative_evidence, failure_scope)
        .map_err(Into::into)
}

pub fn mine_research_patterns(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
) -> Result<Vec<MotifObservation>, HadwigerDiscoveryError> {
    let mut observations = Vec::new();
    for failure in corpus.graph_resident_failures() {
        observations.push(MotifObservation::from_graph_resident_failure(failure)?);
    }
    for reference in corpus.evidence_references() {
        if let Some(artifact) = motif_source_artifact(reference) {
            observations.push(MotifObservation::from_evidence_reference(artifact)?);
        }
    }
    observations.sort_by_key(|observation| observation.reference().stable_token());
    observations.dedup();
    if observations.is_empty() {
        return Err(HadwigerDiscoveryError::NoMotifsAvailable);
    }
    Ok(observations)
}

pub fn propose_invariant_hypotheses(
    _handle: &HadwigerResearchHandle,
    _corpus: &ResearchEvidenceCorpus,
    observations: &[MotifObservation],
) -> Result<Vec<InvariantHypothesis>, HadwigerDiscoveryError> {
    let mut hypotheses = observations
        .iter()
        .map(InvariantHypothesis::from_motif)
        .collect::<Result<Vec<_>, _>>()?;
    hypotheses.sort_by_key(|hypothesis| hypothesis.reference().stable_token());
    hypotheses.dedup();
    Ok(hypotheses)
}

pub fn plan_next_experiments(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    hypotheses: &[InvariantHypothesis],
) -> Result<ExperimentBatch, HadwigerDiscoveryError> {
    let query_readiness_checks = query_readiness_checks_for_planning(handle, corpus);
    let suppression_proofs = suppression_proofs_from_corpus(corpus)?;
    let mut experiment_plans = Vec::new();
    for hypothesis in hypotheses {
        let candidate = InvariantCandidate::from_hypothesis(hypothesis)?;
        let suppression = suppression_proofs.first().cloned();
        let plan = ExperimentPlan::from_hypothesis(hypothesis, suppression)?;
        if candidate.registers_query_invariant_authority() {
            return Err(HadwigerDiscoveryError::Shape(
                HadwigerArtifactShapeError::EmptyField {
                    field: "query_invariant_authority",
                },
            ));
        }
        experiment_plans.push(plan);
    }
    ExperimentBatch::new(experiment_plans, suppression_proofs, query_readiness_checks)
        .map_err(Into::into)
}

pub fn update_discovery_frontier(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    observations: Vec<MotifObservation>,
    hypotheses: Vec<InvariantHypothesis>,
    experiment_batch: ExperimentBatch,
) -> Result<DiscoveryFrontier, HadwigerDiscoveryError> {
    DiscoveryFrontier::new(corpus, &observations, &hypotheses, experiment_batch).map_err(Into::into)
}

pub fn recompute_derived_discovery_frontier(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
) -> Result<DerivedFrontierState, HadwigerDiscoveryError> {
    let observations = mine_research_patterns(handle, corpus)?;
    let hypotheses = propose_invariant_hypotheses(handle, corpus, &observations)?;
    let experiment_batch = plan_next_experiments(handle, corpus, &hypotheses)?;
    DerivedFrontierState::new(corpus, experiment_batch, observations.len()).map_err(Into::into)
}

fn motif_source_artifact(
    reference: &HadwigerDiscoveryEvidenceReference,
) -> Option<crate::domain_artifacts::HadwigerArtifactReference> {
    match reference {
        HadwigerDiscoveryEvidenceReference::Artifact { reference }
            if matches!(
                reference.artifact_kind(),
                HadwigerArtifactKind::RejectionExplanation
                    | HadwigerArtifactKind::PartialAdmissionExplanation
                    | HadwigerArtifactKind::QueryRecoveryExplanation
                    | HadwigerArtifactKind::ReusableNegativeEvidence
            ) =>
        {
            Some(reference.clone())
        }
        _ => None,
    }
}

fn suppression_proofs_from_corpus(
    corpus: &ResearchEvidenceCorpus,
) -> Result<Vec<ExperimentSuppressionProof>, HadwigerArtifactShapeError> {
    let mut proofs = Vec::new();
    for failure in corpus.graph_resident_failures() {
        let signature = DeadEndSignature::from_graph_resident_failure(failure)?;
        proofs.push(ExperimentSuppressionProof::from_dead_end_signature(
            signature,
            failure.failure_basis_fingerprint(),
        )?);
    }
    if proofs.is_empty() && corpus.rejected_evidence_available() {
        for evidence in corpus.reusable_negative_evidence() {
            let scope = FailureScope::artifact(evidence.reference());
            let fingerprint = super::graph_memory::FailureBasisFingerprint::from_negative_evidence(
                evidence, &scope,
            )?;
            let signature =
                DeadEndSignature::from_reusable_negative_evidence(evidence, &fingerprint)?;
            proofs.push(ExperimentSuppressionProof::from_dead_end_signature(
                signature,
                &fingerprint,
            )?);
        }
    }
    proofs.sort_by_key(|proof| proof.reference().stable_token());
    proofs.dedup();
    Ok(proofs)
}

fn query_readiness_checks_for_planning(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
) -> usize {
    if corpus.has_query_recovery_evidence() {
        let _readiness = handle.declaration_entry_readiness::<RejectionExplanationDeclaration>();
        1
    } else {
        0
    }
}
