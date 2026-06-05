mod binding_snapshot;
mod candidate_evaluation;
mod continuity;
mod diagnostics;
mod motion_posture;
mod neighborhood;
mod outcome_classification;
mod selection;

pub use candidate_evaluation::{evaluate_replacement_candidates, ReplacementCandidateEvaluation};
pub use continuity::{evaluate_continuity, BindingContinuityAssessment, BindingContinuityClass};
pub use diagnostics::RebindingExplanation;
pub use motion_posture::{
    evaluate_binding_motion_posture, BindingMotionSemanticsInput, MotionAwareBindingPosture,
};
pub use neighborhood::{
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet,
};
pub use outcome_classification::{
    AdmittedRebindingDecision, RebindingOutcomeClass, UnsupportedRebindingReason,
};

use crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding;
use crate::bindings::authority::SpatialBindingKind;

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
    if let Some(decision) = unsupported_rebinding_decision(
        NeighborhoodBindingFamily::FaceSurface,
        NeighborhoodBindingFamily::supports_face_surface_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(decision);
    }
    rebind_binding(prior_binding, neighborhood)
}

pub fn rebind_curve_on_edge(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    if let Some(decision) = unsupported_rebinding_decision(
        NeighborhoodBindingFamily::EdgeCurve,
        NeighborhoodBindingFamily::supports_edge_curve_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(decision);
    }
    rebind_binding(prior_binding, neighborhood)
}

pub fn rebind_pcurve_on_coedge(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    if let Some(decision) = unsupported_rebinding_decision(
        NeighborhoodBindingFamily::CoedgePCurve,
        NeighborhoodBindingFamily::supports_coedge_pcurve_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(decision);
    }
    rebind_binding(prior_binding, neighborhood)
}

pub fn rebind_geometry_on_vertex(
    prior_binding: SpatialAdmittedPrimitiveBinding,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<AdmittedRebindingDecision, SpatialRebindingAuthorityError> {
    if let Some(decision) = unsupported_rebinding_decision(
        NeighborhoodBindingFamily::VertexGeometry,
        NeighborhoodBindingFamily::supports_vertex_geometry_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(decision);
    }
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
    let motion_posture = evaluate_motion_posture_internal(
        &prior_binding,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )?;
    Ok(AdmittedRebindingDecision::new(&evaluation, motion_posture))
}

fn unsupported_rebinding_decision(
    requested_family: NeighborhoodBindingFamily,
    predicate: fn(NeighborhoodBindingFamily) -> bool,
    prior_binding: &SpatialAdmittedPrimitiveBinding,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<Option<AdmittedRebindingDecision>, SpatialRebindingAuthorityError> {
    let prior_family = NeighborhoodBindingFamily::from_binding(prior_binding)?;
    if !predicate(prior_family) {
        return Ok(Some(AdmittedRebindingDecision::unsupported(
            prior_binding,
            neighborhood,
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: requested_family,
                actual: prior_family,
            },
        )));
    }
    if neighborhood.family() != prior_family {
        return Err(SpatialRebindingAuthorityError::NeighborhoodFamilyMismatch {
            expected: prior_family,
            actual: neighborhood.family(),
        });
    }
    Ok(None)
}
