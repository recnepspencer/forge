use super::super::candidate_set::CorrespondenceCandidateSet;
use super::super::contracts::{
    CorrespondenceComplexityContract, UniqueStructuralCorrespondenceWitness,
};
use super::super::cost::{
    CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract,
};
use super::super::counters::CorrespondenceCounterSnapshot;
use super::super::error::CorrespondenceEvaluationError;
use super::super::outcome::{
    AdvisoryStructuralAmbiguous, AdvisoryStructuralUnique, CorrespondenceOutcome,
};
use super::super::request::{CorrespondenceEvaluationRequest, StructuralEvidenceInput};
use super::CorrespondenceEvidenceResolved;
use crate::identity::LineageDigest;

pub(super) fn resolve_structural_only(
    request: &CorrespondenceEvaluationRequest,
    structural: &StructuralEvidenceInput,
    lineage_digest: LineageDigest,
) -> CorrespondenceEvidenceResolved {
    match structural {
        StructuralEvidenceInput::CandidateSet {
            candidates,
            ordering_contract,
        } => resolve_structural_candidates(
            request,
            lineage_digest,
            None,
            candidates.clone(),
            ordering_contract.clone(),
        ),
        #[cfg(test)]
        StructuralEvidenceInput::UnsupportedFamily { family } => super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedStructuralFamily { family },
            0,
            1,
            0,
        ),
        #[cfg(test)]
        StructuralEvidenceInput::LineageConflict { .. } => super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedMixedEvidence {
                reason: "lineage conflict evidence requires authoritative lineage context",
            },
            0,
            1,
            1,
        ),
    }
}

pub(super) fn resolve_structural_candidates(
    request: &CorrespondenceEvaluationRequest,
    lineage_digest: LineageDigest,
    lineage_hint: Option<(&str, &str)>,
    candidates: Vec<String>,
    ordering_contract: StructuralCandidateOrderingContract,
) -> CorrespondenceEvidenceResolved {
    if matches!(
        request.discovery_plan(),
        StructuralCandidateDiscoveryPlan::RequiresBroadScanDenied
    ) {
        return super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::BroadStructuralScanRequired,
            candidates.len(),
            1,
            usize::from(lineage_hint.is_some()),
        );
    }

    if candidates.len() > request.budget().max_candidates() {
        return super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::StructuralBreadthExceeded {
                budget: request.budget().max_candidates(),
                actual: candidates.len(),
            },
            candidates.len(),
            1,
            usize::from(lineage_hint.is_some()),
        );
    }

    let candidate_set = CorrespondenceCandidateSet::new(
        candidates.clone(),
        request.discovery_plan().clone(),
        request.budget().clone(),
        ordering_contract,
    );

    match candidates.len() {
        0 => super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedMixedEvidence {
                reason: "structural evidence did not yield any candidate within the planned scope",
            },
            0,
            1,
            usize::from(lineage_hint.is_some()),
        ),
        1 => CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::advisory_structural_unique(AdvisoryStructuralUnique::new(
                candidates[0].clone(),
                UniqueStructuralCorrespondenceWitness::new(),
            )),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::StructuralCandidateBounded,
            CorrespondenceComplexityContract::structural_candidate_bounded(),
            CorrespondenceCounterSnapshot::structural_unique(1),
        ),
        count => CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::advisory_structural_ambiguous(AdvisoryStructuralAmbiguous::new(
                candidate_set,
            )),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::StructuralAmbiguityBounded,
            CorrespondenceComplexityContract::structural_ambiguity_bounded(),
            CorrespondenceCounterSnapshot::structural_ambiguous(count),
        ),
    }
}
