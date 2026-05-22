use worth_spatial::facade::{
    admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint,
    admit_spatial_points_toward_constraint, admit_spatial_points_toward_constraint_with_catalog,
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint, SpatialAnchorMatchConstraintSpec, SpatialAnchorRef,
    SpatialConstraintError, SpatialFrameRef, SpatialLiesOnConstraintSpec, SpatialPointWitnessRef,
    SpatialPointsTowardConstraintSpec, SpatialWitnessCatalog,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintMoveSpatialIntent<S> {
    subject: S,
    anchor: SpatialAnchorRef,
}

impl<S> ConstraintMoveSpatialIntent<S> {
    pub(crate) fn new(subject: S, anchor: SpatialAnchorRef) -> Self {
        Self { subject, anchor }
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn lies_on(self, frame: SpatialFrameRef) -> LiesOnSpatialIntent<S> {
        LiesOnSpatialIntent {
            subject: self.subject,
            spec: SpatialLiesOnConstraintSpec::new(self.anchor, frame),
        }
    }

    pub fn matches(self, other_anchor: SpatialAnchorRef) -> AnchorMatchSpatialIntent<S> {
        AnchorMatchSpatialIntent {
            subject: self.subject,
            spec: SpatialAnchorMatchConstraintSpec::new(self.anchor, other_anchor),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintReorientSpatialIntent<S> {
    subject: S,
    anchor: SpatialAnchorRef,
}

impl<S> ConstraintReorientSpatialIntent<S> {
    pub(crate) fn new(subject: S, anchor: SpatialAnchorRef) -> Self {
        Self { subject, anchor }
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn points_toward(self, target_point: [f64; 3]) -> PointsTowardSpatialIntent<S> {
        PointsTowardSpatialIntent {
            subject: self.subject,
            spec: SpatialPointsTowardConstraintSpec::new(self.anchor, target_point),
        }
    }

    pub fn points_toward_witness(
        self,
        target_witness: SpatialPointWitnessRef,
    ) -> PointsTowardSpatialIntent<S> {
        PointsTowardSpatialIntent {
            subject: self.subject,
            spec: SpatialPointsTowardConstraintSpec::with_witness(self.anchor, target_witness),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiesOnSpatialIntent<S> {
    subject: S,
    spec: SpatialLiesOnConstraintSpec,
}

impl<S> LiesOnSpatialIntent<S> {
    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn constraint_spec(&self) -> &SpatialLiesOnConstraintSpec {
        &self.spec
    }

    pub fn admit(&self) -> Result<AdmittedSpatialLiesOnConstraint, SpatialConstraintError> {
        admit_spatial_lies_on_constraint(self.spec.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointsTowardSpatialIntent<S> {
    subject: S,
    spec: SpatialPointsTowardConstraintSpec,
}

impl<S> PointsTowardSpatialIntent<S> {
    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn constraint_spec(&self) -> &SpatialPointsTowardConstraintSpec {
        &self.spec
    }

    pub fn admit(&self) -> Result<AdmittedSpatialPointsTowardConstraint, SpatialConstraintError> {
        admit_spatial_points_toward_constraint(self.spec.clone())
    }

    pub fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<AdmittedSpatialPointsTowardConstraint, SpatialConstraintError> {
        admit_spatial_points_toward_constraint_with_catalog(self.spec.clone(), catalog)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnchorMatchSpatialIntent<S> {
    subject: S,
    spec: SpatialAnchorMatchConstraintSpec,
}

impl<S> AnchorMatchSpatialIntent<S> {
    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn constraint_spec(&self) -> &SpatialAnchorMatchConstraintSpec {
        &self.spec
    }

    pub fn admit(&self) -> Result<AdmittedSpatialAnchorMatchConstraint, SpatialConstraintError> {
        admit_spatial_anchor_match_constraint(self.spec.clone())
    }
}
