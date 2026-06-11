pub(crate) mod binding_snapshot;
#[cfg(test)]
mod candidate_evaluation;
mod continuity;
#[cfg(test)]
mod diagnostics;
mod motion_posture;
mod neighborhood;
mod outcome_classification;
#[cfg(test)]
mod selection;

#[cfg(test)]
pub use candidate_evaluation::ReplacementCandidateEvaluation;
pub use continuity::{BindingContinuityAssessment, BindingContinuityClass};
pub use motion_posture::{BindingMotionSemanticsInput, MotionAwareBindingPosture};
pub use neighborhood::{
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet,
};
pub use outcome_classification::{
    PrimitiveRebindingFactReceipt, PrimitiveRebindingRetainedFactSource, RebindingOutcomeClass,
    UnsupportedRebindingReason,
};

use crate::bindings::authority::SpatialBindingKind;
#[cfg(test)]
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;

#[cfg(test)]
use self::candidate_evaluation::evaluate_replacement_candidates;
#[cfg(test)]
use self::motion_posture::evaluate_binding_motion_posture as evaluate_motion_posture_internal;
#[cfg(test)]
use self::outcome_classification::{
    rebinding_fact_receipt_from_evaluation, unsupported_rebinding_fact_receipt,
};

#[cfg(test)]
pub(crate) fn evaluate_continuity_internal(
    prior_binding: &PrimitiveRebindingPriorBindingFact,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<BindingContinuityAssessment, SpatialRebindingAuthorityError> {
    continuity::evaluate_continuity(prior_binding, neighborhood)
}

#[cfg(test)]
pub(crate) fn evaluate_replacement_candidates_internal(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> Result<ReplacementCandidateEvaluation, SpatialRebindingAuthorityError> {
    candidate_evaluation::evaluate_replacement_candidates(prior_binding, neighborhood)
}

#[cfg(test)]
pub(crate) fn evaluate_binding_motion_posture_internal(
    prior_binding: &PrimitiveRebindingPriorBindingFact,
    motion: BindingMotionSemanticsInput,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    motion_posture::evaluate_binding_motion_posture(prior_binding, motion)
}

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

#[cfg(test)]
pub(crate) fn project_surface_rebinding_fact_receipt_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    if let Some(receipt) = unsupported_rebinding_fact_receipt_if_any(
        NeighborhoodBindingFamily::FaceSurface,
        NeighborhoodBindingFamily::supports_face_surface_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(receipt);
    }
    build_rebinding_fact_receipt(prior_binding, neighborhood, motion)
}

#[cfg(test)]
pub(crate) fn project_curve_rebinding_fact_receipt_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    if let Some(receipt) = unsupported_rebinding_fact_receipt_if_any(
        NeighborhoodBindingFamily::EdgeCurve,
        NeighborhoodBindingFamily::supports_edge_curve_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(receipt);
    }
    build_rebinding_fact_receipt(prior_binding, neighborhood, motion)
}

#[cfg(test)]
pub(crate) fn project_pcurve_rebinding_fact_receipt_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    if let Some(receipt) = unsupported_rebinding_fact_receipt_if_any(
        NeighborhoodBindingFamily::CoedgePCurve,
        NeighborhoodBindingFamily::supports_coedge_pcurve_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(receipt);
    }
    build_rebinding_fact_receipt(prior_binding, neighborhood, motion)
}

#[cfg(test)]
pub(crate) fn project_geometry_rebinding_fact_receipt_with_motion(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    if let Some(receipt) = unsupported_rebinding_fact_receipt_if_any(
        NeighborhoodBindingFamily::VertexGeometry,
        NeighborhoodBindingFamily::supports_vertex_geometry_rebinding,
        &prior_binding,
        &neighborhood,
    )? {
        return Ok(receipt);
    }
    build_rebinding_fact_receipt(prior_binding, neighborhood, motion)
}

#[cfg(test)]
fn build_rebinding_fact_receipt(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    let evaluation = evaluate_replacement_candidates(prior_binding.clone(), neighborhood.clone())?;
    let motion_posture = evaluate_motion_posture_internal(&prior_binding, motion)?;
    Ok(rebinding_fact_receipt_from_evaluation(
        &evaluation,
        motion_posture,
    ))
}

#[cfg(test)]
fn unsupported_rebinding_fact_receipt_if_any(
    requested_family: NeighborhoodBindingFamily,
    predicate: fn(NeighborhoodBindingFamily) -> bool,
    prior_binding: &PrimitiveRebindingPriorBindingFact,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<Option<PrimitiveRebindingFactReceipt>, SpatialRebindingAuthorityError> {
    let prior_family = prior_binding.family();
    if !predicate(prior_family) {
        return Ok(Some(unsupported_rebinding_fact_receipt(
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
