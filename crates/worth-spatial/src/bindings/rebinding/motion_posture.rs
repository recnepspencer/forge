use crate::bindings::authority::SpatialAdmittedPrimitiveBinding;

use super::{
    neighborhood::{LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily},
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MotionAwareBindingPosture {
    Preserved,
    RequiresRebinding,
    Invalidated,
}

pub fn evaluate_binding_motion_posture(
    prior_binding: &SpatialAdmittedPrimitiveBinding,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    let prior_family = NeighborhoodBindingFamily::from_binding(prior_binding)?;
    debug_assert_eq!(neighborhood.family(), prior_family);
    if neighborhood
        .candidates()
        .iter()
        .any(|candidate| candidate.binding().identity() == prior_binding.identity())
    {
        return Ok(MotionAwareBindingPosture::Preserved);
    }
    if neighborhood.candidates().is_empty() {
        return Ok(MotionAwareBindingPosture::Invalidated);
    }
    Ok(MotionAwareBindingPosture::RequiresRebinding)
}
