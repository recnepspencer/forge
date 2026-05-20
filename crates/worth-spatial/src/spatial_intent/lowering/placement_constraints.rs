use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::SpatialPlacementSpec;
use crate::spatial_intent::refs::{
    admit_spatial_frame, SpatialAnchorRef, SpatialFrameError, SpatialFrameRef,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPlacementConstraintError {
    UnsupportedLiesOnAnchor,
    UnsupportedPointsTowardAnchor,
    UnsupportedAnchorMatch,
    InvalidReferenceFrame(SpatialFrameError),
    CoincidentTarget,
}

impl std::fmt::Display for SpatialPlacementConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedLiesOnAnchor => {
                write!(
                    f,
                    "only shape-origin lies-on constraints can lower into placement"
                )
            }
            Self::UnsupportedPointsTowardAnchor => {
                write!(
                    f,
                    "only shape-origin points-toward constraints can lower into placement"
                )
            }
            Self::UnsupportedAnchorMatch => {
                write!(
                    f,
                    "only shape-origin anchor matches against frame or world origins can lower into placement"
                )
            }
            Self::InvalidReferenceFrame(error) => write!(f, "{error}"),
            Self::CoincidentTarget => {
                write!(
                    f,
                    "points-toward target must not collapse into the current origin"
                )
            }
        }
    }
}

impl std::error::Error for SpatialPlacementConstraintError {}

pub fn apply_admitted_lies_on_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialLiesOnConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    match constraint.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => Ok(placement
            .relative_to(constraint.frame().spec().clone())
            .at([0.0, 0.0, 0.0])),
        _ => Err(SpatialPlacementConstraintError::UnsupportedLiesOnAnchor),
    }
}

pub fn apply_admitted_points_toward_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialPointsTowardConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    match constraint.spec().anchor() {
        SpatialAnchorRef::ShapeOrigin => {
            let reference_frame = admit_spatial_frame(placement.reference_frame().clone())
                .map_err(SpatialPlacementConstraintError::InvalidReferenceFrame)?;
            let target_local = reference_frame
                .basis()
                .project_point(constraint.target_point());
            let origin_local = placement.origin();
            let facing = [
                target_local[0] - origin_local[0],
                target_local[1] - origin_local[1],
                target_local[2] - origin_local[2],
            ];
            if facing.iter().all(|value| value.abs() <= f64::MIN_POSITIVE) {
                return Err(SpatialPlacementConstraintError::CoincidentTarget);
            }
            Ok(placement.facing(facing))
        }
        _ => Err(SpatialPlacementConstraintError::UnsupportedPointsTowardAnchor),
    }
}

pub fn apply_admitted_anchor_match_constraint_to_placement(
    placement: SpatialPlacementSpec,
    constraint: &AdmittedSpatialAnchorMatchConstraint,
) -> Result<SpatialPlacementSpec, SpatialPlacementConstraintError> {
    match (constraint.spec().anchor(), constraint.spec().other_anchor()) {
        (SpatialAnchorRef::ShapeOrigin, SpatialAnchorRef::WorldOrigin) => Ok(placement
            .relative_to(SpatialFrameRef::world())
            .at([0.0, 0.0, 0.0])),
        (SpatialAnchorRef::ShapeOrigin, SpatialAnchorRef::FrameOrigin(frame)) => {
            Ok(placement.relative_to(frame.clone()).at([0.0, 0.0, 0.0]))
        }
        _ => Err(SpatialPlacementConstraintError::UnsupportedAnchorMatch),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_admitted_anchor_match_constraint_to_placement,
        apply_admitted_lies_on_constraint_to_placement,
        apply_admitted_points_toward_constraint_to_placement, SpatialPlacementConstraintError,
    };
    use crate::facade::{
        admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint,
        admit_spatial_placement, admit_spatial_points_toward_constraint,
        SpatialAnchorMatchConstraintSpec, SpatialAnchorRef, SpatialFrameRef,
        SpatialLiesOnConstraintSpec, SpatialPlacementSpec, SpatialPointsTowardConstraintSpec,
    };

    #[test]
    fn admitted_constraints_can_lower_shape_origin_constraint_intent_into_placement() {
        let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let placed = apply_admitted_lies_on_constraint_to_placement(
            SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
            &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
                SpatialAnchorRef::shape_origin(),
                workplane.clone(),
            ))
            .expect("lies-on"),
        )
        .expect("placed on frame");
        let pointed = apply_admitted_points_toward_constraint_to_placement(
            placed.clone(),
            &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
                SpatialAnchorRef::shape_origin(),
                [0.0, 1.0, 7.0],
            ))
            .expect("points-toward"),
        )
        .expect("pointed");
        let matched = apply_admitted_anchor_match_constraint_to_placement(
            placed,
            &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
                SpatialAnchorRef::shape_origin(),
                SpatialAnchorRef::world_origin(),
            ))
            .expect("match"),
        )
        .expect("matched");
        let admitted_pointed = admit_spatial_placement(pointed.clone()).expect("admitted pointed");

        assert_eq!(pointed.reference_frame(), &workplane);
        assert_eq!(pointed.origin(), [0.0, 0.0, 0.0]);
        assert!(admitted_pointed.facing_vector()[2] > 0.0);
        assert!(
            admitted_pointed.facing_vector()[0].abs() > 0.0
                || admitted_pointed.facing_vector()[1].abs() > 0.0
        );
        assert_eq!(matched.reference_frame(), &SpatialFrameRef::world());
    }

    #[test]
    fn constraint_lowering_rejects_non_shape_origin_constraint_anchors() {
        let error = apply_admitted_lies_on_constraint_to_placement(
            SpatialPlacementSpec::world(),
            &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
                SpatialAnchorRef::world_origin(),
                SpatialFrameRef::world(),
            ))
            .expect("lies-on"),
        )
        .expect_err("unsupported world-origin constraint should fail");

        assert_eq!(
            error,
            SpatialPlacementConstraintError::UnsupportedLiesOnAnchor
        );
    }
}
