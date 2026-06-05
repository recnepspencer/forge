mod candidate_evaluation;
mod continuity;
mod diagnostics;
mod motion_posture;
mod neighborhood;
mod outcome_classification;

pub use candidate_evaluation::{evaluate_replacement_candidates, ReplacementCandidateEvaluation};
pub use continuity::{evaluate_continuity, BindingContinuityAssessment, BindingContinuityClass};
pub use diagnostics::RebindingExplanation;
pub use motion_posture::{evaluate_binding_motion_posture, MotionAwareBindingPosture};
pub use neighborhood::{
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet,
};
pub use outcome_classification::{AdmittedRebindingDecision, RebindingOutcomeClass};

use crate::bindings::authority::{SpatialAdmittedPrimitiveBinding, SpatialBindingKind};

use self::motion_posture::evaluate_binding_motion_posture as evaluate_motion_posture_internal;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialRebindingAuthorityError {
    UnsupportedBindingKind(SpatialBindingKind),
    MissingReplacementLabel,
    MissingPriorSiteIdentity,
    CandidateSetEmpty,
    CandidateFamilyMismatch {
        expected: NeighborhoodBindingFamily,
        actual: NeighborhoodBindingFamily,
    },
    NeighborhoodFamilyMismatch {
        expected: NeighborhoodBindingFamily,
        actual: NeighborhoodBindingFamily,
    },
}

pub fn rebind_surface_on_face(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    ensure_expected_rebinding_family(
        NeighborhoodBindingFamily::supports_face_surface_rebinding,
        &prior_binding,
        &neighborhood,
    )?;
    rebind_binding(prior_binding, neighborhood)
}

pub fn rebind_curve_on_edge(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    ensure_expected_rebinding_family(
        NeighborhoodBindingFamily::supports_edge_curve_rebinding,
        &prior_binding,
        &neighborhood,
    )?;
    rebind_binding(prior_binding, neighborhood)
}

pub fn rebind_pcurve_on_coedge(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    ensure_expected_rebinding_family(
        NeighborhoodBindingFamily::supports_coedge_pcurve_rebinding,
        &prior_binding,
        &neighborhood,
    )?;
    rebind_binding(prior_binding, neighborhood)
}

pub fn explain_rebinding_decision(decision: &AdmittedRebindingDecision) -> RebindingExplanation {
    decision.explanation().clone()
}

fn rebind_binding(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    let evaluation = evaluate_replacement_candidates(prior_binding.clone(), neighborhood.clone())?;
    let motion_posture = evaluate_motion_posture_internal(&prior_binding, &neighborhood)?;
    Ok(AdmittedRebindingDecision::new(&evaluation, motion_posture))
}

fn ensure_expected_rebinding_family(
    predicate: fn(NeighborhoodBindingFamily) -> bool,
    prior_binding: &SpatialAdmittedPrimitiveBinding,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<(), SpatialRebindingAuthorityError> {
    let prior_family = NeighborhoodBindingFamily::from_binding(prior_binding)?;
    if !predicate(prior_family) {
        return Err(SpatialRebindingAuthorityError::NeighborhoodFamilyMismatch {
            expected: neighborhood.family(),
            actual: prior_family,
        });
    }
    if neighborhood.family() != prior_family {
        return Err(SpatialRebindingAuthorityError::NeighborhoodFamilyMismatch {
            expected: prior_family,
            actual: neighborhood.family(),
        });
    }
    Ok(())
}
