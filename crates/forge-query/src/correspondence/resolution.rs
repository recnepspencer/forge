use super::candidate_set::CorrespondenceCandidateSet;
use super::contracts::{
    CorrespondenceComplexityContract, StructuralCandidateBudget,
    UniqueStructuralCorrespondenceWitness,
};
use super::cost::{
    CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract,
};
use super::counters::CorrespondenceCounterSnapshot;
use super::error::CorrespondenceEvaluationError;
use super::outcome::{
    AdvisoryStructuralAmbiguous, AdvisoryStructuralUnique, CorrespondenceDenied,
    CorrespondenceOutcome, LineageContinuity, LineageStructuralDisagreement,
};
use super::request::{
    CorrespondenceEvaluationRequest, LineageEvidenceInput, StructuralEvidenceInput,
};
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

pub fn resolve_correspondence_evidence(
    request: CorrespondenceEvaluationRequest,
) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> {
    let lineage_digest = lineage_digest_for(request.lineage_evidence());
    match (request.lineage_evidence(), request.structural_evidence()) {
        (None, None) => Err(CorrespondenceEvaluationError::MissingEvidence),
        (Some(lineage), None) => Ok(resolve_lineage_only(&request, lineage, lineage_digest)),
        (None, Some(structural)) => Ok(resolve_structural_only(
            &request,
            structural,
            lineage_digest,
        )),
        (Some(lineage), Some(structural)) => {
            resolve_mixed(&request, lineage, structural, lineage_digest)
        }
    }
}

fn resolve_lineage_only(
    request: &CorrespondenceEvaluationRequest,
    lineage: &LineageEvidenceInput,
    lineage_digest: LineageDigest,
) -> CorrespondenceEvidenceResolved {
    match lineage {
        LineageEvidenceInput::AuthoritativeContinuity {
            canonical_subject,
            authoritative_counterpart,
        } => CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::lineage_continuity(LineageContinuity::new(
                canonical_subject.clone(),
                authoritative_counterpart.clone(),
            )),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::LineageDirect,
            CorrespondenceComplexityContract::lineage_direct(),
            CorrespondenceCounterSnapshot::lineage_direct(),
        ),
        #[cfg(test)]
        LineageEvidenceInput::UnsupportedTopology { topology } => denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedTopology { topology },
            0,
            0,
            0,
        ),
    }
}

fn resolve_structural_only(
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
        StructuralEvidenceInput::UnsupportedFamily { family } => denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedStructuralFamily { family },
            0,
            1,
            0,
        ),
        #[cfg(test)]
        StructuralEvidenceInput::LineageConflict { .. } => denied_resolution(
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

fn resolve_mixed(
    request: &CorrespondenceEvaluationRequest,
    lineage: &LineageEvidenceInput,
    structural: &StructuralEvidenceInput,
    lineage_digest: LineageDigest,
) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> {
    match request.intent() {
        super::request::CorrespondenceFamilyIntent::MixedEvidenceAdmitted => {}
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
            return Ok(denied_resolution(
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
        StructuralEvidenceInput::UnsupportedFamily { family } => Ok(denied_resolution(
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

fn resolve_structural_candidates(
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
        return denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::BroadStructuralScanRequired,
            candidates.len(),
            1,
            usize::from(lineage_hint.is_some()),
        );
    }

    if candidates.len() > request.budget().max_candidates() {
        return denied_resolution(
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
        0 => denied_resolution(
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

fn resolve_lineage_vs_structural_candidates(
    request: &CorrespondenceEvaluationRequest,
    lineage_digest: LineageDigest,
    canonical_subject: String,
    authoritative_counterpart: String,
    candidates: Vec<String>,
    ordering_contract: StructuralCandidateOrderingContract,
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

    denied_resolution(
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

fn lineage_digest_for(lineage: Option<&LineageEvidenceInput>) -> LineageDigest {
    match lineage {
        Some(LineageEvidenceInput::AuthoritativeContinuity {
            canonical_subject,
            authoritative_counterpart,
        }) => LineageDigest::from_parts(&[
            "lineage:authoritative".to_string(),
            format!("subject:{canonical_subject}"),
            format!("counterpart:{authoritative_counterpart}"),
        ]),
        #[cfg(test)]
        Some(LineageEvidenceInput::UnsupportedTopology { topology }) => {
            LineageDigest::from_parts(&[
                "lineage:unsupported_topology".to_string(),
                format!("topology:{topology}"),
            ])
        }
        None => LineageDigest::from_parts(&["lineage:absent".to_string()]),
    }
}
