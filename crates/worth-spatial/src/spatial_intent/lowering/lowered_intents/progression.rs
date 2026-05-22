use forge_proof::{Artifact, DenialTransitionOutcome, TransitionOutcome};

use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::anchors::{
    lower_reorient_anchor, lower_rotation_anchor, lower_subject_anchor,
    lower_translation_anchor_with_catalog, LoweredReorientAnchor,
};
use crate::spatial_intent::lowering::{
    AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialPlacement, AdmittedSpatialReorient,
    AdmittedSpatialRotate, SpatialPlacementSpec,
};
use crate::spatial_intent::refs::SpatialWitnessCatalog;

use super::phases::{AdmittedLoweringIntentPhase, RequestedLoweringIntentPhase};
use super::progression_support::{
    build, coincident, reorient_point_like_posture, reorient_posture, runtime,
};
use super::runtime_declaration::{
    LoweredSpatialIntent, LoweredSpatialIntentArtifact, LoweredSpatialIntentFamily,
    LoweredSpatialNumericPosture, LoweredSpatialOperation, LoweredSpatialTargetBindingPosture,
    RuntimeAnchorSemantic, SpatialLoweringDenial,
};
use super::runtime_payload::LoweredSpatialRuntimePayload;
use super::runtime_targets::{RuntimeDirectionWitnessTarget, RuntimeFrameTarget};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum RequestedLoweringIntent {
    Move(SpatialPlacementSpec, AdmittedSpatialMove),
    Offset(SpatialPlacementSpec, AdmittedSpatialOffset),
    Rotate(SpatialPlacementSpec, AdmittedSpatialRotate),
    Reorient(SpatialPlacementSpec, AdmittedSpatialReorient),
    LiesOn(SpatialPlacementSpec, AdmittedSpatialLiesOnConstraint),
    PointsToward(SpatialPlacementSpec, AdmittedSpatialPointsTowardConstraint),
    AnchorMatch(SpatialPlacementSpec, AdmittedSpatialAnchorMatchConstraint),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum AdmittedLoweringIntent {
    Move(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialMove,
    ),
    Offset(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialOffset,
    ),
    Rotate(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialRotate,
    ),
    Reorient(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialReorient,
    ),
    LiesOn(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialLiesOnConstraint,
    ),
    PointsToward(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialPointsTowardConstraint,
    ),
    AnchorMatch(
        SpatialPlacementSpec,
        AdmittedSpatialPlacement,
        AdmittedSpatialAnchorMatchConstraint,
    ),
}

pub(super) fn request_intent(
    intent: RequestedLoweringIntent,
) -> Artifact<RequestedLoweringIntentPhase, RequestedLoweringIntent> {
    Artifact::new(intent)
}

pub(super) fn admit_requested_intent(
    requested: Artifact<RequestedLoweringIntentPhase, RequestedLoweringIntent>,
) -> DenialTransitionOutcome<
    Artifact<AdmittedLoweringIntentPhase, AdmittedLoweringIntent>,
    SpatialLoweringDenial,
> {
    let admitted = match requested.payload() {
        RequestedLoweringIntent::Move(placement, motion) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::Move(placement.clone(), admitted, motion.clone())
                },
            )
        }
        RequestedLoweringIntent::Offset(placement, motion) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::Offset(placement.clone(), admitted, motion.clone())
                },
            )
        }
        RequestedLoweringIntent::Rotate(placement, motion) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::Rotate(placement.clone(), admitted, motion.clone())
                },
            )
        }
        RequestedLoweringIntent::Reorient(placement, motion) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::Reorient(placement.clone(), admitted, motion.clone())
                },
            )
        }
        RequestedLoweringIntent::LiesOn(placement, constraint) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::LiesOn(placement.clone(), admitted, constraint.clone())
                },
            )
        }
        RequestedLoweringIntent::PointsToward(placement, constraint) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::PointsToward(
                        placement.clone(),
                        admitted,
                        constraint.clone(),
                    )
                },
            )
        }
        RequestedLoweringIntent::AnchorMatch(placement, constraint) => {
            crate::spatial_intent::lowering::admit_spatial_placement(placement.clone()).map(
                |admitted| {
                    AdmittedLoweringIntent::AnchorMatch(
                        placement.clone(),
                        admitted,
                        constraint.clone(),
                    )
                },
            )
        }
    };
    match admitted {
        Ok(value) => TransitionOutcome::success(Artifact::new(value)),
        Err(_) => TransitionOutcome::denied(SpatialLoweringDenial::InvalidExistingPlacement),
    }
}

pub(super) fn lower_admitted_intent<C: SpatialWitnessCatalog>(
    admitted: Artifact<AdmittedLoweringIntentPhase, AdmittedLoweringIntent>,
    catalog: &C,
) -> DenialTransitionOutcome<LoweredSpatialIntentArtifact, SpatialLoweringDenial> {
    let lowered = (|| -> Result<LoweredSpatialIntent, SpatialLoweringDenial> {
        match admitted.payload() {
            AdmittedLoweringIntent::Move(placement, _, motion) => {
                let anchor = lower_translation_anchor_with_catalog(
                    placement,
                    motion.spec().anchor(),
                    catalog,
                )?;
                Ok(build(
                    runtime(
                        LoweredSpatialIntentFamily::Move,
                        Some(anchor.origin().into()),
                        None,
                        LoweredSpatialNumericPosture::Direct,
                        LoweredSpatialTargetBindingPosture::PointWitness,
                        LoweredSpatialRuntimePayload::Move {
                            anchor_world_point: anchor.world_point(),
                            target_world_point: motion.destination_point(),
                        },
                    ),
                    LoweredSpatialOperation::Move {
                        anchor_world_point: anchor.world_point(),
                        target_world_point: motion.destination_point(),
                    },
                ))
            }
            AdmittedLoweringIntent::Offset(placement, _, motion) => {
                let anchor = lower_translation_anchor_with_catalog(
                    placement,
                    motion.spec().anchor(),
                    catalog,
                )?;
                Ok(build(
                    runtime(
                        LoweredSpatialIntentFamily::Offset,
                        Some(anchor.origin().into()),
                        None,
                        LoweredSpatialNumericPosture::Direct,
                        LoweredSpatialTargetBindingPosture::PointWitness,
                        LoweredSpatialRuntimePayload::Offset {
                            offset: motion.spec().offset(),
                        },
                    ),
                    LoweredSpatialOperation::Offset {
                        offset: motion.spec().offset(),
                    },
                ))
            }
            AdmittedLoweringIntent::Rotate(placement, admitted, motion) => {
                match motion.spec().anchor() {
                    crate::spatial_intent::refs::SpatialAnchorRef::ShapeOrigin => Ok(build(
                        runtime(
                            LoweredSpatialIntentFamily::Rotate,
                            Some(RuntimeAnchorSemantic::ShapeOriginPoint),
                            None,
                            LoweredSpatialNumericPosture::Normalized,
                            LoweredSpatialTargetBindingPosture::DirectionWitness,
                            LoweredSpatialRuntimePayload::RotateFacingOnly {
                                source_facing: admitted.facing_vector(),
                                axis: motion.normalized_axis(),
                                angle_radians: motion.spec().angle_radians(),
                            },
                        ),
                        LoweredSpatialOperation::RotateFacingOnly {
                            source_facing: admitted.facing_vector(),
                            axis: motion.normalized_axis(),
                            angle_radians: motion.spec().angle_radians(),
                        },
                    )),
                    _ => {
                        let pivot =
                            lower_rotation_anchor(placement, motion.spec().anchor(), catalog)?;
                        Ok(build(
                            runtime(
                                LoweredSpatialIntentFamily::Rotate,
                                Some(pivot.origin().into()),
                                None,
                                LoweredSpatialNumericPosture::Normalized,
                                LoweredSpatialTargetBindingPosture::DirectionWitness,
                                LoweredSpatialRuntimePayload::RotateAroundPivot {
                                    source_origin: placement.origin(),
                                    source_facing: admitted.facing_vector(),
                                    pivot_world_point: pivot.world_point(),
                                    axis: motion.normalized_axis(),
                                    angle_radians: motion.spec().angle_radians(),
                                },
                            ),
                            LoweredSpatialOperation::RotateAroundPivot {
                                source_facing: admitted.facing_vector(),
                                pivot_world_point: pivot.world_point(),
                                axis: motion.normalized_axis(),
                                angle_radians: motion.spec().angle_radians(),
                            },
                        ))
                    }
                }
            }
            AdmittedLoweringIntent::Reorient(placement, admitted, motion) => {
                match lower_reorient_anchor(placement, motion.spec().anchor(), catalog)? {
                    LoweredReorientAnchor::PointLike(anchor) => Ok(build(
                        runtime(
                            LoweredSpatialIntentFamily::Reorient,
                            Some(anchor.origin().into()),
                            None,
                            reorient_point_like_posture(
                                admitted.facing_vector(),
                                motion.spec().direction_witness(),
                            ),
                            LoweredSpatialTargetBindingPosture::DirectionWitness,
                            LoweredSpatialRuntimePayload::ReorientPointLike {
                                source_anchor_world_point: anchor.world_point(),
                                target_direction: RuntimeDirectionWitnessTarget::from_witness(
                                    motion.spec().direction_witness(),
                                ),
                            },
                        ),
                        LoweredSpatialOperation::ReorientPointLike {
                            source_anchor_world_point: anchor.world_point(),
                            target_direction: RuntimeDirectionWitnessTarget::from_witness(
                                motion.spec().direction_witness(),
                            ),
                        },
                    )),
                    LoweredReorientAnchor::Directional(anchor) => Ok(build(
                        runtime(
                            LoweredSpatialIntentFamily::Reorient,
                            Some(anchor.origin().into()),
                            None,
                            reorient_posture(anchor.world_direction(), motion.normalized_facing()),
                            LoweredSpatialTargetBindingPosture::DirectionWitness,
                            LoweredSpatialRuntimePayload::ReorientDirectional {
                                source_world_direction: anchor.world_direction(),
                                target_world_direction: motion.normalized_facing(),
                            },
                        ),
                        LoweredSpatialOperation::ReorientDirectional {
                            source_world_direction: anchor.world_direction(),
                            target_world_direction: motion.normalized_facing(),
                        },
                    )),
                }
            }
            AdmittedLoweringIntent::LiesOn(placement, _, constraint) => {
                match constraint.spec().anchor() {
                    crate::spatial_intent::refs::SpatialAnchorRef::ShapeOrigin => Ok(build(
                        runtime(
                            LoweredSpatialIntentFamily::LiesOn,
                            Some(RuntimeAnchorSemantic::ShapeOriginPoint),
                            None,
                            LoweredSpatialNumericPosture::Direct,
                            LoweredSpatialTargetBindingPosture::FrameTarget,
                            LoweredSpatialRuntimePayload::LiesOnShapeOrigin {
                                target_frame: RuntimeFrameTarget::from_admitted(constraint.frame()),
                            },
                        ),
                        LoweredSpatialOperation::LiesOnShapeOrigin {
                            target_frame: constraint.frame().clone(),
                        },
                    )),
                    _ => {
                        let anchor =
                            lower_subject_anchor(placement, constraint.spec().anchor(), catalog)?
                                .into_point();
                        Ok(build(
                            runtime(
                                LoweredSpatialIntentFamily::LiesOn,
                                Some(anchor.origin().into()),
                                None,
                                LoweredSpatialNumericPosture::Direct,
                                LoweredSpatialTargetBindingPosture::FrameTarget,
                                LoweredSpatialRuntimePayload::LiesOnProjected {
                                    target_frame: RuntimeFrameTarget::from_admitted(
                                        constraint.frame(),
                                    ),
                                    anchor_world_point: anchor.world_point(),
                                },
                            ),
                            LoweredSpatialOperation::LiesOnProjected {
                                target_frame: constraint.frame().clone(),
                                anchor_world_point: anchor.world_point(),
                            },
                        ))
                    }
                }
            }
            AdmittedLoweringIntent::PointsToward(placement, _, constraint) => {
                let anchor = lower_translation_anchor_with_catalog(
                    placement,
                    constraint.spec().anchor(),
                    catalog,
                )?;
                if coincident(anchor.world_point(), constraint.target_point()) {
                    Err(SpatialLoweringDenial::Coincident)
                } else {
                    Ok(build(
                        runtime(
                            LoweredSpatialIntentFamily::PointsToward,
                            Some(anchor.origin().into()),
                            None,
                            LoweredSpatialNumericPosture::Direct,
                            LoweredSpatialTargetBindingPosture::PointWitness,
                            LoweredSpatialRuntimePayload::PointsToward {
                                anchor_world_point: anchor.world_point(),
                                target_world_point: constraint.target_point(),
                            },
                        ),
                        LoweredSpatialOperation::PointsToward {
                            anchor_world_point: anchor.world_point(),
                            target_world_point: constraint.target_point(),
                        },
                    ))
                }
            }
            AdmittedLoweringIntent::AnchorMatch(placement, _, constraint) => {
                let anchor = lower_subject_anchor(placement, constraint.spec().anchor(), catalog)?
                    .into_point();
                let target = lower_translation_anchor_with_catalog(
                    placement,
                    constraint.spec().other_anchor(),
                    catalog,
                )?;
                Ok(build(
                    runtime(
                        LoweredSpatialIntentFamily::AnchorMatch,
                        Some(anchor.origin().into()),
                        Some(target.origin().into()),
                        LoweredSpatialNumericPosture::Direct,
                        LoweredSpatialTargetBindingPosture::AnchorTarget,
                        LoweredSpatialRuntimePayload::AnchorMatch {
                            anchor_world_point: anchor.world_point(),
                            target_world_point: target.world_point(),
                        },
                    ),
                    LoweredSpatialOperation::AnchorMatch {
                        anchor_world_point: anchor.world_point(),
                        target_world_point: target.world_point(),
                    },
                ))
            }
        }
    })();
    match lowered {
        Ok(value) => TransitionOutcome::success(Artifact::new(value)),
        Err(denial) => TransitionOutcome::denied(denial),
    }
}
