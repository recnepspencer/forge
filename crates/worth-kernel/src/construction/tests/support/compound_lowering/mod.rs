mod motion;
mod relations;
mod spatial_fixture_witness_catalog;

use crate::construction::intent::PrimitiveConstructionIntent;
use worth_spatial::facade::anchor_selection::{
    SpatialAnchorSelectionPlacementError, SpatialConstraintError, SpatialMotionError,
};
use worth_spatial::facade::placement::{
    SpatialPlacementConstraintError, SpatialPlacementMotionError,
};
use worth_spatial::facade::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};

pub(crate) use motion::{
    ConstructionMovePlan, ConstructionOffsetPlan, ConstructionReorientPlan, ConstructionRotatePlan,
};
pub(crate) use relations::{
    ConstructionAnchorMatchConstraintPlan, ConstructionLiesOnConstraintPlan,
    ConstructionPointsTowardConstraintPlan,
};
pub(crate) use spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PrimitiveConstructionMotionLoweringError {
    MotionAdmission(SpatialMotionError),
    ConstraintAdmission(SpatialConstraintError),
    PlacementLowering(SpatialPlacementMotionError),
    ConstraintLowering(SpatialPlacementConstraintError),
}

impl std::fmt::Display for PrimitiveConstructionMotionLoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MotionAdmission(error) => write!(f, "{error}"),
            Self::ConstraintAdmission(error) => write!(f, "{error}"),
            Self::PlacementLowering(error) => write!(f, "{error}"),
            Self::ConstraintLowering(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionMotionLoweringError {}

impl ConstructionMovePlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionOffsetPlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionReorientPlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionRotatePlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionMotionLoweringError::MotionAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionLiesOnConstraintPlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self.admit();
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self.admit_with_catalog(catalog);
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionPointsTowardConstraintPlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit()
            .map_err(PrimitiveConstructionMotionLoweringError::ConstraintAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(
                self.subject().placement_spec(),
                &EmptySpatialWitnessCatalog,
            )
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }

    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self
            .admit_with_catalog(catalog)
            .map_err(PrimitiveConstructionMotionLoweringError::ConstraintAdmission)?;
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

impl ConstructionAnchorMatchConstraintPlan<PrimitiveConstructionIntent> {
    pub(crate) fn finish_with_catalog(
        self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionMotionLoweringError> {
        let admitted = self.admit();
        let updated = admitted
            .apply_to_placement_with_catalog(self.subject().placement_spec(), catalog)
            .map_err(map_anchor_selection_lowering_error)?;
        Ok(self.subject().clone().with_placement_spec(updated))
    }
}

fn map_anchor_selection_lowering_error(
    error: SpatialAnchorSelectionPlacementError,
) -> PrimitiveConstructionMotionLoweringError {
    match error {
        SpatialAnchorSelectionPlacementError::MotionAdmission(error) => {
            PrimitiveConstructionMotionLoweringError::MotionAdmission(error)
        }
        SpatialAnchorSelectionPlacementError::ConstraintAdmission(error) => {
            PrimitiveConstructionMotionLoweringError::ConstraintAdmission(error)
        }
        SpatialAnchorSelectionPlacementError::PlacementMotion(error) => {
            PrimitiveConstructionMotionLoweringError::PlacementLowering(error)
        }
        SpatialAnchorSelectionPlacementError::PlacementConstraint(error) => {
            PrimitiveConstructionMotionLoweringError::ConstraintLowering(error)
        }
    }
}
