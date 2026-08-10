use super::super::contracts::CorrespondenceComplexityContract;
use super::super::cost::CorrespondenceCostPosture;
use super::super::counters::CorrespondenceCounterSnapshot;
use super::super::error::CorrespondenceEvaluationError;
use super::super::outcome::{
    CorrespondenceOutcome, LineageContinuity, LineageStructuralDisagreement,
};
use super::super::request::{
    CorrespondenceEvaluationRequest, CorrespondenceFamilyIntent, LineageEvidenceInput,
    StructuralEvidenceInput,
};
use super::structural::resolve_structural_candidates;
use super::CorrespondenceEvidenceResolved;
use crate::identity::LineageDigest;

pub(super) fn resolve_mixed(
    request: &CorrespondenceEvaluationRequest,
    lineage: &LineageEvidenceInput,
    structural: &StructuralEvidenceInput,
    lineage_digest: LineageDigest,
) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> {
    match request.intent() {
        CorrespondenceFamilyIntent::MixedEvidenceAdmitted => {}
        _ => {
            return Err(CorrespondenceEvaluationError::UnsupportedMixedEvidence {
                reason: "mixed evidence was not admitted by the request intent",
            });
        }
    }

    let (canonical_subject, authoritative_counterpart) = match lineage {
        LineageEvidenceInput::AuthoritativeContinuity {
            canonical_subject,
            authoritative_counterpart,
        } => (canonical_subject.clone(), authoritative_counterpart.clone()),
        #[cfg(test)]
        LineageEvidenceInput::UnsupportedTopology { topology } => {
            return Ok(super::denied_resolution(
                request,
                lineage_digest,
                CorrespondenceEvaluationError::UnsupportedTopology { topology },
                0,
                0,
                0,
            ));
        }
    };

    match structural {
        StructuralEvidenceInput::CandidateSet {
            candidates,
            ordering_contract,
        } => Ok(resolve_lineage_vs_structural_candidates(
            request,
            lineage_digest,
            canonical_subject,
            authoritative_counterpart,
            candidates.clone(),
            ordering_contract.clone(),
        )),
        #[cfg(test)]
        StructuralEvidenceInput::UnsupportedFamily { family } => Ok(super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedStructuralFamily { family },
            0,
            1,
            1,
        )),
        #[cfg(test)]
        StructuralEvidenceInput::LineageConflict {
            structural_counterpart,
        } => Ok(CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::lineage_structural_disagreement(
                LineageStructuralDisagreement::new(
                    authoritative_counterpart,
                    structural_counterpart.clone(),
                ),
            ),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::StructuralCandidateBounded,
            CorrespondenceComplexityContract::lineage_structural_disagreement(),
            CorrespondenceCounterSnapshot::disagreement(1),
        )),
    }
}

fn resolve_lineage_vs_structural_candidates(
    request: &CorrespondenceEvaluationRequest,
    lineage_digest: LineageDigest,
    canonical_subject: String,
    authoritative_counterpart: String,
    candidates: Vec<String>,
    ordering_contract: super::super::cost::StructuralCandidateOrderingContract,
) -> CorrespondenceEvidenceResolved {
    if candidates.len() == 1 && candidates[0] == authoritative_counterpart {
        return CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::lineage_continuity(LineageContinuity::new(
                canonical_subject,
                authoritative_counterpart,
            )),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::LineageDirect,
            CorrespondenceComplexityContract::lineage_direct(),
            CorrespondenceCounterSnapshot::lineage_direct(),
        );
    }

    if candidates
        .iter()
        .any(|candidate| candidate == &authoritative_counterpart)
        && candidates.len() > 1
    {
        return resolve_structural_candidates(
            request,
            lineage_digest,
            Some((&canonical_subject, &authoritative_counterpart)),
            candidates,
            ordering_contract,
        );
    }

    if let Some(structural_counterpart) = candidates.first() {
        return CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::lineage_structural_disagreement(
                LineageStructuralDisagreement::new(
                    authoritative_counterpart,
                    structural_counterpart.clone(),
                ),
            ),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::StructuralCandidateBounded,
            CorrespondenceComplexityContract::lineage_structural_disagreement(),
            CorrespondenceCounterSnapshot::disagreement(candidates.len().max(1)),
        );
    }

    super::denied_resolution(
        request,
        lineage_digest,
        CorrespondenceEvaluationError::UnsupportedMixedEvidence {
            reason:
                "mixed evidence did not yield any structural candidate to compare against lineage",
        },
        0,
        1,
        1,
    )
}
