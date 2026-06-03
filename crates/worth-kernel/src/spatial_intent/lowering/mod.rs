use crate::construction::PrimitiveConstructionIntent;
use crate::spatial_intent::motion::{
    MoveSpatialIntent, OffsetSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use crate::spatial_intent::relations::{
    AnchorMatchSpatialIntent, LiesOnSpatialIntent, PointsTowardSpatialIntent,
};
use worth_spatial::facade::constraints::{
    apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement_with_catalog,
    apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement_with_catalog, SpatialConstraintError,
};
use worth_spatial::facade::motion::{
    admit_spatial_offset, apply_admitted_move_to_placement,
    apply_admitted_move_to_placement_with_catalog, apply_admitted_offset_to_placement,
    apply_admitted_offset_to_placement_with_catalog, apply_admitted_reorient_to_placement,
    apply_admitted_reorient_to_placement_with_catalog, apply_admitted_rotate_to_placement,
    apply_admitted_rotate_to_placement_with_catalog, SpatialMotionError,
};
use worth_spatial::facade::placement::{
    SpatialPlacementConstraintError, SpatialPlacementMotionError,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated = apply_admitted_move_to_placement(self.subject().placement_spec(), &admitted)
            .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = admit_spatial_offset(self.motion_spec())
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_offset_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_reorient_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::MotionAdmission)?;
        let updated =
            apply_admitted_rotate_to_placement(self.subject().placement_spec(), &admitted)
                .map_err(PrimitiveConstructionSpatialIntentError::PlacementLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_lies_on_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_points_toward_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
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
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionSpatialIntentError::ConstraintAdmission)?;
        let updated = apply_admitted_anchor_match_constraint_to_placement(
            self.subject().placement_spec(),
            &admitted,
        )
        .map_err(PrimitiveConstructionSpatialIntentError::ConstraintLowering)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        let admitted = self
            .admit()
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
