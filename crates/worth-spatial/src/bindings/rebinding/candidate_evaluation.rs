use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;

use super::{
    continuity::{continuity_from_selection, BindingContinuityAssessment},
    neighborhood::LocalTopologyReplacementNeighborhood,
    selection::{select_local_rebinding_candidate, LocalNeighborhoodSelection},
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReplacementCandidateEvaluation {
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    selection: LocalNeighborhoodSelection,
    continuity: BindingContinuityAssessment,
}

impl ReplacementCandidateEvaluation {
    pub fn prior_binding(&self) -> &PrimitiveRebindingPriorBindingFact {
        &self.prior_binding
    }

    pub fn neighborhood(&self) -> &LocalTopologyReplacementNeighborhood {
        &self.neighborhood
    }

    pub(crate) fn selection(&self) -> &LocalNeighborhoodSelection {
        &self.selection
    }

    pub fn continuity(&self) -> &BindingContinuityAssessment {
        &self.continuity
    }
}

pub(crate) fn evaluate_replacement_candidates(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<ReplacementCandidateEvaluation, SpatialRebindingAuthorityError> {
    let selection = select_local_rebinding_candidate(&prior_binding, &neighborhood)?;
    let continuity = continuity_from_selection(&selection);
    Ok(ReplacementCandidateEvaluation {
        prior_binding,
        neighborhood,
        selection,
        continuity,
    })
}
