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
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement_with_catalog, apply_admitted_move_to_placement,
    apply_admitted_move_to_placement_with_catalog, apply_admitted_offset_to_placement,
    apply_admitted_offset_to_placement_with_catalog,
    apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement_with_catalog,
    apply_admitted_reorient_to_placement, apply_admitted_reorient_to_placement_with_catalog,
    apply_admitted_rotate_to_placement, apply_admitted_rotate_to_placement_with_catalog,
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
        let updated = apply_admitted_move_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
        )
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

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_offset(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated = apply_admitted_offset_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
        )
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
        let updated = apply_admitted_reorient_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
        )
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
        let updated = apply_admitted_rotate_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
        )
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

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_lies_on_constraint(self.constraint_spec().clone())
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_lies_on_constraint_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
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
        let updated = apply_admitted_points_toward_constraint_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
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

    pub fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_anchor_match_constraint(self.constraint_spec().clone())
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
            self.subject().placement_spec(),
            &admitted,
            catalog,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

#[cfg(test)]
#[path = "lowering_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "lowering_directional_tests.rs"]
mod directional_tests;
