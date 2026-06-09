#![cfg_attr(not(any(test, feature = "test-support-lowering")), allow(dead_code))]

use super::query_native_authoring::{
    AuthorSpatialAnchorSelectionIntent, SpatialAnchorSelectionFailureKind,
    SpatialAnchorSelectionPlacementError, SpatialAnchorSelectionRequestedInput,
    SpatialAnchorSelectionStatus, SpatialResolvedAnchorWitness,
};
use crate::facade::refs::SpatialWitnessCatalog;
use crate::placement::SpatialPlacementSpec;
use crate::placement::{
    admit_spatial_placement, apply_anchor_match_constraint_to_placement_with_catalog,
    apply_lies_on_constraint_to_placement_with_catalog, apply_move_to_placement_with_catalog,
    apply_offset_to_placement_with_catalog,
    apply_points_toward_constraint_to_placement_with_catalog,
    apply_reorient_to_placement_with_catalog, apply_rotate_to_placement_with_catalog,
};
use crate::witness_resolution::witness_resolution::{
    resolve_spatial_direction_witness_with_catalog, resolve_spatial_point_witness_with_catalog,
};
use crate::witness_resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameError, SpatialWitnessResolutionClass,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SpatialAnchorSelectionLoweredFacts {
    Move { destination_point: [f64; 3] },
    Rotate { normalized_axis: [f64; 3] },
    Reorient { normalized_facing: [f64; 3] },
    Offset,
    LiesOnConstraint { frame: AdmittedSpatialFrameRef },
    PointsToward { target_point: [f64; 3] },
    AnchorMatchConstraint,
}

pub(crate) fn lower_intent_with_catalog(
    intent: &AuthorSpatialAnchorSelectionIntent,
    catalog: &impl SpatialWitnessCatalog,
) -> (
    SpatialAnchorSelectionStatus,
    SpatialAnchorSelectionRequestedInput,
    Option<SpatialResolvedAnchorWitness>,
    Option<SpatialWitnessResolutionClass>,
    Option<SpatialAnchorSelectionFailureKind>,
    Option<SpatialAnchorSelectionLoweredFacts>,
) {
    match intent {
        AuthorSpatialAnchorSelectionIntent::Move(spec) => {
            let requested = SpatialAnchorSelectionRequestedInput::PointWitness(
                spec.destination_witness().clone(),
            );
            match resolve_spatial_point_witness_with_catalog(
                spec.destination_witness().clone(),
                catalog,
            ) {
                Ok(resolved) => (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    Some(SpatialResolvedAnchorWitness::Point(
                        resolved.resolved_world_point(),
                    )),
                    Some(resolved.resolution_class()),
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::Move {
                        destination_point: resolved.resolved_world_point(),
                    }),
                ),
                Err(class) => {
                    rejected(requested, SpatialAnchorSelectionFailureKind::Witness(class))
                }
            }
        }
        AuthorSpatialAnchorSelectionIntent::Rotate(spec) => {
            let requested =
                SpatialAnchorSelectionRequestedInput::DirectionWitness(spec.axis_witness().clone());
            if !spec.angle_radians().is_finite() {
                return rejected(
                    requested,
                    SpatialAnchorSelectionFailureKind::NonFiniteRotationAngle,
                );
            }
            match resolve_spatial_direction_witness_with_catalog(
                spec.axis_witness().clone(),
                catalog,
            ) {
                Ok(resolved) => (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    Some(SpatialResolvedAnchorWitness::Direction(
                        resolved.resolved_world_direction(),
                    )),
                    Some(resolved.resolution_class()),
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::Rotate {
                        normalized_axis: resolved.resolved_world_direction(),
                    }),
                ),
                Err(class) => {
                    rejected(requested, SpatialAnchorSelectionFailureKind::Witness(class))
                }
            }
        }
        AuthorSpatialAnchorSelectionIntent::Reorient(spec) => {
            let requested = SpatialAnchorSelectionRequestedInput::DirectionWitness(
                spec.direction_witness().clone(),
            );
            match resolve_spatial_direction_witness_with_catalog(
                spec.direction_witness().clone(),
                catalog,
            ) {
                Ok(resolved) => (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    Some(SpatialResolvedAnchorWitness::Direction(
                        resolved.resolved_world_direction(),
                    )),
                    Some(resolved.resolution_class()),
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::Reorient {
                        normalized_facing: resolved.resolved_world_direction(),
                    }),
                ),
                Err(class) => {
                    rejected(requested, SpatialAnchorSelectionFailureKind::Witness(class))
                }
            }
        }
        AuthorSpatialAnchorSelectionIntent::Offset(spec) => {
            let requested = SpatialAnchorSelectionRequestedInput::Offset(spec.offset());
            if spec.offset().iter().any(|value| !value.is_finite()) {
                rejected(
                    requested,
                    SpatialAnchorSelectionFailureKind::NonFiniteOffset,
                )
            } else {
                (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    None,
                    None,
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::Offset),
                )
            }
        }
        AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(spec) => {
            let requested = SpatialAnchorSelectionRequestedInput::Frame(spec.frame().clone());
            match admit_spatial_frame(spec.frame().clone()) {
                Ok(frame) => (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    None,
                    None,
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::LiesOnConstraint { frame }),
                ),
                Err(_) => rejected(requested, SpatialAnchorSelectionFailureKind::InvalidFrame),
            }
        }
        AuthorSpatialAnchorSelectionIntent::PointsToward(spec) => {
            let requested =
                SpatialAnchorSelectionRequestedInput::PointWitness(spec.target_witness().clone());
            match resolve_spatial_point_witness_with_catalog(spec.target_witness().clone(), catalog)
            {
                Ok(resolved) => (
                    SpatialAnchorSelectionStatus::Admitted,
                    requested,
                    Some(SpatialResolvedAnchorWitness::Point(
                        resolved.resolved_world_point(),
                    )),
                    Some(resolved.resolution_class()),
                    None,
                    Some(SpatialAnchorSelectionLoweredFacts::PointsToward {
                        target_point: resolved.resolved_world_point(),
                    }),
                ),
                Err(class) => {
                    rejected(requested, SpatialAnchorSelectionFailureKind::Witness(class))
                }
            }
        }
        AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(spec) => (
            SpatialAnchorSelectionStatus::Admitted,
            SpatialAnchorSelectionRequestedInput::OtherAnchor(spec.other_anchor().clone()),
            None,
            None,
            None,
            Some(SpatialAnchorSelectionLoweredFacts::AnchorMatchConstraint),
        ),
    }
}

impl SpatialAnchorSelectionLoweredFacts {
    pub(crate) fn apply_to_placement(
        &self,
        intent: &AuthorSpatialAnchorSelectionIntent,
        placement: SpatialPlacementSpec,
        catalog: &impl SpatialWitnessCatalog,
    ) -> Result<SpatialPlacementSpec, SpatialAnchorSelectionPlacementError> {
        match (intent, self) {
            (AuthorSpatialAnchorSelectionIntent::Move(spec), Self::Move { destination_point }) => {
                validate_motion_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)?;
                apply_move_to_placement_with_catalog(placement, spec, *destination_point, catalog)
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)
            }
            (
                AuthorSpatialAnchorSelectionIntent::Rotate(spec),
                Self::Rotate { normalized_axis },
            ) => {
                let existing = validate_motion_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)?;
                apply_rotate_to_placement_with_catalog(
                    existing.facing_vector(),
                    placement,
                    spec,
                    *normalized_axis,
                    catalog,
                )
                .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)
            }
            (
                AuthorSpatialAnchorSelectionIntent::Reorient(spec),
                Self::Reorient { normalized_facing },
            ) => {
                let existing = validate_motion_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)?;
                apply_reorient_to_placement_with_catalog(
                    existing.frame(),
                    existing.facing_vector(),
                    placement,
                    spec,
                    spec.direction_witness(),
                    *normalized_facing,
                    catalog,
                )
                .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)
            }
            (AuthorSpatialAnchorSelectionIntent::Offset(spec), Self::Offset) => {
                validate_motion_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)?;
                apply_offset_to_placement_with_catalog(placement, spec, catalog)
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementMotion)
            }
            (
                AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(spec),
                Self::LiesOnConstraint { frame },
            ) => {
                validate_constraint_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)?;
                apply_lies_on_constraint_to_placement_with_catalog(placement, spec, frame, catalog)
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)
            }
            (
                AuthorSpatialAnchorSelectionIntent::PointsToward(spec),
                Self::PointsToward { target_point },
            ) => {
                validate_constraint_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)?;
                apply_points_toward_constraint_to_placement_with_catalog(
                    placement,
                    spec,
                    *target_point,
                    catalog,
                )
                .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)
            }
            (
                AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(spec),
                Self::AnchorMatchConstraint,
            ) => {
                validate_constraint_placement(placement.clone())
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)?;
                apply_anchor_match_constraint_to_placement_with_catalog(placement, spec, catalog)
                    .map_err(SpatialAnchorSelectionPlacementError::PlacementConstraint)
            }
            (AuthorSpatialAnchorSelectionIntent::Move(_), _)
            | (AuthorSpatialAnchorSelectionIntent::Rotate(_), _)
            | (AuthorSpatialAnchorSelectionIntent::Reorient(_), _)
            | (AuthorSpatialAnchorSelectionIntent::Offset(_), _)
            | (AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(_), _)
            | (AuthorSpatialAnchorSelectionIntent::PointsToward(_), _)
            | (AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(_), _) => {
                unreachable!("lowered anchor-selection facts must match declaration intent")
            }
        }
    }
}

fn rejected(
    requested: SpatialAnchorSelectionRequestedInput,
    failure_kind: SpatialAnchorSelectionFailureKind,
) -> (
    SpatialAnchorSelectionStatus,
    SpatialAnchorSelectionRequestedInput,
    Option<SpatialResolvedAnchorWitness>,
    Option<SpatialWitnessResolutionClass>,
    Option<SpatialAnchorSelectionFailureKind>,
    Option<SpatialAnchorSelectionLoweredFacts>,
) {
    (
        SpatialAnchorSelectionStatus::Rejected,
        requested,
        None,
        None,
        Some(failure_kind),
        None,
    )
}

fn validate_motion_placement(
    placement: SpatialPlacementSpec,
) -> Result<crate::placement::AdmittedSpatialPlacement, crate::placement::SpatialPlacementMotionError>
{
    admit_spatial_placement(placement)
        .map_err(|_| crate::placement::SpatialPlacementMotionError::InvalidExistingPlacement)
}

fn validate_constraint_placement(
    placement: SpatialPlacementSpec,
) -> Result<(), crate::placement::SpatialPlacementConstraintError> {
    admit_spatial_placement(placement)
        .map(|_| ())
        .map_err(map_existing_placement_constraint_failure)
}

fn map_existing_placement_constraint_failure(
    error: crate::placement::SpatialPlacementError,
) -> crate::placement::SpatialPlacementConstraintError {
    match error {
        crate::placement::SpatialPlacementError::InvalidReferenceFrame(error) => {
            crate::placement::SpatialPlacementConstraintError::InvalidReferenceFrame(error)
        }
        _ => crate::placement::SpatialPlacementConstraintError::InvalidReferenceFrame(
            SpatialFrameError::InvalidNormal,
        ),
    }
}
