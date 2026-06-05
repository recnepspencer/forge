use crate::bindings::authority::SpatialAdmittedPrimitiveBinding;

use super::{
    continuity::{evaluate_continuity, BindingContinuityAssessment},
    neighborhood::LocalTopologyReplacementNeighborhood,
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReplacementCandidateEvaluation {
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
    continuity: BindingContinuityAssessment,
}

impl ReplacementCandidateEvaluation {
    pub fn prior_binding(&self) -> &SpatialAdmittedPrimitiveBinding {
        &self.prior_binding
    }

    pub fn neighborhood(&self) -> &LocalTopologyReplacementNeighborhood {
        &self.neighborhood
    }

    pub fn continuity(&self) -> &BindingContinuityAssessment {
        &self.continuity
    }
}

pub fn evaluate_replacement_candidates(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<ReplacementCandidateEvaluation, SpatialRebindingAuthorityError> {
    let continuity = evaluate_continuity(&prior_binding, &neighborhood)?;
    Ok(ReplacementCandidateEvaluation {
        prior_binding,
        neighborhood,
        continuity,
    })
}
