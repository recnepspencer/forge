use crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding;
use crate::spatial_intent::{AdmittedSpatialMove, AdmittedSpatialReorient, AdmittedSpatialRotate};

use super::SpatialRebindingAuthorityError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BindingMotionSemanticsInput<'a> {
    Move(&'a AdmittedSpatialMove),
    Rotate(&'a AdmittedSpatialRotate),
    Reorient(&'a AdmittedSpatialReorient),
    InvalidatedByLocalTopologyReplacement,
    UnresolvedWithoutMotionWorkflow,
}

impl<'a> BindingMotionSemanticsInput<'a> {
    pub fn for_move(motion: &'a AdmittedSpatialMove) -> Self {
        Self::Move(motion)
    }

    pub fn for_rotate(motion: &'a AdmittedSpatialRotate) -> Self {
        Self::Rotate(motion)
    }

    pub fn for_reorient(motion: &'a AdmittedSpatialReorient) -> Self {
        Self::Reorient(motion)
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

pub fn evaluate_binding_motion_posture(
    prior_binding: &SpatialAdmittedPrimitiveBinding,
    motion: BindingMotionSemanticsInput<'_>,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    match prior_binding {
        SpatialAdmittedPrimitiveBinding::FaceSurface(_)
        | SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(_)
        | SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor(_) => {
            evaluate_carrier_bound_motion_posture(motion)
        }
        SpatialAdmittedPrimitiveBinding::EdgeCurve(_)
        | SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor(_)
        | SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor(_) => {
            evaluate_carrier_bound_motion_posture(motion)
        }
        SpatialAdmittedPrimitiveBinding::CoedgePCurve(_)
        | SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor(_)
        | SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor(_) => {
            evaluate_carrier_bound_motion_posture(motion)
        }
        SpatialAdmittedPrimitiveBinding::VertexGeometry(_) => {
            evaluate_carrier_bound_motion_posture(motion)
        }
    }
}

fn evaluate_carrier_bound_motion_posture(
    motion: BindingMotionSemanticsInput<'_>,
) -> Result<MotionAwareBindingPosture, SpatialRebindingAuthorityError> {
    match motion {
        BindingMotionSemanticsInput::Move(_) => {
            Ok(MotionAwareBindingPosture::TransformedWithCarrier)
        }
        BindingMotionSemanticsInput::Rotate(motion) => {
            if motion.spec().angle_radians().abs() <= f64::EPSILON {
                Ok(MotionAwareBindingPosture::Preserved)
            } else {
                Ok(MotionAwareBindingPosture::TransformedWithCarrier)
            }
        }
        BindingMotionSemanticsInput::Reorient(_) => Ok(MotionAwareBindingPosture::Unresolved),
        BindingMotionSemanticsInput::InvalidatedByLocalTopologyReplacement => {
            Ok(MotionAwareBindingPosture::Invalidated)
        }
        BindingMotionSemanticsInput::UnresolvedWithoutMotionWorkflow => {
            Ok(MotionAwareBindingPosture::Unresolved)
        }
    }
}
