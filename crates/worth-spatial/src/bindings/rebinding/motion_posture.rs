#[cfg(test)]
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;

#[cfg(test)]
use super::SpatialRebindingAuthorityError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BindingMotionSemanticsInput {
    Move,
    Rotate { angle_radians: f64 },
    Reorient,
    InvalidatedByLocalTopologyReplacement,
    UnresolvedWithoutMotionWorkflow,
}

impl BindingMotionSemanticsInput {
    pub fn moved_with_carrier() -> Self {
        Self::Move
    }

    pub fn rotated_with_carrier(angle_radians: f64) -> Self {
        Self::Rotate { angle_radians }
    }

    pub fn reoriented_with_carrier() -> Self {
        Self::Reorient
    }

    pub fn invalidated_by_local_topology_replacement() -> Self {
        Self::InvalidatedByLocalTopologyReplacement
    }

    pub fn unresolved_without_motion_workflow() -> Self {
        Self::UnresolvedWithoutMotionWorkflow
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MotionAwareBindingPosture {
    Preserved,
    TransformedWithCarrier,
    Invalidated,
    Unresolved,
}

#[cfg(test)]
pub(crate) fn evaluate_binding_motion_posture(
    _prior_binding: &PrimitiveRebindingPriorBindingFact,
    motion: BindingMotionSemanticsInput,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    evaluate_carrier_bound_motion_posture(motion)
}

#[cfg(test)]
fn evaluate_carrier_bound_motion_posture(
    motion: BindingMotionSemanticsInput,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    match motion {
        BindingMotionSemanticsInput::Move => Ok(MotionAwareBindingPosture::TransformedWithCarrier),
        BindingMotionSemanticsInput::Rotate { angle_radians } => {
            if angle_radians.abs() <= f64::EPSILON {
                Ok(MotionAwareBindingPosture::Preserved)
            } else {
                Ok(MotionAwareBindingPosture::TransformedWithCarrier)
            }
        }
        BindingMotionSemanticsInput::Reorient => Ok(MotionAwareBindingPosture::Unresolved),
        BindingMotionSemanticsInput::InvalidatedByLocalTopologyReplacement => {
            Ok(MotionAwareBindingPosture::Invalidated)
        }
        BindingMotionSemanticsInput::UnresolvedWithoutMotionWorkflow => {
            Ok(MotionAwareBindingPosture::Unresolved)
        }
    }
}
