use crate::spatial_intent::lowering::{
    admit_spatial_placement, AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialReorient,
    AdmittedSpatialRotate, SpatialPlacementSpec,
};
use crate::spatial_intent::refs::SpatialAnchorRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementMotionError {
    UnsupportedMoveAnchor,
    UnsupportedOffsetAnchor,
    UnsupportedRotateAnchor,
    UnsupportedReorientAnchor,
    InvalidExistingPlacement,
}

impl std::fmt::Display for SpatialPlacementMotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMoveAnchor => {
                write!(f, "only shape-origin movement can lower into placement")
            }
            Self::UnsupportedOffsetAnchor => {
                write!(f, "only shape-origin offset can lower into placement")
            }
            Self::UnsupportedRotateAnchor => {
                write!(f, "only shape-origin rotation can lower into placement")
            }
            Self::UnsupportedReorientAnchor => {
                write!(
                    f,
                    "only shape-origin reorientation can lower into placement"
                )
            }
            Self::InvalidExistingPlacement => {
                write!(
                    f,
                    "existing placement could not be admitted before motion lowering"
                )
            }
        }
    }
}

impl std::error::Error for SpatialPlacementMotionError {}

pub fn apply_admitted_move_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialMove,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => Ok(placement.at(motion.destination_point())),
        _ => Err(SpatialPlacementMotionError::UnsupportedMoveAnchor),
    }
}

pub fn apply_admitted_offset_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialOffset,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => {
            let origin = placement.origin();
            let offset = motion.spec().offset();
            Ok(placement.at([
                origin[0] + offset[0],
                origin[1] + offset[1],
                origin[2] + offset[2],
            ]))
        }
        _ => Err(SpatialPlacementMotionError::UnsupportedOffsetAnchor),
    }
}

pub fn apply_admitted_reorient_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialReorient,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => {
            Ok(placement.facing_witness(motion.spec().direction_witness().clone()))
        }
        _ => Err(SpatialPlacementMotionError::UnsupportedReorientAnchor),
    }
}

pub fn apply_admitted_rotate_to_placement(
    placement: SpatialPlacementSpec,
    motion: &AdmittedSpatialRotate,
) -> Result<SpatialPlacementSpec, SpatialPlacementMotionError> {
    match motion.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => {
            let facing = admit_spatial_placement(placement.clone())
                .map_err(|_| SpatialPlacementMotionError::InvalidExistingPlacement)?
                .facing_vector();
            Ok(placement.facing(rotate_vector(
                facing,
                motion.normalized_axis(),
                motion.spec().angle_radians(),
            )))
        }
        _ => Err(SpatialPlacementMotionError::UnsupportedRotateAnchor),
    }
}

fn rotate_vector(vector: [f64; 3], axis: [f64; 3], angle_radians: f64) -> [f64; 3] {
    let cos_theta = angle_radians.cos();
    let sin_theta = angle_radians.sin();
    let dot = axis[0] * vector[0] + axis[1] * vector[1] + axis[2] * vector[2];
    let cross = [
        axis[1] * vector[2] - axis[2] * vector[1],
        axis[2] * vector[0] - axis[0] * vector[2],
        axis[0] * vector[1] - axis[1] * vector[0],
    ];
    [
        vector[0] * cos_theta + cross[0] * sin_theta + axis[0] * dot * (1.0 - cos_theta),
        vector[1] * cos_theta + cross[1] * sin_theta + axis[1] * dot * (1.0 - cos_theta),
        vector[2] * cos_theta + cross[2] * sin_theta + axis[2] * dot * (1.0 - cos_theta),
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        apply_admitted_move_to_placement, apply_admitted_offset_to_placement,
        apply_admitted_reorient_to_placement, apply_admitted_rotate_to_placement,
        SpatialPlacementMotionError,
    };
    use crate::facade::{
        admit_spatial_move, admit_spatial_offset, admit_spatial_placement, admit_spatial_reorient,
        admit_spatial_rotate, SpatialAnchorRef, SpatialDirectionWitnessRef, SpatialMoveSpec,
        SpatialOffsetSpec, SpatialPlacementSpec, SpatialPointWitnessRef, SpatialReorientSpec,
        SpatialRotateSpec,
    };

    #[test]
    fn admitted_motion_can_lower_shape_origin_move_offset_and_reorient_into_placement() {
        let moved = apply_admitted_move_to_placement(
            SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
            &admit_spatial_move(SpatialMoveSpec::shape_origin().to([10.0, -4.0, 8.0]))
                .expect("move"),
        )
        .expect("moved placement");
        let offset = apply_admitted_offset_to_placement(
            moved.clone(),
            &admit_spatial_offset(SpatialOffsetSpec::shape_origin().by([2.0, 0.0, -3.0]))
                .expect("offset"),
        )
        .expect("offset placement");
        let reoriented = apply_admitted_reorient_to_placement(
            offset,
            &admit_spatial_reorient(
                SpatialReorientSpec::shape_origin()
                    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
            )
            .expect("reorient"),
        )
        .expect("reoriented placement");
        let rotated = apply_admitted_rotate_to_placement(
            reoriented.clone(),
            &admit_spatial_rotate(
                SpatialRotateSpec::shape_origin()
                    .around([1.0, 0.0, 0.0])
                    .by_radians(std::f64::consts::FRAC_PI_2),
            )
            .expect("rotate"),
        )
        .expect("rotated placement");
        let admitted_reoriented = admit_spatial_placement(reoriented.clone()).expect("admitted");
        let admitted_rotated = admit_spatial_placement(rotated.clone()).expect("admitted rotated");

        assert_eq!(moved.origin(), [10.0, -4.0, 8.0]);
        assert_eq!(reoriented.origin(), [12.0, -4.0, 5.0]);
        assert!(admitted_reoriented.facing_vector()[1] > 0.70);
        assert!(admitted_reoriented.facing_vector()[2] > 0.70);
        assert!(admitted_rotated.facing_vector()[1] < -0.70);
        assert!(admitted_rotated.facing_vector()[2] > 0.70);
    }

    #[test]
    fn lowering_rejects_non_shape_origin_anchors_for_current_placement_model() {
        let error = apply_admitted_move_to_placement(
            SpatialPlacementSpec::world(),
            &admit_spatial_move(
                SpatialMoveSpec::shape_origin()
                    .from(SpatialAnchorRef::world_origin())
                    .to_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0])),
            )
            .expect("move"),
        )
        .expect_err("unsupported world-origin move anchor should fail");

        assert_eq!(error, SpatialPlacementMotionError::UnsupportedMoveAnchor);
    }
}
