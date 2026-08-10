mod lineage;
mod mixed;
mod structural;

use super::contracts::{CorrespondenceComplexityContract, StructuralCandidateBudget};
use super::cost::{CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan};
use super::counters::CorrespondenceCounterSnapshot;
use super::error::CorrespondenceEvaluationError;
use super::outcome::{CorrespondenceDenied, CorrespondenceOutcome};
use super::request::CorrespondenceEvaluationRequest;
use crate::identity::LineageDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceEvidenceResolved {
    lineage_digest: LineageDigest,
    outcome: CorrespondenceOutcome,
    discovery_plan: StructuralCandidateDiscoveryPlan,
    budget: StructuralCandidateBudget,
    cost_posture: CorrespondenceCostPosture,
    complexity_contract: CorrespondenceComplexityContract,
    counters: CorrespondenceCounterSnapshot,
}

impl CorrespondenceEvidenceResolved {
    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn outcome(&self) -> &CorrespondenceOutcome {
        &self.outcome
    }

    pub fn discovery_plan(&self) -> &StructuralCandidateDiscoveryPlan {
        &self.discovery_plan
    }

    pub fn budget(&self) -> &StructuralCandidateBudget {
        &self.budget
    }

    pub fn cost_posture(&self) -> &CorrespondenceCostPosture {
        &self.cost_posture
    }

    pub fn complexity_contract(&self) -> &CorrespondenceComplexityContract {
        &self.complexity_contract
    }

    pub fn counters(&self) -> &CorrespondenceCounterSnapshot {
        &self.counters
    }

    pub(crate) fn new(
        lineage_digest: LineageDigest,
        outcome: CorrespondenceOutcome,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
        cost_posture: CorrespondenceCostPosture,
        complexity_contract: CorrespondenceComplexityContract,
        counters: CorrespondenceCounterSnapshot,
    ) -> Self {
        Self {
            lineage_digest,
            outcome,
            discovery_plan,
            budget,
            cost_posture,
            complexity_contract,
            counters,
        }
    }
}

pub(crate) fn resolve_correspondence_evidence(
    request: CorrespondenceEvaluationRequest,
) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> {
    let lineage_digest = lineage::lineage_digest_for(request.lineage_evidence());
    match (request.lineage_evidence(), request.structural_evidence()) {
        (None, None) => Err(CorrespondenceEvaluationError::MissingEvidence),
        (Some(lineage), None) => Ok(lineage::resolve_lineage_only(
            &request,
            lineage,
            lineage_digest,
        )),
        (None, Some(structural)) => Ok(structural::resolve_structural_only(
            &request,
            structural,
            lineage_digest,
        )),
        (Some(lineage), Some(structural)) => {
            mixed::resolve_mixed(&request, lineage, structural, lineage_digest)
        }
    }
}

fn denied_resolution(
    request: &CorrespondenceEvaluationRequest,
    lineage_digest: LineageDigest,
    error: CorrespondenceEvaluationError,
    predicted_structural_candidate_count: usize,
    structural_candidate_rejection_count: usize,
    structural_authority_promotion_denial_count: usize,
) -> CorrespondenceEvidenceResolved {
    CorrespondenceEvidenceResolved::new(
        lineage_digest,
        CorrespondenceOutcome::denied(CorrespondenceDenied::new(
            error.denial_posture(),
            error.reason(),
        )),
        request.discovery_plan().clone(),
        request.budget().clone(),
        error.denial_posture(),
        CorrespondenceComplexityContract::denied(),
        CorrespondenceCounterSnapshot::denied(
            predicted_structural_candidate_count,
            structural_candidate_rejection_count,
            structural_authority_promotion_denial_count,
        ),
    )
}
