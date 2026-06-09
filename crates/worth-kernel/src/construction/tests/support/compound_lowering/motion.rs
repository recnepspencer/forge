use super::relations::{ConstructionMoveConstraintPlan, ConstructionReorientConstraintPlan};
use worth_spatial::facade::anchor_selection::{
    AuthorSpatialAnchorSelectionIntent, SpatialAnchorSelectionDeclarationEntry,
    SpatialAnchorSelectionFailureKind, SpatialAnchorSelectionStatus, SpatialMotionError,
    SpatialMoveSpec, SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
use worth_spatial::facade::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialDirectionWitnessRef};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionMovePlan<S> {
    subject: S,
    spec: SpatialMoveSpec,
}

impl<S> ConstructionMovePlan<S> {
    pub(crate) fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialMoveSpec::shape_origin(),
        }
    }

    pub(crate) fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.from(anchor),
            ..self
        }
    }

    pub(crate) fn to(self, destination: [f64; 3]) -> Self {
        Self {
            spec: self.spec.to(destination),
            ..self
        }
    }

    pub(crate) fn so(self, anchor: SpatialAnchorRef) -> ConstructionMoveConstraintPlan<S> {
        ConstructionMoveConstraintPlan::new(self.subject, anchor)
    }

    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }
    pub(crate) fn admit(
        &self,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::Move(self.spec.clone()),
            catalog,
        );
        match declaration.status() {
            SpatialAnchorSelectionStatus::Admitted => Ok(declaration),
            SpatialAnchorSelectionStatus::Rejected => match declaration.failure_kind() {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    Err(SpatialMotionError::DestinationWitnessFailure(class))
                }
                _ => unreachable!("move declaration should reject only with witness failure"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionRotatePlan<S> {
    subject: S,
    spec: SpatialRotateSpec,
}

impl<S> ConstructionRotatePlan<S> {
    pub(crate) fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialRotateSpec::shape_origin(),
        }
    }

    pub(crate) fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.about(anchor),
            ..self
        }
    }

    pub(crate) fn around(self, axis: [f64; 3]) -> Self {
        Self {
            spec: self.spec.around(axis),
            ..self
        }
    }

    pub(crate) fn by_radians(self, angle_radians: f64) -> Self {
        Self {
            spec: self.spec.by_radians(angle_radians),
            ..self
        }
    }

    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }
    pub(crate) fn admit(
        &self,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::Rotate(self.spec.clone()),
            catalog,
        );
        match declaration.status() {
            SpatialAnchorSelectionStatus::Admitted => Ok(declaration),
            SpatialAnchorSelectionStatus::Rejected => match declaration.failure_kind() {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    Err(SpatialMotionError::RotationWitnessFailure(class))
                }
                Some(SpatialAnchorSelectionFailureKind::NonFiniteRotationAngle) => {
                    Err(SpatialMotionError::NonFiniteRotationAngle)
                }
                _ => unreachable!("rotate declaration should reject with known motion failure"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionReorientPlan<S> {
    subject: S,
    spec: SpatialReorientSpec,
}

impl<S> ConstructionReorientPlan<S> {
    pub(crate) fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialReorientSpec::shape_origin(),
        }
    }

    pub(crate) fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.about(anchor),
            ..self
        }
    }

    pub(crate) fn toward(self, facing: [f64; 3]) -> Self {
        Self {
            spec: self.spec.toward(facing),
            ..self
        }
    }

    pub(crate) fn toward_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            spec: self.spec.toward_witness(direction_witness),
            ..self
        }
    }

    pub(crate) fn so(self, anchor: SpatialAnchorRef) -> ConstructionReorientConstraintPlan<S> {
        ConstructionReorientConstraintPlan::new(self.subject, anchor)
    }

    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }
    pub(crate) fn admit(
        &self,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::Reorient(self.spec.clone()),
            catalog,
        );
        match declaration.status() {
            SpatialAnchorSelectionStatus::Admitted => Ok(declaration),
            SpatialAnchorSelectionStatus::Rejected => match declaration.failure_kind() {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    Err(SpatialMotionError::DirectionWitnessFailure(class))
                }
                _ => {
                    unreachable!("reorient declaration should reject only with witness failure")
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionOffsetPlan<S> {
    subject: S,
    spec: SpatialOffsetSpec,
}

impl<S> ConstructionOffsetPlan<S> {
    pub(crate) fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialOffsetSpec::shape_origin(),
        }
    }

    pub(crate) fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.from(anchor),
            ..self
        }
    }

    pub(crate) fn by(self, offset: [f64; 3]) -> Self {
        Self {
            spec: self.spec.by(offset),
            ..self
        }
    }

    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }
    pub(crate) fn admit(
        &self,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialMotionError> {
        let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::Offset(self.spec.clone()),
            catalog,
        );
        match declaration.status() {
            SpatialAnchorSelectionStatus::Admitted => Ok(declaration),
            SpatialAnchorSelectionStatus::Rejected => match declaration.failure_kind() {
                Some(SpatialAnchorSelectionFailureKind::NonFiniteOffset) => {
                    Err(SpatialMotionError::NonFiniteOffset)
                }
                _ => unreachable!("offset declaration should reject only with non-finite offset"),
            },
        }
    }
}
