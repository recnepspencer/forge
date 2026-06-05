use super::{
    neighborhood::LocalTopologyReplacementNeighborhood,
    selection::{
        select_local_rebinding_candidate, CandidateContinuityRank, LocalNeighborhoodSelection,
    },
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingContinuityClass {
    Exact,
    AuthoritativeSuccessor,
    CorrespondenceOnly,
    InsufficientEvidenceFromAdmittedPartial,
    InsufficientEvidenceFromDeniedIncomplete,
    Ambiguous,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingContinuityAssessment {
    continuity_class: BindingContinuityClass,
}

impl BindingContinuityAssessment {
    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }
}

pub fn evaluate_continuity(
    prior_binding: &crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<BindingContinuityAssessment, SpatialRebindingAuthorityError> {
    let selection = select_local_rebinding_candidate(prior_binding, neighborhood)?;
    Ok(continuity_from_selection(&selection))
}

pub(crate) fn continuity_from_selection(
    selection: &LocalNeighborhoodSelection,
) -> BindingContinuityAssessment {
    let continuity_class = classify_continuity(selection);
    BindingContinuityAssessment { continuity_class }
}

fn classify_continuity(selection: &LocalNeighborhoodSelection) -> BindingContinuityClass {
    if selection.is_ambiguous() {
        return BindingContinuityClass::Ambiguous;
    }
    match selection.continuity_rank() {
        CandidateContinuityRank::Exact => BindingContinuityClass::Exact,
        CandidateContinuityRank::AuthoritativeSuccessor => {
            BindingContinuityClass::AuthoritativeSuccessor
        }
        CandidateContinuityRank::CorrespondenceOnly => BindingContinuityClass::CorrespondenceOnly,
        CandidateContinuityRank::AdmittedPartial => {
            BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
        }
        CandidateContinuityRank::DeniedIncomplete => {
            BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
        }
        CandidateContinuityRank::None => BindingContinuityClass::None,
    }
}
