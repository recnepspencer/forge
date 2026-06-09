use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use super::query_native_lowered_facts::{
    lower_intent_with_catalog, SpatialAnchorSelectionLoweredFacts,
};
use super::{SpatialConstraintError, SpatialMotionError};
use crate::anchor_selection::query_native::{
    SpatialAnchorSelectionDeclarationFamily, SpatialAnchorSelectionQueryDomain,
};
use crate::anchor_selection::{
    SpatialAnchorMatchConstraintSpec, SpatialLiesOnConstraintSpec, SpatialMoveSpec,
    SpatialOffsetSpec, SpatialPointsTowardConstraintSpec, SpatialReorientSpec, SpatialRotateSpec,
};
#[cfg(test)]
use crate::facade::refs::EmptySpatialWitnessCatalog;
use crate::facade::refs::SpatialWitnessCatalog;
use crate::facade::refs::{
    SpatialAnchorRef, SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
};
#[cfg(any(test, feature = "test-support-lowering"))]
use crate::placement::SpatialPlacementSpec;
use crate::placement::{SpatialPlacementConstraintError, SpatialPlacementMotionError};
use crate::witness_resolution::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionKind {
    Move,
    Rotate,
    Reorient,
    Offset,
    LiesOnConstraint,
    PointsToward,
    AnchorMatchConstraint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionStatus {
    Admitted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionFailureKind {
    Witness(SpatialWitnessFailureClass),
    NonFiniteRotationAngle,
    NonFiniteOffset,
    InvalidFrame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionPlacementError {
    MotionAdmission(SpatialMotionError),
    ConstraintAdmission(SpatialConstraintError),
    PlacementMotion(SpatialPlacementMotionError),
    PlacementConstraint(SpatialPlacementConstraintError),
}

impl std::fmt::Display for SpatialAnchorSelectionPlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MotionAdmission(error) => write!(f, "{error}"),
            Self::ConstraintAdmission(error) => write!(f, "{error}"),
            Self::PlacementMotion(error) => write!(f, "{error}"),
            Self::PlacementConstraint(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SpatialAnchorSelectionPlacementError {}

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialAnchorSelectionRequestedInput {
    PointWitness(SpatialPointWitnessRef),
    DirectionWitness(SpatialDirectionWitnessRef),
    Offset([f64; 3]),
    Frame(SpatialFrameRef),
    OtherAnchor(SpatialAnchorRef),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpatialResolvedAnchorWitness {
    Point([f64; 3]),
    Direction([f64; 3]),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorSpatialAnchorSelectionIntent {
    Move(SpatialMoveSpec),
    Rotate(SpatialRotateSpec),
    Reorient(SpatialReorientSpec),
    Offset(SpatialOffsetSpec),
    LiesOnConstraint(SpatialLiesOnConstraintSpec),
    PointsToward(SpatialPointsTowardConstraintSpec),
    AnchorMatchConstraint(SpatialAnchorMatchConstraintSpec),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialAnchorSelectionDeclarationEntry {
    intent: AuthorSpatialAnchorSelectionIntent,
    status: SpatialAnchorSelectionStatus,
    requested_input: SpatialAnchorSelectionRequestedInput,
    resolved_witness: Option<SpatialResolvedAnchorWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<SpatialAnchorSelectionFailureKind>,
    lowered_facts: Option<SpatialAnchorSelectionLoweredFacts>,
    projection_seed: SpatialAnchorSelectionProjectionSeed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialAnchorSelectionProjectionSeed {
    kind: SpatialAnchorSelectionKind,
    anchor: String,
    requested_input: SpatialAnchorSelectionRequestedInput,
    status: SpatialAnchorSelectionStatus,
    resolved_witness: Option<SpatialResolvedAnchorWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<SpatialAnchorSelectionFailureKind>,
}

impl SpatialAnchorSelectionProjectionSeed {
    fn from_parts(
        kind: SpatialAnchorSelectionKind,
        anchor: &SpatialAnchorRef,
        requested_input: &SpatialAnchorSelectionRequestedInput,
        status: SpatialAnchorSelectionStatus,
        resolved_witness: Option<SpatialResolvedAnchorWitness>,
        resolution_class: Option<SpatialWitnessResolutionClass>,
        failure_kind: Option<SpatialAnchorSelectionFailureKind>,
    ) -> Self {
        Self {
            kind,
            anchor: format!("{anchor:?}"),
            requested_input: requested_input.clone(),
            status,
            resolved_witness,
            resolution_class,
            failure_kind,
        }
    }

    pub fn kind(&self) -> SpatialAnchorSelectionKind {
        self.kind
    }

    pub fn anchor(&self) -> &str {
        &self.anchor
    }

    pub fn requested_input(&self) -> &SpatialAnchorSelectionRequestedInput {
        &self.requested_input
    }

    pub fn status(&self) -> SpatialAnchorSelectionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<SpatialResolvedAnchorWitness> {
        self.resolved_witness
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<SpatialAnchorSelectionFailureKind> {
        self.failure_kind
    }
}

impl SpatialAnchorSelectionDeclarationEntry {
    #[cfg(test)]
    pub fn from_author_intent(intent: AuthorSpatialAnchorSelectionIntent) -> Self {
        Self::from_author_intent_with_catalog(intent, &EmptySpatialWitnessCatalog)
    }

    pub fn from_author_intent_with_catalog(
        intent: AuthorSpatialAnchorSelectionIntent,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Self {
        let (
            status,
            requested_input,
            resolved_witness,
            resolution_class,
            failure_kind,
            lowered_facts,
        ) = lower_intent_with_catalog(&intent, catalog);
        let kind = match &intent {
            AuthorSpatialAnchorSelectionIntent::Move(_) => SpatialAnchorSelectionKind::Move,
            AuthorSpatialAnchorSelectionIntent::Rotate(_) => SpatialAnchorSelectionKind::Rotate,
            AuthorSpatialAnchorSelectionIntent::Reorient(_) => SpatialAnchorSelectionKind::Reorient,
            AuthorSpatialAnchorSelectionIntent::Offset(_) => SpatialAnchorSelectionKind::Offset,
            AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(_) => {
                SpatialAnchorSelectionKind::LiesOnConstraint
            }
            AuthorSpatialAnchorSelectionIntent::PointsToward(_) => {
                SpatialAnchorSelectionKind::PointsToward
            }
            AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(_) => {
                SpatialAnchorSelectionKind::AnchorMatchConstraint
            }
        };
        let anchor = match &intent {
            AuthorSpatialAnchorSelectionIntent::Move(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Rotate(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Reorient(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Offset(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::PointsToward(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(spec) => spec.anchor(),
        };
        let projection_seed = SpatialAnchorSelectionProjectionSeed::from_parts(
            kind,
            anchor,
            &requested_input,
            status,
            resolved_witness,
            resolution_class,
            failure_kind,
        );
        Self {
            intent,
            status,
            requested_input,
            resolved_witness,
            resolution_class,
            failure_kind,
            lowered_facts,
            projection_seed,
        }
    }

    pub fn kind(&self) -> SpatialAnchorSelectionKind {
        match self.intent {
            AuthorSpatialAnchorSelectionIntent::Move(_) => SpatialAnchorSelectionKind::Move,
            AuthorSpatialAnchorSelectionIntent::Rotate(_) => SpatialAnchorSelectionKind::Rotate,
            AuthorSpatialAnchorSelectionIntent::Reorient(_) => SpatialAnchorSelectionKind::Reorient,
            AuthorSpatialAnchorSelectionIntent::Offset(_) => SpatialAnchorSelectionKind::Offset,
            AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(_) => {
                SpatialAnchorSelectionKind::LiesOnConstraint
            }
            AuthorSpatialAnchorSelectionIntent::PointsToward(_) => {
                SpatialAnchorSelectionKind::PointsToward
            }
            AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(_) => {
                SpatialAnchorSelectionKind::AnchorMatchConstraint
            }
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        match &self.intent {
            AuthorSpatialAnchorSelectionIntent::Move(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Rotate(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Reorient(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::Offset(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::PointsToward(spec) => spec.anchor(),
            AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(spec) => spec.anchor(),
        }
    }

    pub fn requested_input(&self) -> &SpatialAnchorSelectionRequestedInput {
        &self.requested_input
    }

    pub fn status(&self) -> SpatialAnchorSelectionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<SpatialResolvedAnchorWitness> {
        self.resolved_witness
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<SpatialAnchorSelectionFailureKind> {
        self.failure_kind
    }

    pub fn projection_seed(&self) -> &SpatialAnchorSelectionProjectionSeed {
        &self.projection_seed
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub fn apply_to_placement_with_catalog(
        &self,
        placement: SpatialPlacementSpec,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialPlacementSpec, SpatialAnchorSelectionPlacementError> {
        match &self.lowered_facts {
            Some(lowered) => lowered.apply_to_placement(&self.intent, placement, catalog),
            None => Err(admission_failure_for_intent(
                &self.intent,
                self.failure_kind,
            )),
        }
    }
}

#[cfg(any(test, feature = "test-support-lowering"))]
fn admission_failure_for_intent(
    intent: &AuthorSpatialAnchorSelectionIntent,
    failure_kind: Option<SpatialAnchorSelectionFailureKind>,
) -> SpatialAnchorSelectionPlacementError {
    match intent {
        AuthorSpatialAnchorSelectionIntent::Move(_) => {
            SpatialAnchorSelectionPlacementError::MotionAdmission(match failure_kind {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    SpatialMotionError::DestinationWitnessFailure(class)
                }
                _ => SpatialMotionError::DestinationWitnessFailure(
                    SpatialWitnessFailureClass::Unsupported,
                ),
            })
        }
        AuthorSpatialAnchorSelectionIntent::Rotate(_) => {
            SpatialAnchorSelectionPlacementError::MotionAdmission(match failure_kind {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    SpatialMotionError::RotationWitnessFailure(class)
                }
                Some(SpatialAnchorSelectionFailureKind::NonFiniteRotationAngle) => {
                    SpatialMotionError::NonFiniteRotationAngle
                }
                _ => SpatialMotionError::RotationWitnessFailure(
                    SpatialWitnessFailureClass::Unsupported,
                ),
            })
        }
        AuthorSpatialAnchorSelectionIntent::Reorient(_) => {
            SpatialAnchorSelectionPlacementError::MotionAdmission(match failure_kind {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    SpatialMotionError::DirectionWitnessFailure(class)
                }
                _ => SpatialMotionError::DirectionWitnessFailure(
                    SpatialWitnessFailureClass::Unsupported,
                ),
            })
        }
        AuthorSpatialAnchorSelectionIntent::Offset(_) => {
            SpatialAnchorSelectionPlacementError::MotionAdmission(
                SpatialMotionError::NonFiniteOffset,
            )
        }
        AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(_) => {
            SpatialAnchorSelectionPlacementError::ConstraintAdmission(
                SpatialConstraintError::InvalidFrame(
                    crate::witness_resolution::SpatialFrameError::InvalidNormal,
                ),
            )
        }
        AuthorSpatialAnchorSelectionIntent::PointsToward(_) => {
            SpatialAnchorSelectionPlacementError::ConstraintAdmission(match failure_kind {
                Some(SpatialAnchorSelectionFailureKind::Witness(class)) => {
                    SpatialConstraintError::TargetWitnessFailure(class)
                }
                _ => SpatialConstraintError::TargetWitnessFailure(
                    SpatialWitnessFailureClass::Unsupported,
                ),
            })
        }
        AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(_) => {
            SpatialAnchorSelectionPlacementError::ConstraintAdmission(
                SpatialConstraintError::InvalidFrame(
                    crate::witness_resolution::SpatialFrameError::InvalidNormal,
                ),
            )
        }
    }
}

impl ForgeQueryDeclarationInput<SpatialAnchorSelectionQueryDomain>
    for SpatialAnchorSelectionDeclarationEntry
{
    type Family = SpatialAnchorSelectionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.kind",
                format!("{:?}", self.kind()),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.anchor",
                format!("{:?}", self.anchor()),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.requested_input",
                format!("{:?}", self.requested_input()),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.status",
                format!("{:?}", self.status()),
            ),
        ];
        if let Some(value) = self.resolved_witness() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.resolved_witness",
                format!("{value:?}"),
            ));
        }
        if let Some(value) = self.resolution_class() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.resolution_class",
                format!("{value:?}"),
            ));
        }
        if let Some(value) = self.failure_kind() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                "anchor_selection.failure_kind",
                format!("{value:?}"),
            ));
        }
        entries
    }
}
