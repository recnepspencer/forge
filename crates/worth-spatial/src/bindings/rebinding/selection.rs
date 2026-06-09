use crate::bindings::authority::SpatialBindingCompleteness;
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;

use super::{
    binding_snapshot::{AnchorSnapshot, BindingSnapshot},
    neighborhood::{LocalTopologyReplacementNeighborhood, ReplacementCandidate},
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CandidateContinuityRank {
    None,
    DeniedIncomplete,
    AdmittedPartial,
    CorrespondenceOnly,
    AuthoritativeSuccessor,
    Exact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LocalNeighborhoodSelection {
    continuity_rank: CandidateContinuityRank,
    selected_candidate_label: Option<String>,
    selected_candidate_identity: Option<String>,
    ambiguous: bool,
}

impl LocalNeighborhoodSelection {
    pub(crate) fn continuity_rank(&self) -> CandidateContinuityRank {
        self.continuity_rank
    }

    pub(crate) fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub(crate) fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub(crate) fn is_ambiguous(&self) -> bool {
        self.ambiguous
    }
}

pub(crate) fn select_local_rebinding_candidate(
    prior_binding: &PrimitiveRebindingPriorBindingFact,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<LocalNeighborhoodSelection, SpatialRebindingAuthorityError> {
    let prior = prior_binding.snapshot();
    let mut best_rank = CandidateContinuityRank::None;
    let mut best_candidates: Vec<&ReplacementCandidate> = Vec::new();
    for candidate in neighborhood.candidates() {
        let rank = rebinding_rank(prior, candidate.snapshot());
        if rank > best_rank {
            best_rank = rank;
            best_candidates.clear();
            best_candidates.push(candidate);
        } else if rank == best_rank {
            best_candidates.push(candidate);
        }
    }
    let ambiguous = best_rank != CandidateContinuityRank::None && best_candidates.len() > 1;
    let selected = (!ambiguous)
        .then(|| best_candidates.first().copied())
        .flatten();
    Ok(LocalNeighborhoodSelection {
        continuity_rank: best_rank,
        selected_candidate_label: selected.map(|candidate| candidate.label().to_string()),
        selected_candidate_identity: selected
            .map(|candidate| candidate.binding_identity().to_string()),
        ambiguous,
    })
}

pub(crate) fn rebinding_rank(
    prior: &BindingSnapshot,
    candidate: &BindingSnapshot,
) -> CandidateContinuityRank {
    if prior.family != candidate.family {
        return CandidateContinuityRank::None;
    }
    let completeness_rank = completeness_rank(candidate.completeness);
    if completeness_rank != CandidateContinuityRank::Exact {
        return completeness_rank;
    }
    if prior.geometry_digest == candidate.geometry_digest
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return CandidateContinuityRank::Exact;
    }
    if prior.birth_class == candidate.birth_class
        && prior.anchor.is_none()
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return CandidateContinuityRank::AuthoritativeSuccessor;
    }
    if prior.birth_class == candidate.birth_class {
        return CandidateContinuityRank::CorrespondenceOnly;
    }
    CandidateContinuityRank::AdmittedPartial
}

fn completeness_rank(completeness: SpatialBindingCompleteness) -> CandidateContinuityRank {
    match completeness {
        SpatialBindingCompleteness::Complete => CandidateContinuityRank::Exact,
        SpatialBindingCompleteness::AdmittedPartial(_) => CandidateContinuityRank::AdmittedPartial,
        SpatialBindingCompleteness::DeniedIncomplete(_) => {
            CandidateContinuityRank::DeniedIncomplete
        }
    }
}

fn same_anchor_semantics(
    prior: Option<&AnchorSnapshot>,
    candidate: Option<&AnchorSnapshot>,
) -> bool {
    match (prior, candidate) {
        (None, None) => true,
        (Some(prior), Some(candidate)) => prior.same_semantics(candidate),
        _ => false,
    }
}
