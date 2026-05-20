use crate::construction::PrimitiveConstructionIntent;
use crate::spatial_intent::motion::{
    MoveSpatialIntent, OffsetSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use crate::spatial_intent::relations::{
    AnchorMatchSpatialIntent, LiesOnSpatialIntent, PointsTowardSpatialIntent,
};
use worth_spatial::facade::{
    admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint, admit_spatial_move,
    admit_spatial_offset, admit_spatial_points_toward_constraint, admit_spatial_reorient,
    admit_spatial_rotate, apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement, apply_admitted_move_to_placement,
    apply_admitted_offset_to_placement, apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_reorient_to_placement, apply_admitted_rotate_to_placement,
    SpatialConstraintError, SpatialMotionError, SpatialPlacementConstraintError,
    SpatialPlacementMotionError, SpatialWitnessCatalog,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveConstructionSpatialIntentError {
    MotionAdmission(SpatialMotionError),
    ConstraintAdmission(SpatialConstraintError),
    PlacementLowering(SpatialPlacementMotionError),
    ConstraintLowering(SpatialPlacementConstraintError),
}

impl std::fmt::Display for PrimitiveConstructionSpatialIntentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MotionAdmission(error) => write!(f, "{error}"),
            Self::ConstraintAdmission(error) => write!(f, "{error}"),
            Self::PlacementLowering(error) => write!(f, "{error}"),
            Self::ConstraintLowering(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionSpatialIntentError {}

impl MoveSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_move(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated = apply_admitted_move_to_placement(self.subject().placement_spec(), &admitted)
            .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated = apply_admitted_move_to_placement(self.subject().placement_spec(), &admitted)
            .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl OffsetSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_offset(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_offset_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ReorientSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_reorient(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_reorient_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_reorient_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl RotateSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_rotate(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_rotate_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_rotate_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl LiesOnSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_lies_on_constraint(self.constraint_spec().clone())
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_lies_on_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl PointsTowardSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_points_toward_constraint(self.constraint_spec().clone())
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_points_toward_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_points_toward_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl AnchorMatchSpatialIntent<PrimitiveConstructionIntent> {
    pub fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_anchor_match_constraint(self.constraint_spec().clone())
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_anchor_match_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

#[cfg(test)]
mod tests {
    use super::PrimitiveConstructionSpatialIntentError;
    use crate::construction::{PrimitiveConstructionIntent, RegularPyramidSpec, WireBodySpec};
    use crate::facade::{
        MoveSpatialIntent, OffsetSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
    };
    use worth_spatial::facade::{admit_spatial_placement, SpatialAnchorRef, SpatialFrameRef};

    #[test]
    fn primitive_construction_motion_finish_updates_embedded_placement() {
        let moved =
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .to([10.0, 0.0, 3.0])
            .finish()
            .expect("moved wire");
        let offset = OffsetSpatialIntent::shape(moved)
            .by([0.0, -2.0, 1.0])
            .finish()
            .expect("offset wire");
        let reoriented = ReorientSpatialIntent::shape(
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 4,
                radius: 2.0,
                height: 5.0,
            }),
        )
        .toward([0.0, 1.0, 1.0])
        .finish()
        .expect("reoriented pyramid");
        let rotated = RotateSpatialIntent::shape(reoriented.clone())
            .around([1.0, 0.0, 0.0])
            .by_radians(std::f64::consts::FRAC_PI_2)
            .finish()
            .expect("rotated pyramid");
        let constrained =
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .so(SpatialAnchorRef::shape_origin())
            .lies_on(SpatialFrameRef::workplane(
                "wp-1",
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 1.0],
            ))
            .finish()
            .expect("wire placed on workplane");
        let admitted_reoriented =
            admit_spatial_placement(reoriented.placement_spec()).expect("reoriented placement");
        let admitted_rotated =
            admit_spatial_placement(rotated.placement_spec()).expect("rotated placement");

        assert_eq!(offset.placement_spec().origin(), [10.0, -2.0, 4.0]);
        assert!(admitted_reoriented.facing_vector()[1] > 0.70);
        assert!(admitted_reoriented.facing_vector()[2] > 0.70);
        assert!(admitted_rotated.facing_vector()[1] < -0.70);
        assert!(admitted_rotated.facing_vector()[2] > 0.70);
        assert_eq!(constrained.placement_spec().origin(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn primitive_construction_motion_finish_rejects_unsupported_non_shape_origin_anchor() {
        let error =
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .from(SpatialAnchorRef::world_origin())
            .to([10.0, 0.0, 3.0])
            .finish()
            .expect_err("unsupported anchor should fail");

        assert!(matches!(
            error,
            PrimitiveConstructionSpatialIntentError::PlacementLowering(_)
        ));
    }
}
