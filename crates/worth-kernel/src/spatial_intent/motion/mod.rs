use crate::spatial_intent::relations::{
    ConstraintMoveSpatialIntent, ConstraintReorientSpatialIntent,
};
use worth_spatial::facade::motion::{
    admit_spatial_move, admit_spatial_move_with_catalog, admit_spatial_offset,
    admit_spatial_reorient, admit_spatial_reorient_with_catalog, admit_spatial_rotate,
    admit_spatial_rotate_with_catalog, AdmittedSpatialMove, AdmittedSpatialOffset,
    AdmittedSpatialReorient, AdmittedSpatialRotate, SpatialMotionError, SpatialMoveSpec,
    SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

#[derive(Clone, Debug, PartialEq)]
pub struct MoveSpatialIntent<S> {
    subject: S,
    spec: SpatialMoveSpec,
}

impl<S> MoveSpatialIntent<S> {
    pub fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialMoveSpec::shape_origin(),
        }
    }

    pub fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.from(anchor),
            ..self
        }
    }

    pub fn to(self, destination: [f64; 3]) -> Self {
        Self {
            spec: self.spec.to(destination),
            ..self
        }
    }

    pub fn to_witness(self, destination_witness: SpatialPointWitnessRef) -> Self {
        Self {
            spec: self.spec.to_witness(destination_witness),
            ..self
        }
    }

    pub fn so(self, anchor: SpatialAnchorRef) -> ConstraintMoveSpatialIntent<S> {
        ConstraintMoveSpatialIntent::new(self.subject, anchor)
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }
    pub fn motion_spec(&self) -> SpatialMoveSpec {
        self.spec.clone()
    }
    pub(crate) fn admit(&self) -> Result<AdmittedSpatialMove, SpatialMotionError> {
        admit_spatial_move(self.spec.clone())
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<AdmittedSpatialMove, SpatialMotionError> {
        admit_spatial_move_with_catalog(self.spec.clone(), catalog)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateSpatialIntent<S> {
    subject: S,
    spec: SpatialRotateSpec,
}

impl<S> RotateSpatialIntent<S> {
    pub fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialRotateSpec::shape_origin(),
        }
    }

    pub fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.about(anchor),
            ..self
        }
    }

    pub fn around(self, axis: [f64; 3]) -> Self {
        Self {
            spec: self.spec.around(axis),
            ..self
        }
    }

    pub fn around_witness(self, axis_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            spec: self.spec.around_witness(axis_witness),
            ..self
        }
    }

    pub fn by_radians(self, angle_radians: f64) -> Self {
        Self {
            spec: self.spec.by_radians(angle_radians),
            ..self
        }
    }

    pub fn rotated_about(self, axis: [f64; 3], angle_radians: f64) -> Self {
        self.around(axis).by_radians(angle_radians)
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }
    pub fn motion_spec(&self) -> SpatialRotateSpec {
        self.spec.clone()
    }
    pub(crate) fn admit(&self) -> Result<AdmittedSpatialRotate, SpatialMotionError> {
        admit_spatial_rotate(self.spec.clone())
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<AdmittedSpatialRotate, SpatialMotionError> {
        admit_spatial_rotate_with_catalog(self.spec.clone(), catalog)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReorientSpatialIntent<S> {
    subject: S,
    spec: SpatialReorientSpec,
}

impl<S> ReorientSpatialIntent<S> {
    pub fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialReorientSpec::shape_origin(),
        }
    }

    pub fn about(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.about(anchor),
            ..self
        }
    }

    pub fn toward(self, facing: [f64; 3]) -> Self {
        Self {
            spec: self.spec.toward(facing),
            ..self
        }
    }

    pub fn toward_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            spec: self.spec.toward_witness(direction_witness),
            ..self
        }
    }

    pub fn aligned_with(self, frame: SpatialFrameRef) -> Self {
        Self {
            spec: self.spec.aligned_with(frame),
            ..self
        }
    }

    pub fn parallel_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            spec: self.spec.parallel_to(frame),
            ..self
        }
    }

    pub fn perpendicular_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            spec: self.spec.perpendicular_to(frame),
            ..self
        }
    }

    pub fn so(self, anchor: SpatialAnchorRef) -> ConstraintReorientSpatialIntent<S> {
        ConstraintReorientSpatialIntent::new(self.subject, anchor)
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }
    pub fn motion_spec(&self) -> SpatialReorientSpec {
        self.spec.clone()
    }
    pub(crate) fn admit(&self) -> Result<AdmittedSpatialReorient, SpatialMotionError> {
        admit_spatial_reorient(self.spec.clone())
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<AdmittedSpatialReorient, SpatialMotionError> {
        admit_spatial_reorient_with_catalog(self.spec.clone(), catalog)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetSpatialIntent<S> {
    subject: S,
    spec: SpatialOffsetSpec,
}

impl<S> OffsetSpatialIntent<S> {
    pub fn shape(subject: S) -> Self {
        Self {
            subject,
            spec: SpatialOffsetSpec::shape_origin(),
        }
    }

    pub fn from(self, anchor: SpatialAnchorRef) -> Self {
        Self {
            spec: self.spec.from(anchor),
            ..self
        }
    }

    pub fn by(self, offset: [f64; 3]) -> Self {
        Self {
            spec: self.spec.by(offset),
            ..self
        }
    }

    pub fn translated_by(self, offset: [f64; 3]) -> Self {
        Self {
            spec: self.spec.translated_by(offset),
            ..self
        }
    }

    pub fn offset_by(self, offset: [f64; 3]) -> Self {
        Self {
            spec: self.spec.offset_by(offset),
            ..self
        }
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }
    pub fn motion_spec(&self) -> SpatialOffsetSpec {
        self.spec.clone()
    }
    pub(crate) fn admit(&self) -> Result<AdmittedSpatialOffset, SpatialMotionError> {
        admit_spatial_offset(self.spec.clone())
    }
}
