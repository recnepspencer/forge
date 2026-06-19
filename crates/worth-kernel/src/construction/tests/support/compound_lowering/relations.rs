use worth_spatial::facade::anchor_selection::{
    AuthorSpatialAnchorSelectionIntent, SpatialAnchorMatchConstraintSpec,
    SpatialAnchorSelectionDeclarationEntry, SpatialAnchorSelectionFailureKind,
    SpatialAnchorSelectionStatus, SpatialConstraintError, SpatialLiesOnConstraintSpec,
    SpatialPointsTowardConstraintSpec, SpatialWitnessFailureClass,
};
use worth_spatial::facade::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialFrameRef};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionMoveConstraintPlan<S> {
    subject: S,
    anchor: SpatialAnchorRef,
}

impl<S> ConstructionMoveConstraintPlan<S> {
    pub(crate) fn new(subject: S, anchor: SpatialAnchorRef) -> Self {
        Self { subject, anchor }
    }

    pub(crate) fn lies_on(self, frame: SpatialFrameRef) -> ConstructionLiesOnConstraintPlan<S> {
        ConstructionLiesOnConstraintPlan {
            subject: self.subject,
            spec: SpatialLiesOnConstraintSpec::new(self.anchor, frame),
        }
    }

    pub(crate) fn matches(
        self,
        other_anchor: SpatialAnchorRef,
    ) -> ConstructionAnchorMatchConstraintPlan<S> {
        ConstructionAnchorMatchConstraintPlan {
            subject: self.subject,
            spec: SpatialAnchorMatchConstraintSpec::new(self.anchor, other_anchor),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionReorientConstraintPlan<S> {
    subject: S,
    anchor: SpatialAnchorRef,
}

impl<S> ConstructionReorientConstraintPlan<S> {
    pub(crate) fn new(subject: S, anchor: SpatialAnchorRef) -> Self {
        Self { subject, anchor }
    }

    pub(crate) fn points_toward(
        self,
        target_point: [f64; 3],
    ) -> ConstructionPointsTowardConstraintPlan<S> {
        ConstructionPointsTowardConstraintPlan {
            subject: self.subject,
            spec: SpatialPointsTowardConstraintSpec::new(self.anchor, target_point),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionLiesOnConstraintPlan<S> {
    subject: S,
    spec: SpatialLiesOnConstraintSpec,
}

impl<S> ConstructionLiesOnConstraintPlan<S> {
    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }

    pub(crate) fn admit(&self) -> SpatialAnchorSelectionDeclarationEntry {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> SpatialAnchorSelectionDeclarationEntry {
        SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(self.spec.clone()),
            catalog,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionPointsTowardConstraintPlan<S> {
    subject: S,
    spec: SpatialPointsTowardConstraintSpec,
}

impl<S> ConstructionPointsTowardConstraintPlan<S> {
    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }

    pub(crate) fn admit(
        &self,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialConstraintError> {
        self.admit_with_catalog(&EmptySpatialWitnessCatalog)
    }

    pub(crate) fn admit_with_catalog(
        &self,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialAnchorSelectionDeclarationEntry, SpatialConstraintError> {
        let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::PointsToward(self.spec.clone()),
            catalog,
        );
        match declaration.status() {
            SpatialAnchorSelectionStatus::Admitted => Ok(declaration),
            SpatialAnchorSelectionStatus::Rejected => match declaration.failure_kind() {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    Err(SpatialConstraintError::TargetWitnessFailure(class))
                }
                _ => Err(SpatialConstraintError::TargetWitnessFailure(
                    SpatialWitnessFailureClass::Unsupported,
                )),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstructionAnchorMatchConstraintPlan<S> {
    subject: S,
    spec: SpatialAnchorMatchConstraintSpec,
}

impl<S> ConstructionAnchorMatchConstraintPlan<S> {
    pub(crate) fn subject(&self) -> &S {
        &self.subject
    }

    pub(crate) fn admit(&self) -> SpatialAnchorSelectionDeclarationEntry {
        SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(self.spec.clone()),
            &EmptySpatialWitnessCatalog,
        )
    }
}
